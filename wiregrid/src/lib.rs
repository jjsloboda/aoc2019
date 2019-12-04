use std::collections::HashSet;
use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};

struct Point {
    x: i32,
    y: i32,
    s: i32,
}
impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}
impl Eq for Point {}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

pub struct Wire {
    pts: HashSet<Point>,
}

impl Wire {
    pub fn new(path: &str) -> Wire {
        let mut p: HashSet<Point> = HashSet::new();
        let directions = path.split(',');
        let (mut x, mut y, mut steps) = (0, 0, 0);
        for d in directions {
            let dir = &d[0..1];
            let n = d[1..].trim().parse::<i32>().expect("bad input"); 
            for _ in 0..n {
                match dir {
                    "U" => y += 1,
                    "D" => y -= 1,
                    "L" => x -= 1,
                    "R" => x += 1,
                    _ => panic!("unsound input"),
                }
                steps += 1;
                let point = Point{ x: x, y: y, s: steps };
                if !p.contains(&point) {
                    p.insert(point);
                }
            }
        }
        Wire{ pts: p }
    }
}

fn closest_intersection(w1: &Wire, w2: &Wire, dist_fn: &Fn(&Point) -> i32) -> i32 {
    w1.pts.intersection(&w2.pts)
        .map(dist_fn)
        .min()
        .expect("wires don't cross")
}

pub fn shortest_manhattan(w1: &Wire, w2: &Wire) -> i32 {
    closest_intersection(&w1, &w2,
        &|pt: &Point| pt.x.abs() + pt.y.abs())
}

pub fn shortest_wire_path(w1: &Wire, w2: &Wire) -> i32 {
    closest_intersection(&w1, &w2,
        &|pt: &Point| w1.pts.get(pt).unwrap().s + w2.pts.get(pt).unwrap().s)
}

#[cfg(test)]
mod tests {
    use super::{Wire, shortest_manhattan, shortest_wire_path};

    #[test]
    fn given_examples_part_one() {
        assert_eq!(6, shortest_manhattan(
            &Wire::new("R8,U5,L5,D3"),
            &Wire::new("U7,R6,D4,L4")));
        assert_eq!(159, shortest_manhattan(
            &Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &Wire::new("U62,R66,U55,R34,D71,R55,D58,R83")));
        assert_eq!(135, shortest_manhattan(
            &Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")));
    }

    #[test]
    fn given_examples_part_two() {
        assert_eq!(30, shortest_wire_path(
            &Wire::new("R8,U5,L5,D3"),
            &Wire::new("U7,R6,D4,L4")));
        assert_eq!(610, shortest_wire_path(
            &Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &Wire::new("U62,R66,U55,R34,D71,R55,D58,R83")));
        assert_eq!(410, shortest_wire_path(
            &Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")));
    }
}
