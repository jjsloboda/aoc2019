extern crate ampcircuit;

use std::fs::read_to_string;

mod permutations;
use permutations::permutation_indices;

use ampcircuit::run_phase_settings;
use ampcircuit::intcode::Processor;

fn main() {
    let perms = permutation_indices(5);
    let input = read_to_string("input.txt").unwrap();
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();
    let processor = Processor::new_intcode();
    let mut max_output = isize::min_value();
    for phase_settings in perms {
        let output = run_phase_settings(&processor, &mem, &phase_settings);
        if output > max_output {
            max_output = output;
        }
    }
    println!("max_output: {}", max_output);
}
