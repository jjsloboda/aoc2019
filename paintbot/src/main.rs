use std::io;
use std::fs::read_to_string;

use paintbot::{paint_hull_with_robot, Robot, Hull, Point, WHITE};

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    let mut robot = Robot::new(mem.clone());
    let mut hull = Hull::new();
    paint_hull_with_robot(&mut hull, &mut robot);
    println!("num tiles painted: {}", hull.num_tiles_painted());

    let mut robot = Robot::new(mem);
    let mut hull = Hull::new();
    hull.set_color_at_loc(&Point::new(0, 0), WHITE);
    paint_hull_with_robot(&mut hull, &mut robot);
    println!("{}", hull);
    println!("{}", hull.num_white_tiles());
    println!("num white tiles: {}", hull.num_white_tiles());

    Ok(())
}
