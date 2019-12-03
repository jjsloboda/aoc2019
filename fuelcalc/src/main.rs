extern crate fuelcalc;

use std::fs::File;
use std::io::{BufReader, BufRead};

use fuelcalc::{fuel_4_mass, fuel_integration};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut naive_total: i64 = 0;
    let mut integrated_total: i64 = 0;
    for line in reader.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        naive_total += fuel_4_mass(num) as i64;
        integrated_total += fuel_integration(num) as i64;
    }
    println!("Part 1: Naive total: {}", naive_total);
    println!("Part 2: Integrated total: {}", integrated_total);
}
