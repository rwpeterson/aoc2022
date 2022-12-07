use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use anyhow::{Context, Result};
#[allow(unused_imports)]
use either::{Either, Left, Right};
use itertools::Itertools;

pub fn main(input: &str) -> Result<Either<String, (String, String)>> {
    let x = input
        .chars()
        .enumerate()
        .tuple_windows()
        .filter_map(|(
            (_, a),
            (_, b),
            (_, c),
            (i, d)
        )| {
            let hs = HashSet::from([a, b, c, d]);
            match hs.len() {
                4 => Some(i + 1),
                _ => None,
            }
        })
        .next().context("no marker received")?;

    // Part 2: tuple_windows goes up to 12, it's like they knew :)
    // we'll have to do it more explicitly
    let mut d = VecDeque::new();
    let mut j = None;
    for (i, c) in input.chars().enumerate() {
        d.push_back(c);
        if d.len() < 13 {
            continue
        }
        if d.len() > 14 {
            d.pop_front();
        }
        let hs = d.iter().collect::<HashSet<_>>();
        if hs.len() == 14 {
            j = Some(i + 1);
            break;
        }
    }

    let s = x.to_string();
    let t = j.expect("no message header").to_string();
    Ok(Right((s,t)))
}
