use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};

use ceresstation::Point;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    for line in reader.lines() {
    }
}
