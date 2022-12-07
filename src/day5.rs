#[allow(unused_imports)]
use anyhow::{Context, Result};
#[allow(unused_imports)]
use either::{Either, Left, Right};
use nom::{
    IResult,
    branch::alt,
    sequence::{delimited, tuple, terminated, separated_pair, preceded},
    bytes::complete::tag,
    combinator::map,
    character::complete::{alpha1, digit1, line_ending},
    multi::{many1, separated_list1},
};
use std::collections::VecDeque;

fn stack_row(input: &str) -> IResult<&str, Vec<Option<u32>>> {
    terminated(
        separated_list1(
            tag(" "),
            map(
                alt((
                    tag("   "),
                    delimited(tag("["), alpha1, tag("]")),
                )),
                |s: &str| s.trim().chars().next().and_then(|c| Some(c as u32))
            ),
        ),
        line_ending,
    )(input)
}

fn stack_indices(input: &str) -> IResult<&str, Vec<&str>> {
    terminated(
        separated_list1(
            tag(" "),
            delimited(tag(" "), digit1, tag(" ")),
        ),
        line_ending,
    )(input)
}

fn stacks(input: &str) -> IResult<&str, Vec<VecDeque<u32>>> {
    let (input, u) = terminated(
        many1(stack_row),
        stack_indices,
    )(input)?;
    let n = u[0].len();
    let mut v = vec![VecDeque::new(); n];
    // push onto the stacks (in reverse order)
    for row in u {
        for (j, elem) in row.into_iter().enumerate() {
            match elem {
                None => continue,
                Some(x) => v[j].push_front(x),
            }
        }

    }

    Ok((input, v))
}

fn commands(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    many1(
        terminated(
            map(
                tuple((
                    preceded(
                        tag("move "),
                        digit1,
                    ),
                    preceded(
                        tag(" from "),
                        digit1,
                    ),
                    preceded(
                        tag(" to "),
                        digit1,
                    ),
                )),
                |(s, t, u)| {(
                    usize::from_str_radix(s, 10).unwrap(), // TODO bubble up error properly
                    usize::from_str_radix(t, 10).unwrap(),
                    usize::from_str_radix(u, 10).unwrap(),
                )},
            ),
            line_ending,
        )
    )(input)
}

fn total_input(input: &str) -> IResult<&str, (Vec<VecDeque<u32>>, Vec<(usize, usize, usize)>)> {
    separated_pair(
        stacks,
        line_ending,
        commands,
    )(input)
}

pub fn main(input: &str) -> Result<Either<String, (String, String)>> {
    // This "needs" to be an .unwrap() since the ? passes a nom error type that contains input, which generates a lifetime error.
    // TODO break the error type chain to remove the .unwrap()
    let (_, (mut stacks, commands)) = total_input(input).unwrap();

    let mut stacks2 = stacks.clone();
    let commands2 = commands.clone();

    for (n, src, dst) in commands {
        let (src, dst) = (src - 1, dst - 1); // commands are 1-indexed
        for _ in 0..n {
            let x = stacks[src].pop_back().context("invalid command")?;
            stacks[dst].push_back(x);
        }
    }

    let mut s = String::new();
    for stack in stacks {
        match stack.back() {
            Some(c) => s.push(char::from_u32(*c).context("invalid crate name")?),
            None => continue,
        }
    }

    // Part 2: the stacks are rearranged in order
    for (n, src, dst) in commands2 {
        let (src, dst) = (src - 1, dst - 1);
        let l = stacks2[src].len();
        let xs = stacks2[src].drain(l-n..l).collect::<Vec<_>>();
        stacks2[dst].extend(xs);
    }

    let mut t = String::new();
    for stack in stacks2 {
        match stack.back() {
            Some(c) => t.push(char::from_u32(*c).context("invalid crate name")?),
            None => continue,
        }
    }

    Ok(Right((s,t)))
}
