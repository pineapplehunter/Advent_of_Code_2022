use anyhow::Result;
use std::fs::read_to_string;

struct Section(u8, u8);

impl Section {
    fn overlaps(&self, other: &Self) -> bool {
        !(self.1 < other.0 || other.1 < self.0)
    }
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/4.txt")?;

    let result: u32 = contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once(','))
        .map(|(a, b)| [a, b])
        .map(|elfs| elfs.map(|e| e.split_once('-').unwrap()))
        .map(|elfs| elfs.map(|a| Section(a.0.parse().unwrap(), a.1.parse().unwrap())))
        .map(|elfs| elfs[0].overlaps(&elfs[1]))
        .map(|b| b as u32)
        .sum();

    println!("result = {result}");

    Ok(())
}
