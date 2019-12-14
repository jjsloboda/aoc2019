use std::collections::HashMap;

mod intcode;
use intcode::{Processor, Resources};

type Color = isize;
const BLACK: Color = 0;
const WHITE: Color = 1;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point{ x: x, y: y, }
    }
}
struct Tile {
    c: Color,
    painted: bool,
}
impl Tile {
    pub fn new() -> Self {
        Tile{ c: BLACK, painted: false, }
    }
}

pub struct Hull {
    tiles: HashMap<Point, Tile>,
}
impl Hull {
    pub fn new() -> Self {
        Hull{ tiles: HashMap::new(), }
    }
    fn tile_at_loc(&mut self, pt: &Point) -> &mut Tile {
        self.tiles.entry(*pt).or_insert(Tile::new())
    }
    pub fn color_at_loc(&mut self, pt: &Point) -> Color {
        self.tile_at_loc(pt).c
    }
    pub fn update_color_at_loc(&mut self, pt: &Point, c: Color) {
        let mut tile = self.tile_at_loc(pt);
        tile.c = c;
        tile.painted = true;
    }
    pub fn num_tiles_painted(&self) -> i32 {
        self.tiles.values()
            .map(|t| if t.painted { 1 } else { 0 })
            .sum()
    }
}

const TURN_LEFT: isize = 0;
const TURN_RIGHT: isize = 1;
#[derive(Eq, PartialEq, Debug)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}
impl Direction {
    pub fn new_from_turn(&self, t: isize) -> Self {
        if t == TURN_LEFT {
            match self {
                Direction::LEFT => Direction::DOWN,
                Direction::UP => Direction::LEFT,
                Direction::RIGHT => Direction::UP,
                Direction::DOWN => Direction::RIGHT,
            }
        } else if t == TURN_RIGHT {
            match self {
                Direction::LEFT => Direction::UP,
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
            }
        } else {
            panic!("precondition failed, invalid turn value");
        }
    }
}

pub struct Robot {
    proc: Processor,
    res: Resources,
    loc: Point,
    dir: Direction,
}
impl Robot {
    pub fn new(mem: Vec<isize>) -> Self {
        let mut robot = Robot {
            proc: Processor::new_intcode(),
            res: Resources::new(mem),
            loc: Point::new(0, 0),
            dir: Direction::UP,
        };
        // Will suspend and return once blocked on input
        robot.proc.execute(&mut robot.res);
        robot
    }
    pub fn paint_panel_move_on(&mut self, cur_color: Color) -> Option<Color> {
        self.res.write_input(cur_color);
        self.proc.resume(&mut self.res);
        let painted_color = self.res.read_output();
        let direction_num = self.res.read_output()?;
        self.dir = self.dir.new_from_turn(direction_num);
        self.move_forward();
        painted_color
    }
    fn move_forward(&mut self) {
        match self.dir {
            Direction::LEFT => self.loc.x -= 1,
            Direction::UP => self.loc.y += 1,
            Direction::RIGHT => self.loc.x += 1,
            Direction::DOWN => self.loc.y -= 1,
        }
    }
}

pub fn paint_hull_with_robot(hull: &mut Hull, robot: &mut Robot) {
    loop {
        let initial_loc = robot.loc;
        let color_painted = robot.paint_panel_move_on(
            hull.color_at_loc(&robot.loc));
        match color_painted {
            Some(c) => {
                hull.update_color_at_loc(&initial_loc, c);
            },
            None => break,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_turn() {
        use super::{Direction, TURN_LEFT, TURN_RIGHT};
        let mut dir = Direction::LEFT;
        dir = dir.new_from_turn(TURN_LEFT);
        dir = dir.new_from_turn(TURN_LEFT);
        assert_eq!(Direction::RIGHT, dir);
        dir = dir.new_from_turn(TURN_LEFT);
        dir = dir.new_from_turn(TURN_LEFT);
        assert_eq!(Direction::LEFT, dir);
        dir = dir.new_from_turn(TURN_RIGHT);
        dir = dir.new_from_turn(TURN_RIGHT);
        assert_eq!(Direction::RIGHT, dir);
        dir = dir.new_from_turn(TURN_RIGHT);
        dir = dir.new_from_turn(TURN_RIGHT);
        assert_eq!(Direction::LEFT, dir);
    }

    #[test]
    fn robot_works() {
        use super::{Robot, WHITE};
        let mut robot = Robot::new(
            vec![3,100,4,100,4,101,3,100,4,100,4,101,99]);
        assert_eq!(Some(WHITE), robot.paint_panel_move_on(WHITE));
        assert_eq!(Some(WHITE), robot.paint_panel_move_on(WHITE));
        assert_eq!(None, robot.paint_panel_move_on(WHITE));
    }

    #[test]
    fn hull_works() {
        use super::{Hull, Point, BLACK, WHITE};
        let mut hull = Hull::new();
        assert_eq!(BLACK, hull.color_at_loc(&Point::new(3, 4)));
        hull.update_color_at_loc(&Point::new(3, 4), WHITE);
        assert_eq!(WHITE, hull.color_at_loc(&Point::new(3, 4)));
        hull.update_color_at_loc(&Point::new(3, 1), BLACK);
        assert_eq!(BLACK, hull.color_at_loc(&Point::new(3, 1)));
        assert_eq!(2, hull.num_tiles_painted());
    }

    #[test]
    fn paint_hull_with_robot_works() {
        use super::{paint_hull_with_robot, Robot, Hull, Point, BLACK};
        let mut robot = Robot::new(
            vec![3,100,4,100,4,101,3,100,4,100,4,101,99]);
        let mut hull = Hull::new();
        paint_hull_with_robot(&mut hull, &mut robot);
        assert_eq!(BLACK, hull.color_at_loc(&Point::new(0, 0)));
        assert_eq!(2, hull.num_tiles_painted());
    }
}
