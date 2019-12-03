extern crate intcode;

use std::fs::read_to_string;

use intcode::run_with_inputs;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mem: Vec<i32> = input.trim().split(',').map(
        |x| x.parse::<i32>().unwrap()).collect();
    let result = run_with_inputs(12, 2, &mem);
    println!("part one result: {}", result);

    for a in 0..100 {
        for b in 0..100 {
            if 19690720 == run_with_inputs(a, b, &mem) {
                println!("part two result: {}{}", a, b);
            }
        }
    }
}
