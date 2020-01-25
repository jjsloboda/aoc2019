use std::io;
use std::fs::read_to_string;

use aftscaffold::{intersection_alignment_sum, total_space_dust};

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    let ialign_sum = intersection_alignment_sum(&mem);
    println!("intersection alignment sum: {}", ialign_sum);

    // Part 2
    let total = total_space_dust(&mem);
    println!("total space dust: {}", total);

    Ok(())
}
