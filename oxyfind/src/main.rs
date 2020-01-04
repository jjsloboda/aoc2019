use std::io;
use std::fs::read_to_string;

use oxyfind::min_distance_to_oxygen;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    let min_dist = min_distance_to_oxygen(mem).expect("no oxygen found");
    println!("min distance to oxygen: {}", min_dist);

    Ok(())
}
