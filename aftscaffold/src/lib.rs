use std::cmp::min;
use std::fmt;

mod intcode;
use intcode::{Processor, Resources, Status};

#[derive(Debug)]
enum Movement {
    STRAIGHT,
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}
impl Direction {
    pub fn after_movement(&self, m: Movement) -> Direction {
        match m {
            Movement::LEFT => match self {
                Direction::LEFT => Direction::DOWN,
                Direction::UP => Direction::LEFT,
                Direction::RIGHT => Direction::UP,
                Direction::DOWN => Direction::RIGHT,
            },
            Movement::RIGHT => match self {
                Direction::LEFT => Direction::UP,
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
            },
            Movement::STRAIGHT => *self,
        }
    }
}

struct Robot {
    x: usize,
    y: usize,
    d: Direction,
}
impl Robot {
    pub fn from_intcode(mem: Vec<isize>) -> Option<Self> {
        let proc = Processor::new_intcode();
        let mut res = Resources::new(mem);
        proc.execute(&mut res);
        let (mut x, mut y): (usize, usize) = (0, 0);
        loop {
            if let Some(code) = res.read_output() {
                if code == 10 {
                    x = 0;
                    y += 1;
                }
                match code as u8 as char {
                    '<' => return Some(Robot{x: x, y: y, d: Direction::LEFT}),
                    '^' => return Some(Robot{x: x, y: y, d: Direction::UP}),
                    '>' => return Some(Robot{x: x, y: y, d: Direction::RIGHT}),
                    'v' => return Some(Robot{x: x, y: y, d: Direction::DOWN}),
                    _ => {},
                }
                if code != 10 {
                    x += 1;
                }
            } else {
                break;
            }
        }
        None
    }
    pub fn loc_after_movement(&self, m: Movement) -> (isize, isize) {
        let (x, y) = (self.x as isize, self.y as isize);
        match self.d.after_movement(m) {
            Direction::LEFT => (x-1, y),
            Direction::UP => (x, y-1),
            Direction::RIGHT => (x+1, y),
            Direction::DOWN => (x, y+1),
        }
    }
    pub fn advance(&mut self, grid: &Grid) -> Option<Movement> {
        let loc_straight_ahead = self.loc_after_movement(Movement::STRAIGHT);
        let loc_leftward = self.loc_after_movement(Movement::LEFT);
        let loc_rightward = self.loc_after_movement(Movement::RIGHT);
        if grid.loc_is_valid(loc_straight_ahead) {
            self.x = loc_straight_ahead.0 as usize;
            self.y = loc_straight_ahead.1 as usize;
            self.d = self.d.after_movement(Movement::STRAIGHT);
            Some(Movement::STRAIGHT)
        } else if grid.loc_is_valid(loc_leftward) {
            self.x = loc_leftward.0 as usize;
            self.y = loc_leftward.1 as usize;
            self.d = self.d.after_movement(Movement::LEFT);
            Some(Movement::LEFT)
        } else if grid.loc_is_valid(loc_rightward) {
            self.x = loc_rightward.0 as usize;
            self.y = loc_rightward.1 as usize;
            self.d = self.d.after_movement(Movement::RIGHT);
            Some(Movement::RIGHT)
        } else {
            None
        }
    }
}
impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self.d {
            Direction::LEFT => '<',
            Direction::UP => '^',
            Direction::RIGHT => '>',
            Direction::DOWN => 'v',
        };
        write!(f, "{}", ch)
    }
}

#[derive(PartialEq)]
enum Cell {
    SPACE,
    SCAFFOLD,
}
impl Cell {
    pub fn from_code(code: isize) -> Self {
        match code as u8 as char {
            '.' => Cell::SPACE,
            '#' | '<' | '^' | '>' | 'v' => Cell::SCAFFOLD,
            _ => panic!("invalid input code: {}", code),
        }
    }
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Cell::SPACE => '.',
            Cell::SCAFFOLD => '#',
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
        for y in 1..self.height()-1 {
            for x in 1..self.width()-1 {
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
    pub fn width(&self) -> usize {
        self.cells[0].len()
    }
    pub fn height(&self) -> usize {
        self.cells.len()
    }
    pub fn loc_is_valid(&self, loc: (isize, isize)) -> bool {
        loc.0 >= 0 && (loc.0 as usize) < self.width() &&
            loc.1 >= 0 && (loc.1 as usize) < self.height() &&
            self.cells[loc.1 as usize][loc.0 as usize] == Cell::SCAFFOLD
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

#[derive(Debug)]
enum Instruction {
    LEFT,
    RIGHT,
    STRAIGHT(i64),
}

pub fn intersection_alignment_sum(mem: &Vec<isize>) -> usize {
    let grid = Grid::from_intcode(mem.clone());
    print!("grid:\n{}", grid);
    grid.intersections().iter().map(|(x, y)| x*y).sum()
}

fn generate_instruction_list(mem: &Vec<isize>) -> Vec<Instruction> {
    let mut robot = Robot::from_intcode(mem.clone()).expect("no robot found");
    let grid = Grid::from_intcode(mem.clone());
    let mut movecount = 0;
    let mut instruction_list = Vec::new();
    loop {
        if let Some(m) = robot.advance(&grid) {
            match m {
                Movement::LEFT => {
                    if movecount > 0 {
                        instruction_list.push(Instruction::STRAIGHT(movecount));
                    }
                    instruction_list.push(Instruction::LEFT);
                    movecount = 1;
                },
                Movement::RIGHT => {
                    if movecount > 0 {
                        instruction_list.push(Instruction::STRAIGHT(movecount));
                    }
                    instruction_list.push(Instruction::RIGHT);
                    movecount = 1;
                },
                Movement::STRAIGHT => movecount += 1,
            }
        } else {
            break;
        }
    }
    if movecount > 0 {
        instruction_list.push(Instruction::STRAIGHT(movecount));
    }
    instruction_list
}

fn abbreviate_instruction_list(insts: &Vec<Instruction>) -> Vec<i64> {
    let mut out = Vec::new();
    for i in 0..(insts.len()/2) {
        let mut n = 0;
        if let Instruction::STRAIGHT(x) = insts[i*2+1] {
            n = x;
        }
        if let Instruction::LEFT = insts[i*2] {
            n = -n;
        }
        out.push(n);
    }
    out
}

fn replace_subsequences(s: &[i64], sub: &[i64], rep: &[i64]) -> Vec<i64> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if sub[..] == s[i..min(s.len(), i+sub.len())] {
            out.extend(rep);
            i += sub.len();
        } else {
            out.push(s[i]);
            i += 1;
        }
    }
    out
}

fn find_suitable_subsequences_helper(s: &Vec<i64>, so_far: Vec<Vec<i64>>) -> Option<Vec<Vec<i64>>> {
    if so_far.len() == 3 {
        if s.is_empty() {
            Some(so_far)
        } else {
            None
        }
    } else {
        for i in (1..min(6, s.len())).rev() {
            let subseq = &s[0..i];
            let new_s = replace_subsequences(&s, subseq, &vec![]);
            let mut new_so_far = so_far.clone();
            new_so_far.push(subseq.to_vec());
            if let Some(seqs) = find_suitable_subsequences_helper(&new_s, new_so_far) {
                return Some(seqs);
            }
        }
        None
    }
}

fn find_suitable_subsequences(s: &[i64]) -> Option<Vec<Vec<i64>>> {
    find_suitable_subsequences_helper(&s.to_vec(), Vec::new())
}

fn add_commas_and_newline(s: &[i64]) -> Vec<i64> {
    let mut out = Vec::new();
    for i in 0..s.len() {
        out.push(s[i]);
        if i < s.len() - 1 {
            out.push(',' as i64);
        } else {
            out.push(10);
        }
    }
    out
}

fn abbreviated_sequence_to_output(s: &[i64]) -> Vec<i64> {
    let mut out = Vec::new();
    for i in 0..s.len() {
        if s[i] < 0 {
            out.push('L' as i64);
            out.push(',' as i64);
        } else {
            out.push('R' as i64);
            out.push(',' as i64);
        }
        out.extend(s[i].abs().to_string().chars().map(|d| d as i64));
        if i < s.len() - 1 {
            out.push(',' as i64);
        } else {
            out.push(10);
        }
    }
    out
}

pub fn total_space_dust(mem: &Vec<isize>) -> usize {
    let insts = generate_instruction_list(mem);
    let abbr_insts = abbreviate_instruction_list(&insts);
    let subseqs = find_suitable_subsequences(&abbr_insts)
        .expect("no suitable subsequence breakdown found");
    let mut main_seq = abbr_insts.clone();
    main_seq = replace_subsequences(&main_seq, &subseqs[0], &vec!['A' as i64]);
    main_seq = replace_subsequences(&main_seq, &subseqs[1], &vec!['B' as i64]);
    main_seq = replace_subsequences(&main_seq, &subseqs[2], &vec!['C' as i64]);
    main_seq = add_commas_and_newline(&main_seq);
    println!("main: {:?}, funcs: {:?}", main_seq, subseqs);

    let proc = Processor::new_intcode();
    let mut res = Resources::new(mem.clone());
    res.write_mem(0, 2);
    for x in main_seq.iter() {
        res.write_input(*x as isize);
        println!("sent to input: {}", *x);
    }
    for ss in subseqs.iter() {
        let routine = abbreviated_sequence_to_output(ss);
        for x in routine.iter() {
            res.write_input(*x as isize);
            println!("sent to input: {}", *x);
        }
    }
    res.write_input('n' as isize);
    res.write_input(10 as isize);
    proc.execute(&mut res);
    assert_eq!(Status::TERMINATED, res.get_status());
    let mut total = 0;
    loop {
        if let Some(x) = res.read_output() {
            total = x;
        } else {
            break;
        }
    }
    total as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_instruction_to_string() {
        use super::Instruction;
        assert_eq!(vec!['L' as u8], Instruction::LEFT.to_ascii());
        assert_eq!(vec!['R' as u8], Instruction::RIGHT.to_ascii());
        assert_eq!(vec!['1' as u8, '2' as u8], Instruction::STRAIGHT(12).to_ascii());
    }

    #[test]
    fn check_replace_subsequences() {
        use super::replace_subsequences;
        assert_eq!(vec![4, 5, 2, 3],
            replace_subsequences(
                &vec![1, 2, 3, 4, 5, 1, 2, 3, 2, 3], &vec![1, 2, 3], &vec![]));
        assert_eq!(vec![65, 4, 5, 65, 2, 3],
            replace_subsequences(
                &vec![1, 2, 3, 4, 5, 1, 2, 3, 2, 3], &vec![1, 2, 3], &vec![65]));
    }

    #[test]
    fn check_find_suitable_subsequences() {
        use super::find_suitable_subsequences;
        assert_eq!(vec![vec![1, 2, 3, 4, 5], vec![1], vec![2, 3]],
            find_suitable_subsequences(&vec![1, 2, 3, 4, 5, 1, 2, 3, 2, 3]).unwrap());
        assert_eq!(vec![
                vec![-4, 8, -6, -10],
                vec![-6, 8, 10, -6, -6],
                vec![-4, -4, -10],
            ],
            find_suitable_subsequences(&vec![
                -4, 8, -6, -10, -6, 8, 10, -6, -6, -4, 8, -6, -10, -6, 8, 10,
                -6, -6, -4, -4, -10, -4, -4, -10, -6, 8, 10, -6, -6, -4, 8, -6,
                -10, -6, 8, 10, -6, -6, -4, -4, -10
            ]).unwrap());
    }

    #[test]
    fn check_abbreviated_sequence_to_output() {
        use super::abbreviated_sequence_to_output;
        assert_eq!(vec![
                'L' as i64, ',' as i64, '4' as i64, ',' as i64,
                'R' as i64, ',' as i64, '8' as i64, ',' as i64,
                'L' as i64, ',' as i64, '6' as i64, ',' as i64,
                'L' as i64, ',' as i64, '1' as i64, '0' as i64, 10,
            ],
            abbreviated_sequence_to_output(&vec![-4, 8, -6, -10]));
    }
}
