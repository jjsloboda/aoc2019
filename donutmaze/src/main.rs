use std::fs::File;
use std::io;
use std::io::BufReader;

use donutmaze::find_shortest_path;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let distance = find_shortest_path(reader);
    println!("shortest path: {}", distance.expect("no path found"));

    Ok(())
}
