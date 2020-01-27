use std::fs::File;
use std::io;
use std::io::BufReader;

use vaultkeys::fewest_steps_for_all_keys;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Part 1
    let steps = fewest_steps_for_all_keys(reader);
    println!("fewest steps for all keys: {}", steps.expect("cannot collect all keys"));

    // Part 2
    // TODO

    Ok(())
}
