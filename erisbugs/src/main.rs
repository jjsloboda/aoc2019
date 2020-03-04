use std::fs::File;
use std::io;
use std::io::BufReader;

use erisbugs::{load_grid, find_first_repeated};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let grid = load_grid(reader);

    // Part 1
    let first_rep = find_first_repeated(&grid);
    println!("first repeated pattern: {}", first_rep);

    // Part 2

    Ok(())
}
