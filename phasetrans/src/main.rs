use std::{char, io};
use std::fs::read_to_string;

use phasetrans::calculate_phases;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let signal: Vec<i32> = input.trim().chars()
        .map(|ch| ch.to_digit(10).expect("failed to parse input") as i32)
        .collect();

    // Part 1
    let post_100 = calculate_phases(100, &signal);
    let first_8 = post_100[..8].iter()
        .map(|d| char::from_digit(*d as u32, 10).expect("bad output"))
        .collect::<String>();
    println!("first 8 after 100 phases: {}", first_8);

    Ok(())
}
