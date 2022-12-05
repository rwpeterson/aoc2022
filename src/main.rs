#[allow(unused_imports)]
use anyhow::{Result, Context};
#[allow(unused_imports)]
use either::{Either, Left, Right};
use std::fs::read_to_string;

mod day3;
mod day4;

const HELP: &str = "\
Bob - Advent of Code
USAGE:
  aoc [OPTIONS] DAY
FLAGS:
  -h, --help            Prints help information
OPTIONS:
  --part NUM            Specifies only one part
ARGS:
  DAY
";

#[derive(Debug)]
struct AppArgs {
    example: bool,
    day: u32
}

fn main() -> Result<()> {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let filepath = format!(
        "{}/day{}.txt",
        if args.example { "example" } else { "input" },
        args.day,
    );

    let input = read_to_string(&filepath)?;

    let output = match args.day {
        3 => day3::main(&input)?,
        4 => day4::main(&input)?,
        x => {
            eprintln!("Error: Day {} not implemented", x);
            std::process::exit(1);
        }
    };

    match output {
        Left(s) => println!("{}", s),
        Right((s, t)) => println!("{}\n{}", s, t),
    }
    Ok(())
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        example: pargs.contains(["-e", "--example"]),
        day: pargs.free_from_str()?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}
