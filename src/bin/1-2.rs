use anyhow::Result;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Elf {
    snaks: u32,
}

impl Elf {
    fn new() -> Self {
        Self { snaks: 0 }
    }
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/1.txt")?;
    let mut each_elfs = contents
        .split('\n')
        .map(str::trim)
        .fold(vec![Elf::new()], |mut a, v| {
            if v.is_empty() {
                a.push(Elf::new());
                a
            } else {
                a.last_mut().unwrap().snaks += v.parse::<u32>().unwrap();
                a
            }
        });

    each_elfs.sort();
    each_elfs.reverse();
    let top3_total: u32 = each_elfs.iter().take(3).map(|e| e.snaks).sum();
    println!("total = {top3_total}");
    Ok(())
}
