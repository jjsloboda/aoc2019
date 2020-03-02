use std::io;
use std::fs::read_to_string;

use cat6network::y_val_of_err_packet;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1 & 2
    println!("y value of err packet: {}", y_val_of_err_packet(&mem));

    Ok(())
}
