use std::fs::File;
use std::io;
use std::io::BufReader;

use slamshuffle::{fully_shuffle_card, fully_deshuffle_card};

fn main() -> io::Result<()> {

    // Part 1
    let file1 = File::open("input.txt")?;
    let reader1 = BufReader::new(file1);
    let card1 = fully_shuffle_card(reader1, 2019, 10007, 1);
    println!("card 2019 is at position {}", card1);

    // Sanity check
    let file4 = File::open("input.txt")?;
    let reader4 = BufReader::new(file4);
    let c2 = fully_deshuffle_card(reader4, 6526, 10007, 1);
    println!("sanity check: card at 6526 deshuffles to {}", c2);

    // Part 2
    let file2 = File::open("input.txt")?;
    let reader2 = BufReader::new(file2);
    let card = fully_deshuffle_card(reader2, 2020, 119315717514047, 101741582076661);
    println!("card in position 2020 is {}", card);

    Ok(())
}
