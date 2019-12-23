use std::fmt;

mod intcode;
use intcode::{Processor, Resources};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
}
impl Tile {
    pub fn new(x: isize) -> Self {
        match x {
            0 => Tile::EMPTY,
            1 => Tile::WALL,
            2 => Tile::BLOCK,
            3 => Tile::PADDLE,
            4 => Tile::BALL,
            _ => panic!("invalid tile"),
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Tile::EMPTY => ' ',
            Tile::WALL => '*',
            Tile::BLOCK => '#',
            Tile::PADDLE => '-',
            Tile::BALL => 'o',
        };
        write!(f, "{}", ch)
    }
}

const DIM_X: usize = 50;
const DIM_Y: usize = 30;
#[derive(Clone)]
struct Screen {
    tiles: [[Tile; DIM_X]; DIM_Y],
    score: isize,
}
impl Screen {
    pub fn new() -> Self {
        Screen{
            tiles: [[Tile::EMPTY; DIM_X]; DIM_Y],
            score: 0,
        }
    }
    pub fn set_tile(&mut self, x: isize, y: isize, t: isize) {
        if x == -1 && y == 0 {
            self.score = t;
        } else {
            let (xp, yp) = (x as usize, y as usize);
            let tile = Tile::new(t);
            self.tiles[yp][xp] = tile;
        }
    }
    pub fn num_block_tiles(&self) -> i32 {
        self.tiles.iter().map(
            |row| row.iter().filter(|t| **t == Tile::BLOCK).count()).sum::<usize>() as i32
    }
}
impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Score: {}\n", self.score)?;
        for y in 0..DIM_Y {
            for x in 0..DIM_X {
                write!(f, "{}", self.tiles[y][x])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub struct Arcade {
    proc: Processor,
    res: Resources,
    screen: Screen,
    save: Option<(Resources, Screen)>,
}
impl Arcade {
    pub fn new(mem: Vec<isize>) -> Self {
        Arcade{
            proc: Processor::new_intcode(),
            res: Resources::new(mem),
            screen: Screen::new(),
            save: None,
        }
    }
    fn draw_output_to_screen(&mut self) {
        loop {
            let x = match self.res.read_output() {
                Some(x) => x,
                None => { break; },
            };
            let y = match self.res.read_output() {
                Some(y) => y,
                None => { break; },
            };
            let t = match self.res.read_output() {
                Some(t) => t,
                None => { break; },
            };
            self.screen.set_tile(x, y, t);
        }
    }
    pub fn start(&mut self) {
        self.proc.execute(&mut self.res);
        self.draw_output_to_screen();
    }
    pub fn is_ended(&self) -> bool {
        self.res.get_status() == intcode::Status::TERMINATED
    }
    pub fn joystick_input(&mut self, i: isize) {
        self.res.write_input(i);
        self.proc.resume(&mut self.res);
        self.draw_output_to_screen();
    }
    pub fn num_block_tiles(&self) -> i32 {
        self.screen.num_block_tiles()
    }
    pub fn save(&mut self) {
        self.save = Some((self.res.clone(), self.screen.clone()))
    }
    pub fn load(&mut self) {
        if let Some(save) = self.save.clone() {
            self.res = save.0;
            self.screen = save.1;
        }
    }
    pub fn score(&self) -> isize {
        self.res.read_mem(386)
    }
}
impl fmt::Display for Arcade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.screen)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
