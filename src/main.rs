use anyhow::{Result, Context};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let f = File::open("p1.txt")?;
    let lines = io::BufReader::new(f).lines();
    let mut v: Vec<u64> = Vec::new();
    for rline in lines {
        let line = rline?;
        if !(line.is_empty()) {
            let cals = str::parse::<u64>(&line)?;
            if let Some(x) = v.last_mut() {
                *x += cals
            } else {
                v.push(cals);
            }
        } else {
            v.push(0);
        }
    }
    println!("Most calories held: {}", v.iter().max().context("nobody is carrying any food")?);
    let mut sum = 0;
    let number = 3;
    for _ in 0..number {
        let maxcals = v.iter().max().context("nobody is carrying any food")?;
        let pos = v.iter().position(|x| x == maxcals).context("max needs to be present")?;
        sum += v.swap_remove(pos);
    }
    println!("Sum of top {} calories held: {}", number, sum);
    Ok(())
}
