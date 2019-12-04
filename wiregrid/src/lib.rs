use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub struct Wire {
    pts: HashSet<Point>,
}

impl Wire {
    pub fn new(path: &str) -> Wire {
        let mut p: HashSet<Point> = HashSet::new();
        let directions = path.split(',');
        let (mut x, mut y) = (0, 0);
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
                p.insert(Point{ x: x, y: y });
            }
        }
        Wire{ pts: p }
    }
}

pub fn closest_intersection(w1: &Wire, w2: &Wire) -> i32 {
    w1.pts.intersection(&w2.pts)
        .map(|pt| pt.x.abs() + pt.y.abs())
        .min()
        .expect("wires don't cross")
}

#[cfg(test)]
mod tests {
    use super::{Wire, closest_intersection};

    #[test]
    fn given_examples_part_one() {
        assert_eq!(6, closest_intersection(
            &Wire::new("R8,U5,L5,D3"),
            &Wire::new("U7,R6,D4,L4")));
        assert_eq!(159, closest_intersection(
            &Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &Wire::new("U62,R66,U55,R34,D71,R55,D58,R83")));
        assert_eq!(135, closest_intersection(
            &Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")));
    }
}
