use std::io;
use std::fs::read_to_string;

use paintbot::{paint_hull_with_robot, Robot, Hull};

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    let mut robot = Robot::new(mem);
    let mut hull = Hull::new();
    paint_hull_with_robot(&mut hull, &mut robot);

    println!("num tiles painted: {}", hull.num_tiles_painted());

    Ok(())
}
