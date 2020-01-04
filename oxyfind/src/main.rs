use std::io;
use std::fs::read_to_string;

use oxyfind::{min_distance_to_oxygen, max_time_to_oxygenation};

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    let min_dist = min_distance_to_oxygen(mem.clone()).expect("no oxygen found");
    println!("min distance to oxygen: {}", min_dist);

    // Part 2
    let max_time = max_time_to_oxygenation(mem).expect("no oxygen found");
    println!("time until oxygen filled: {} m", max_time);

    Ok(())
}
