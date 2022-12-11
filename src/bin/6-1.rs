use std::fs::read_to_string;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let contents = read_to_string("inputs/6.txt")?;
    let pos = contents.bytes().tuple_windows().position(|(a,b,c,d)|{
        let a = [a,b,c,d];
        let mut flag = false;
        for i in 0..4{
            for j in (i+1)..4 {
                if a[i] == a[j]{
                    flag = true;
                }
            }
        }
        !flag
    }).unwrap() + 1;

    println!("{pos:?}");

    Ok(())
}