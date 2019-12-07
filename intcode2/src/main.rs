extern crate intcode2;

use std::fs::read_to_string;

use intcode2::InstructionSet;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();
    let processor = InstructionSet::new_intcode();
    processor.execute(&mut mem);
}
