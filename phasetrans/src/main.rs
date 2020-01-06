use std::{char, io};
use std::fs::read_to_string;

use phasetrans::calculate_phases_2;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let signal: Vec<i32> = input.trim().chars()
        .map(|ch| ch.to_digit(10).expect("failed to parse input") as i32)
        .collect();

    // Part 1
    let post_100 = calculate_phases_2(100, &signal);
    let first_8 = post_100[..8].iter()
        .map(|d| char::from_digit(*d as u32, 10).expect("bad output"))
        .collect::<String>();
    println!("first 8 after 100 phases: {}", first_8);

    // Part 2
    let mut big_signal: Vec<i32> = Vec::with_capacity(signal.len() * 10_000);
    for _ in 0..10_000 {
        big_signal.extend(signal.iter());
    }
    let offset = signal[..7].iter().fold(0, |acc, x| acc * 10 + x) as usize;
    let big_post_100 = calculate_phases_2(1, &big_signal);
    let big_8 = big_post_100[offset..offset+8].iter()
        .map(|d| char::from_digit(*d as u32, 10).expect("bad output"))
        .collect::<String>();
    println!("offset 8 after 10000x input: {}", big_8);

    Ok(())
}
