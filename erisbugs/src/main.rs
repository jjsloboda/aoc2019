use std::fs::File;
use std::io;
use std::io::BufReader;

use erisbugs::{load_grid, find_first_repeated, find_num_bugs};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let grid = load_grid(reader);

    // Part 1
    let first_rep = find_first_repeated(&grid);
    println!("first repeated pattern: {}", first_rep);

    // Part 2
    let mins = 200;
    let num_bugs = find_num_bugs(&grid, mins);
    println!("num bugs after {} minutes: {}", mins, num_bugs);

    Ok(())
}
