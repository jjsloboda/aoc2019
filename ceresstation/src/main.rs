use std::fs::File;
use std::io;
use std::io::BufReader;

use ceresstation::{
    points_from_data, max_asteriod_visibility, laser_blast_order};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut asteriods = points_from_data(reader);

    // Part 1
    let (max_visible_asteriod, max_num_visible) =
        max_asteriod_visibility(&asteriods);
    println!("max visible asteriod: {}", max_visible_asteriod);
    println!("max num visible: {}", max_num_visible);

    // Part 2
    asteriods.remove(&max_visible_asteriod);
    match laser_blast_order(&max_visible_asteriod, &asteriods, 200) {
        Some(x) => println!("200th asteriod is at {}", x),
        None => println!("No asteriod for index {}", 200),
    }


    Ok(())
}
