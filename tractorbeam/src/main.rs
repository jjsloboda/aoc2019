use std::io;
use std::fs::read_to_string;

use tractorbeam::scan_immediate_area;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    let num_influenced_squares = scan_immediate_area(&mem);
    println!("num influenced squares: {}", num_influenced_squares);

    // Part 2

    Ok(())
}
