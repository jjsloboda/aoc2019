use std::io;
use std::fs::read_to_string;

use cryostasis::run_droid;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    run_droid(&mem)?;

    Ok(())
}

