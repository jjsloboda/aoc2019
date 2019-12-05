extern crate venuscode;

use std::io;

use venuscode::num_possible_codes;

fn main() -> io::Result<()> {
    const START: i32 = 123;
    const END: i32 = 567;

    let code_count = num_possible_codes(START, END);
    println!("Number of possible codes: {}", code_count);

    Ok(())
}
