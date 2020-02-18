use std::fs::File;
use std::io;
use std::io::BufReader;

use slamshuffle::shuffle_cards;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Part 1
    let cards = shuffle_cards(reader);
    for i in 0..cards.len() {
        if cards[i] == 2019 {
            println!("card 2019 is at position {}", i);
        }
    }

    Ok(())
}
