use std::fs::File;
use std::io;
use std::io::BufReader;

use erisbugs::{load_grid, print_grid};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let grid = load_grid(reader);

    // Part 1
    print_grid(&grid);

    Ok(())
}
