use std::fs::File;
use std::io;
use std::io::BufReader;

use ceresstation::{points_from_data, max_asteriod_visibility};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let asteriods = points_from_data(reader);

    println!("max visible asteriods: {}", max_asteriod_visibility(&asteriods));

    Ok(())
}
