extern crate wiregrid;

use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};

use wiregrid::{Wire, closest_intersection};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let (mut ws1, mut ws2) = (String::new(), String::new());
    reader.read_line(&mut ws1)?;
    reader.read_line(&mut ws2)?;

    let (w1, w2) = (Wire::new(&ws1), Wire::new(&ws2));
    println!("Distance of closest intersection: {}",
        closest_intersection(&w1, &w2));

    Ok(())
}
