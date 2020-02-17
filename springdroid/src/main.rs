use std::io;
use std::fs::read_to_string;

use springdroid::run_with_first_prgm;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    run_with_first_prgm(&mem);

    // Part 2
    // TODO

    Ok(())
}
