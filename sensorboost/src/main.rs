extern crate sensorboost;

use std::fs::read_to_string;

use sensorboost::intcode::{Processor, Resources};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();
    let processor = Processor::new_intcode();

    let mut res = Resources::new(mem.clone());
    res.write_input(1);
    processor.execute(&mut res);
    println!("boost code: {}", res.read_output());

    let mut res2 = Resources::new(mem);
    res2.write_input(2);
    processor.execute(&mut res2);
    println!("distress signal coords: {}", res2.read_output());
}
