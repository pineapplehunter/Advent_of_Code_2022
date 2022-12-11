use anyhow::Result;
use std::fs::read_to_string;

#[derive(Debug)]
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
    let each_elfs = contents
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

    let max = each_elfs.iter().map(|e| e.snaks).max().unwrap();
    println!("max = {max}");
    Ok(())
}
