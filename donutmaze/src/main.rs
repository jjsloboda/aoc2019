use std::fs::File;
use std::io;
use std::io::BufReader;

use donutmaze::{find_shortest_path, find_shortest_recursive_path};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Part 1
    let distance = find_shortest_path(reader);
    println!("shortest path: {}", distance.expect("no path found"));

    // Part 2
    let file2 = File::open("input.txt")?;
    let reader2 = BufReader::new(file2);
    let recursive_distance = find_shortest_recursive_path(reader2);
    println!("shortest recursive path: {}", recursive_distance.expect("no path found"));

    Ok(())
}
