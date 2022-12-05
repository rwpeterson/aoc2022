use anyhow::Result;
#[allow(unused_imports)]
use either::{Either, Left, Right};
use itertools::Itertools;
use std::collections::HashSet;

pub fn main(input: &str) -> Result<Either<String, (String, String)>> {
    let prio_sum: u32 = input
        .lines()
        .map(|l| {
            let n = l.len();
            let mut a = HashSet::new();
            let mut b = HashSet::new();
            for (i, c) in l.chars().enumerate() {
                if i < n / 2 {
                    a.insert(c);
                } else {
                    b.insert(c);
                }
            }
            item_priority(*a.intersection(&b).next().expect("No overlap between rucksack compartments"))
        })
        .sum();

    let s = prio_sum.to_string();

    let badges = input
        .lines()
        .chunks(3)
        .into_iter()
        .filter_map(|group| {
            let mut inventories = vec![HashSet::new(); 3];
            for (i, rucksack) in group.enumerate() {
                for item in rucksack.chars() {
                    inventories[i].insert(item);
                }
            }
            let mut badge_set = inventories[0].clone();
            for inventory in inventories.iter().skip(1) {
                badge_set = badge_set.intersection(inventory).cloned().collect();
            }
            let badge = badge_set.into_iter().next();
            badge

        })
        .collect::<Vec<_>>();

    let badge_sum: u32 = badges
        .iter()
        .map(|b| {
            item_priority(*b)
        })
        .sum();

    let t = badge_sum.to_string();

    Ok(Right((s, t)))
}

fn item_priority(c: char) -> u32 {
    let ascii = c as u32;
    if c.is_uppercase() {
        return ascii - 65 + 27
    } else {
        return ascii - 97 + 1
    }
}
