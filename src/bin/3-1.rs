use anyhow::Result;
use std::{fmt, fs::read_to_string};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Item(u8);

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Item").field(&(self.0 as char)).finish()
    }
}

impl Item {
    fn priority(&self) -> u32 {
        (match self.0 {
            b'a'..=b'z' => self.0 - b'a' + 1,
            b'A'..=b'Z' => self.0 - b'A' + 27,
            _ => unimplemented!(),
        })
        .into()
    }
}

fn intersection(c1: &Vec<Item>, c2: &Vec<Item>) -> Vec<Item> {
    let mut output = Vec::new();
    for v1 in c1 {
        for v2 in c2 {
            if v1 == v2 && output.iter().all(|a| a != v1) {
                output.push(*v1);
            }
        }
    }
    output
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/3.txt")?;

    let result: u32 = contents
        .split('\n')
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .map(|a| a.split_at(a.len() / 2))
        .map(|(a, b)| (a.bytes().map(Item).collect(), b.bytes().map(Item).collect()))
        .map(|(a, b): (Vec<Item>, Vec<Item>)| intersection(&a, &b))
        .map(|a| dbg!(a).iter().map(Item::priority).sum::<u32>())
        .sum();

    println!("result = {result}");

    Ok(())
}
