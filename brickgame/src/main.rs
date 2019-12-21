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
    let mut move_num = 0;
    let g = Getch::new();
    mem[0] = 2;
    let mut arcade = Arcade::new(mem);
    let mut save = arcade.save();
    arcade.start();
    while !arcade.is_ended() {
        println!("{}", arcade);
        let ch = char::from_u32(g.getch()? as u32).expect("bad getch");
        move_num += 1;
        if ch == 'r' {
            arcade.load(&save);
        } else {
            let js = match ch {
                'j' => -1,
                'k' => 0,
                'l' => 1,
                _ => panic!("bad input"),
            };
            arcade.joystick_input(js);
            if move_num % 100 == 0 {
                save = arcade.save();
            }
        }
    }

    Ok(())
}
