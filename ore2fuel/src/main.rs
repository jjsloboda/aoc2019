use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
    }

    Ok(())
}
