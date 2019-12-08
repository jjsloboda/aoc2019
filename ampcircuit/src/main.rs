extern crate ampcircuit;

use std::fs::read_to_string;

mod intcode;
use intcode::{Processor, Resources};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();
    let processor = Processor::new_intcode();
    let mut resources = Resources::new(mem);
    processor.execute(&mut resources);
}
