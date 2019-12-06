extern crate venuscode;

use std::io;

use venuscode::{num_initial_possible_codes, num_actual_possible_codes};

fn main() -> io::Result<()> {
    const START: i32 = 245318;
    const END: i32 = 765747;

    let initial_code_count = num_initial_possible_codes(START, END);
    println!("Number of initial possible codes: {}", initial_code_count);

    let actual_code_count = num_actual_possible_codes(START, END);
    println!("Number of actual possible codes: {}", actual_code_count);

    Ok(())
}
