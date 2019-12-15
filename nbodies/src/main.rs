use std::io;

use std::fs::File;
use std::io::{BufReader, BufRead};

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

    let mut system_part1 = system.clone();
    for _ in 0..1000 { system_part1.step_forward(); }
    println!("system energy after 1000 iterations: {}", system_part1.energy());

    let mut system_part2 = system.clone();
    system_part2.step_forward();
    let mut steps: u64 = 1;
    while system_part2 != system && steps < 5000000000 {
        system_part2.step_forward();
        steps += 1;
    }
    println!("steps to repeat: {}", steps);

    Ok(())
}
