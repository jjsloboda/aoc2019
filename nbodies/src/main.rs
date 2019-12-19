use std::io;

use std::fs::File;
use std::io::{BufReader, BufRead};

use num::integer::lcm;
use regex::Regex;

use nbodies::{Body, System};

fn main() -> io::Result<()> {
    let point_regex = Regex::new(
        r"^<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>$").unwrap();
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut bodies = Vec::new();
    for line in reader.lines() {
        let li = line.unwrap();
        let caps = point_regex.captures(&li).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let z = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        bodies.push(Body::new(x, y, z));
    }
    let system = System::new(bodies);
    /*
    let system = System::new(vec![
        Body::new(-1, 0, 2),
        Body::new(2, -10, -7),
        Body::new(4, -8, 8),
        Body::new(3, 5, -1),
    ]);
    let system = System::new(vec![
        Body::new(-8, -10, 0),
        Body::new(5, 5, 10),
        Body::new(2, -7, 3),
        Body::new(9, -8, -3),
    ]);
    */

    let mut system_part1 = system.clone();
    for _ in 0..1000 { system_part1.step_forward(); }
    println!("system energy after 1000 iterations: {}", system_part1.energy());

    let (orig_x, orig_y, orig_z) = system.pos_by_dim();
    let mut system_part2 = system.clone();
    system_part2.step_forward();
    let mut steps: u64 = 1;
    let (mut fx, mut fy, mut fz) = (false, false, false);
    let (mut x, mut y, mut z) = (0, 0, 0);
    while !(fx && fy && fz) {
        let (xv, yv, zv) = system_part2.pos_by_dim();
        if !fx && xv == orig_x {
            x = steps;
            fx = true;
            println!("x steps: {}", x)
        }
        if !fy && yv == orig_y {
            y = steps;
            fy = true;
            println!("y steps: {}", y)
        }
        if !fz && zv == orig_z {
            z = steps;
            fz = true;
            println!("z steps: {}", z)
        }
        system_part2.step_forward();
        steps += 1;
    }
    let total_steps = lcm(lcm(x, y), z);
    println!("total steps: {}", total_steps);

    Ok(())
}
