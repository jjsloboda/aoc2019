use std::io;
use std::fs::read_to_string;

use springdroid::{walk_with_first_prgm, run_with_second_prgm};

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    walk_with_first_prgm(&mem);

    // Part 2
    run_with_second_prgm(&mem);

    Ok(())
}
