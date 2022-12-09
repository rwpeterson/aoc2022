#[allow(unused_imports)]
use anyhow::{Result, Context};
#[allow(unused_imports)]
use either::{Either, Left, Right};
use nom::{
    IResult,
    sequence::separated_pair,
    bytes::complete::tag,
    combinator::{map, map_res},
    character::complete::digit1,
};

#[derive(Debug, PartialEq, Eq)]
struct Range(
    u32,
    u32,
);

#[derive(Debug, PartialEq, Eq)]
struct Assignment {
    r: Range,
    s: Range,
}

impl Assignment {
    pub fn is_subset(self) -> bool {
        (self.r.0 <= self.s.0 && self.s.1 <= self.r.1) ||
        (self.s.0 <= self.r.0 && self.r.1 <= self.s.1)
    }

    pub fn intersect(self) -> bool {
        (self.s.0 <= self.r.1 && self.r.0 <= self.s.1) ||
        (self.r.0 <= self.s.1 && self.s.0 <= self.r.1)
    }
}

fn range(input: &str) -> IResult<&str, Range> {
    map(
        separated_pair(
            map_res(digit1, move |d| u32::from_str_radix(d, 10)),
            tag("-"),
            map_res(digit1, move |d| u32::from_str_radix(d, 10)),
        ),
        |(a, b)| Range(a, b)
    )(input)
}

fn assign(input: &str) -> IResult<&str, Assignment> {
    map(
        separated_pair(
            range,
            tag(","),
            range,
        ),
        |(r, s)| Assignment { r, s }
    )(input)
}

pub fn main(input: &str) -> Result<Either<String, (String, String)>> {
    let strict_subsets = input
        .lines()
        .filter(|l| {
            let (_, a) = assign(l).expect("not a pair assignment");
            a.is_subset()
        })
        .count();

    let intersections = input
        .lines()
        .filter(|l| {
            let (_, a) = assign(l).expect("not a pair assignment");
            a.intersect()
        })
        .count();

    Ok(Right((
        strict_subsets.to_string(),
        intersections.to_string(),
    )))
}
