use std::{char, io};
use std::fs::read_to_string;

use brickgame::Arcade;
use getch::Getch;

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let mut mem: Vec<isize> = input.trim().split(',')
        .map(|x| x.parse::<isize>().expect("failed to parse input"))
        .collect();

    // Part 1
    let mut arcade = Arcade::new(mem.clone());
    arcade.start();
    println!("num initial block tiles: {}", arcade.num_block_tiles());

    // Part 2
    let g = Getch::new();
    mem[0] = 2;
    let mut arcade = Arcade::new(mem);
    arcade.start();
    while !arcade.is_ended() {
        println!("{}", arcade);
        let ch = char::from_u32(g.getch()? as u32).expect("bad getch");
        if ch == 'r' {
            arcade.load();
        } else {
            match ch {
                'j' => arcade.joystick_input(-1),
                'k' => arcade.joystick_input(0),
                'l' => arcade.joystick_input(1),
                's' => arcade.save(),
                'r' => arcade.load(),
                _ => {},
            };
        }
    }
    println!("Final score: {}", arcade.score());

    Ok(())
}
