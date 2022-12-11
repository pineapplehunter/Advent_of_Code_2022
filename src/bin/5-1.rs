use std::{fmt, fs::read_to_string};

use anyhow::Result;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::*;

fn main() -> Result<()> {
    let src = read_to_string("inputs/5.txt")?;
    let input = parse_all().parse(src.clone());
    let input = match input {
        Ok(input) => input,
        Err(e) => {
            report(e, src);
            panic!("parse end");
        }
    };

    let mut boat = input.boat;

    for m in input.movements {
        for _ in 0..m.count {
            let p = boat.0[m.from - 1].pop().unwrap();
            boat.0[m.to - 1].push(p)
        }
    }

    println!("{boat}");

    Ok(())
}


#[test]
fn boat_parse_test() {
    let p = parse_boat().then_ignore(end());
    let out = p.parse(
        r#"            [C]         [N] [R]
    [J] [T]     [H]         [P] [L]
    [F] [S] [T] [B]         [M] [D]
    [C] [L] [J] [Z] [S]     [L] [B]
    [N] [Q] [G] [J] [J]     [F] [F] [R]
    [D] [V] [B] [L] [B] [Q] [D] [M] [T]
    [B] [Z] [Z] [T] [V] [S] [V] [S] [D]
    [W] [P] [P] [D] [G] [P] [B] [P] [V]"#,
    );

    assert_eq!(
        format!("{}", out.unwrap()),
        r#"Boat
1:
2: W B D N C F J
3: P Z V Q L S T
4: P Z B G J T C
5: D T L J Z B H
6: G V B J S
7: P S Q N
8: B V D F L M P R
9: P S M F B D L
10: V D T R
"#
        .to_string()
    );
}

#[test]
fn movement_parse_test() {
    let out = parse_movement().parse(r#"move 4 from 9 to 6"#);

    assert_eq!(
        out,
        Ok(Movement {
            count: 4,
            from: 9,
            to: 6
        })
    )
}

#[derive(Debug, Clone, PartialEq)]
struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Boat(Vec<Vec<char>>);

impl fmt::Display for Boat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Boat")?;
        for (i, c) in self.0.iter().enumerate() {
            write!(f, "{}:", i + 1)?;
            for c in c {
                write!(f, " {c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Input {
    boat: Boat,
    movements: Vec<Movement>,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.boat)?;
        for m in self.movements.iter() {
            writeln!(f, "{m:?}")?;
        }
        Ok(())
    }
}

fn parse_boat() -> impl Parser<char, Boat, Error = Simple<char>> {
    let boat_crate = just('[').ignore_then(any()).then_ignore(just(']'));
    let boat_block = boat_crate
        .map(Some)
        .or(just("   ").map(|_| None));

    let boat_line = boat_block.separated_by(just(' '));
    let boat = boat_line.separated_by(just('\n'));
    boat.labelled("boat").map(|i| {
        let mut output = Vec::new();
        for row in i.iter().rev() {
            for (idx, line) in row.iter().enumerate() {
                while output.len() < idx + 1 {
                    output.push(Vec::new())
                }
                if let Some(v) = line {
                    output[idx].push(*v);
                }
            }
        }
        Boat(output)
    })
}

fn parse_movement() -> impl Parser<char, Movement, Error = Simple<char>> {
    let int = text::int(10).try_map(|s: String, span| {
        s.parse::<usize>()
            .map_err(|e| Simple::custom(span, format!("{e}")))
    });
    just("move ")
        .ignore_then(int)
        .then_ignore(just(" from "))
        .then(int)
        .then_ignore(just(" to "))
        .then(int)
        .map(|((m, f), t)| Movement {
            count: m,
            from: f,
            to: t,
        })
        .labelled("movement")
}

fn parse_all() -> impl Parser<char, Input, Error = Simple<char>> {
    let boat = parse_boat();
    let dummy_line = (just(" ").then(any()).then(just(" "))).separated_by(just(' '));
    let movements = parse_movement()
        .separated_by(just('\n'))
        .labelled("movements");
    boat.then_ignore(dummy_line.labelled("dummy line"))
        .then_ignore(just("\n\n"))
        .then(movements)
        .then_ignore(end())
        .map(|(boat, movements)| Input { boat, movements })
}

fn report(errors: Vec<Simple<char>>, src: String) {
    errors
        .into_iter()
        .map(|e| e.map(|c| c.to_string()))
        .for_each(|e| {
            let report = Report::build(ReportKind::Error, (), e.span().start);

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            ))
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Must be closed before this {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Unexpected => report
                    .with_message(format!(
                        "{}, expected {}",
                        if e.found().is_some() {
                            "Unexpected token in input"
                        } else {
                            "Unexpected end of input"
                        },
                        if e.expected().len() == 0 {
                            "something else".to_string()
                        } else {
                            e.expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ))
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Unexpected token {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                    Label::new(e.span())
                        .with_message(format!("{}", msg.fg(Color::Red)))
                        .with_color(Color::Red),
                ),
            };

            report.finish().print(Source::from(&src)).unwrap();
        });
}
