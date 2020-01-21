use std::fmt;

mod intcode;
use intcode::{Processor, Resources};

#[derive(PartialEq)]
enum RobotStatus {
    LEFT,
    UP,
    RIGHT,
    DOWN,
    TUMBLING,
}

#[derive(PartialEq)]
enum Cell {
    SPACE,
    SCAFFOLD,
    ROBOT(RobotStatus),
}
impl Cell {
    pub fn from_code(code: isize) -> Self {
        match code as u8 as char {
            '.' => Cell::SPACE,
            '#' => Cell::SCAFFOLD,
            '<' => Cell::ROBOT(RobotStatus::LEFT),
            '^' => Cell::ROBOT(RobotStatus::UP),
            '>' => Cell::ROBOT(RobotStatus::RIGHT),
            'v' => Cell::ROBOT(RobotStatus::DOWN),
            'X' => Cell::ROBOT(RobotStatus::TUMBLING),
            _ => panic!("invalid input code: {}", code),
        }
    }
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Cell::SPACE => '.',
            Cell::SCAFFOLD => '#',
            Cell::ROBOT(RobotStatus::LEFT) => '<',
            Cell::ROBOT(RobotStatus::UP) => '^',
            Cell::ROBOT(RobotStatus::RIGHT) => '>',
            Cell::ROBOT(RobotStatus::DOWN) => 'v',
            Cell::ROBOT(RobotStatus::TUMBLING) => 'X',
        };
        write!(f, "{}", ch)
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    pub fn from_intcode(mem: Vec<isize>) -> Self {
        let proc = Processor::new_intcode();
        let mut res = Resources::new(mem);
        proc.execute(&mut res);
        let mut cells = Vec::new();
        let mut row = Vec::new();
        loop {
            if let Some(code) = res.read_output() {
                if code == 10 {
                    if row.len() > 0 {
                        cells.push(row);
                        row = Vec::new();
                    }
                } else {
                    row.push(Cell::from_code(code));
                }
            } else {
                break;
            }
        }
        Grid{ cells: cells }
    }
    pub fn intersections(&self) -> Vec<(usize, usize)> {
        let mut isects = Vec::new();
        for y in 1..self.cells.len()-1 {
            for x in 1..self.cells[0].len()-1 {
                if self.cells[y][x] == Cell::SCAFFOLD &&
                    self.cells[y-1][x] == Cell::SCAFFOLD &&
                    self.cells[y+1][x] == Cell::SCAFFOLD &&
                    self.cells[y][x-1] == Cell::SCAFFOLD &&
                    self.cells[y][x+1] == Cell::SCAFFOLD {
                        isects.push((x, y));
                }
            }
        }
        isects
    }
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in self.cells.iter() {
            for c in r.iter() {
                write!(f, "{}", c)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

pub fn intersection_alignment_sum(mem: Vec<isize>) -> usize {
    let grid = Grid::from_intcode(mem);
    print!("grid:\n{}", grid);
    grid.intersections().iter().map(|(x, y)| x*y).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
