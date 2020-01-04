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
    OXYGEN,
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
            MOVE => MoveResult::MOVE(RepairDroid{
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
            }),
            OXYGEN => MoveResult::OXYGEN,
            _ => panic!("bad intcode program result"),
        }
    }
}

pub fn min_distance_to_oxygen(mem: Vec<isize>) -> Option<u32> {
    // BFS
    let mut nodes_queue = VecDeque::new();
    let mut points_seen: HashSet<Point> = HashSet::new();
    nodes_queue.push_back(RepairDroid::new(mem));
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
                    MoveResult::OXYGEN => {
                        return Some(droid.moves() + 1);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::RepairDroid;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
