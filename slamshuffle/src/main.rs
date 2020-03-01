use std::fs::File;
use std::io;
use std::io::BufReader;

use slamshuffle::{fully_deshuffle_card, shuffle_cards};

fn main() -> io::Result<()> {

    // Part 1
    let file1 = File::open("input.txt")?;
    let reader1 = BufReader::new(file1);
    let cards = shuffle_cards(reader1);
    for i in 0..cards.len() {
        if cards[i] == 2019 {
            println!("card 2019 is at position {}", i);
        }
    }

    // Part 2
    let file2 = File::open("input.txt")?;
    let reader2 = BufReader::new(file2);
    let card = fully_deshuffle_card(reader2, 2020, 119315717514047, 1000000000/*101741582076661*/);
    println!("card in position 2020 is {}", card);

    Ok(())
}
