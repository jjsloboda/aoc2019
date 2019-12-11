extern crate ampcircuit;

use std::fs::read_to_string;

mod permutations;
use permutations::permutation_indices;

use ampcircuit::{run_phase_settings, run_feedback_loop};
use ampcircuit::intcode::Processor;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();
    let processor = Processor::new_intcode();

    let mut max_output = isize::min_value();
    for phase_settings in permutation_indices(5) {
        let output = run_phase_settings(&processor, &mem, &phase_settings);
        if output > max_output {
            max_output = output;
        }
    }
    println!("max_output: {}", max_output);

    let mut max_feedback_output = isize::min_value();
    for phase_settings in permutation_indices(5) {
        let new_phase_settings = phase_settings.iter().map(|x| x + 5).collect();
        let output = run_feedback_loop(&processor, &mem, &new_phase_settings);
        if output > max_feedback_output {
            max_feedback_output = output;
        }
    }
    println!("max feedback loop output: {}", max_feedback_output);
}
