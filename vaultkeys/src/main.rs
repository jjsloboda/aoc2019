use std::fs::File;
use std::io;
use std::io::BufReader;

use vaultkeys::fewest_steps_for_all_keys;

fn main() -> io::Result<()> {
    // Part 1
    let file1 = File::open("input.txt")?;
    let reader1 = BufReader::new(file1);
    let steps1 = fewest_steps_for_all_keys(reader1);
    println!("fewest steps for all keys: {}", steps1.expect("cannot collect all keys"));

    // Part 2
    let file2 = File::open("input2.txt")?;
    let reader2 = BufReader::new(file2);
    let steps2 = fewest_steps_for_all_keys(reader2);
    println!("fewest steps for all keys with 4 bots: {}",
        steps2.expect("cannot collect all keys"));

    Ok(())
}
