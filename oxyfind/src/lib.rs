use std::collections::{VecDeque, HashSet};

mod intcode;
use intcode::{Processor, Resources};

// Inputs
const NORTH: isize = 1;
const SOUTH: isize = 2;
const WEST: isize = 3;
const EAST: isize = 4;

// Outputs
const WALL: isize = 0;
const MOVE: isize = 1;
const OXYGEN: isize = 2;

enum MoveResult {
    WALL,
    MOVE(RepairDroid),
    OXYGEN(RepairDroid),
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point{ x: x, y: y, }
    }
}

struct RepairDroid {
    proc: Processor,
    res: Resources,
    moves: u32,
    point: Point,
}
impl RepairDroid {
    pub fn new(mem: Vec<isize>) -> Self {
        let mut droid = RepairDroid{
            proc: Processor::new_intcode(),
            res: Resources::new(mem),
            moves: 0,
            point: Point::new(0, 0),
        };
        droid.proc.execute(&mut droid.res);
        droid
    }
    fn from_move(&self, dir: isize, new_res: Resources) -> Self {
        RepairDroid{
            proc: Processor::new_intcode(),
            res: new_res,
            moves: self.moves + 1,
            point: match dir {
                NORTH => Point::new(self.point.x, self.point.y-1),
                SOUTH => Point::new(self.point.x, self.point.y+1),
                WEST => Point::new(self.point.x-1, self.point.y),
                EAST => Point::new(self.point.x+1, self.point.y),
                _ => panic!("bad direction"),
            },
        }
    }
    pub fn moves(&self) -> u32 {
        self.moves
    }
    pub fn point(&self) -> &Point {
        &self.point
    }
    pub fn move_in_dir(&self, dir: isize) -> MoveResult {
        let mut new_res = self.res.clone();
        new_res.write_input(dir);
        self.proc.resume(&mut new_res);
        match new_res.read_output().expect("no output") {
            WALL => MoveResult::WALL,
            MOVE => MoveResult::MOVE(self.from_move(dir, new_res)),
            OXYGEN => MoveResult::OXYGEN(self.from_move(dir, new_res)),
            _ => panic!("bad intcode program result"),
        }
    }
    pub fn reset_loc(&mut self) {
        self.point = Point::new(0, 0);
        self.moves = 0;
    }
}

fn bfs_find_oxygen(d: RepairDroid) -> Option<RepairDroid> {
    let mut nodes_queue = VecDeque::new();
    nodes_queue.push_back(d);
    let mut points_seen: HashSet<Point> = HashSet::new();
    while !nodes_queue.is_empty() {
        let droid = nodes_queue.pop_front().unwrap();
        if !points_seen.contains(droid.point()) {
            points_seen.insert(droid.point().clone());
            for &dir in [NORTH, SOUTH, WEST, EAST].iter() {
                match droid.move_in_dir(dir) {
                    MoveResult::WALL => (),
                    MoveResult::MOVE(new_droid) => {
                        nodes_queue.push_back(new_droid);
                    },
                    MoveResult::OXYGEN(new_droid) => {
                        return Some(new_droid);
                    }
                }
            }
        }
    }
    None
}

fn bfs_farthest_distance(d: RepairDroid) -> u32 {
    let mut max_dist = 0;
    let mut nodes_queue = VecDeque::new();
    nodes_queue.push_back(d);
    let mut points_seen: HashSet<Point> = HashSet::new();
    while !nodes_queue.is_empty() {
        let droid = nodes_queue.pop_front().unwrap();
        if !points_seen.contains(droid.point()) {
            points_seen.insert(droid.point().clone());
            max_dist = if max_dist < droid.moves() { droid.moves() } else { max_dist };
            for &dir in [NORTH, SOUTH, WEST, EAST].iter() {
                match droid.move_in_dir(dir) {
                    MoveResult::WALL => (),
                    MoveResult::MOVE(new_droid) | MoveResult::OXYGEN(new_droid) => {
                        nodes_queue.push_back(new_droid);
                    },
                }
            }
        }
    }
    max_dist
}

pub fn min_distance_to_oxygen(mem: Vec<isize>) -> Option<u32> {
    if let Some(bot) = bfs_find_oxygen(RepairDroid::new(mem)) {
        Some(bot.moves())
    } else {
        None
    }
}

pub fn max_time_to_oxygenation(mem: Vec<isize>) -> Option<u32> {
    if let Some(mut oxy_droid) = bfs_find_oxygen(RepairDroid::new(mem)) {
        oxy_droid.reset_loc();
        Some(bfs_farthest_distance(oxy_droid))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::RepairDroid;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
