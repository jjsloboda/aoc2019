use std::fs::File;
use std::io::{self, BufReader};

use ore2fuel::{load_rxns, min_ore_qty_for_fuel};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Part 1
    let rxns = load_rxns(reader);
    let min_ore_qty = min_ore_qty_for_fuel(&rxns);
    println!("min ore required for a fuel: {}", min_ore_qty);

    Ok(())
}
