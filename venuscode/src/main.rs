extern crate venuscode;

use std::io;

use venuscode::num_possible_codes;

fn main() -> io::Result<()> {
    const START: i32 = 245318;
    const END: i32 = 765747;

    let code_count = num_possible_codes(START, END);
    println!("Number of possible codes: {}", code_count);

    Ok(())
}
