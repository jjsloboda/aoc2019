use std::io;
use std::fs::read_to_string;

use cat6network::y_val_of_255_packet;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    println!("y value of first packet to 255: {}", y_val_of_255_packet(&mem));

    // Part 2

    Ok(())
}
