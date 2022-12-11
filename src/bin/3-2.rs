use anyhow::Result;
use itertools::Itertools;
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
            if v1 == v2 && !output.iter().any(|a| a == v1) {
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
        .chunks(3)
        .into_iter()
        .map(|a| {
            a.into_iter()
                .map(|s| s.bytes().map(Item).collect::<Vec<Item>>())
                .collect::<Vec<Vec<Item>>>()
                .try_into()
                .unwrap()
        })
        .map(|a: [Vec<Item>; 3]| intersection(&intersection(&a[0], &a[1]), &a[2]))
        .map(|a| {
            assert_eq!(a.len(), 1);
            a[0]
        })
        .map(|p| Item::priority(&p))
        .sum();

    println!("result = {result}");

    Ok(())
}
