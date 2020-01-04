use std::fs::File;
use std::io::{self, BufReader};

use ore2fuel::{load_rxns, min_ore_qty_for_fuel, max_fuel_qty_for_ore};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let rxns = load_rxns(reader);

    // Part 1
    let min_ore_qty = min_ore_qty_for_fuel(1, &rxns);
    println!("min ore required for a fuel: {}", min_ore_qty);

    // Part 2
    let max_fuel_qty = max_fuel_qty_for_ore(1_000_000_000_000, &rxns);
    println!("max fuel obtainable from 1T ore: {}", max_fuel_qty);

    Ok(())
}
