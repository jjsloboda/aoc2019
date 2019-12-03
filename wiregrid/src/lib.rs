use std::collections::HashSet;
use std::cmp::{PartialEq, Eq};
use std::hash::Hash;

pub struct Wire {
    pts: HashSet<Point>,
}

impl Wire {
    pub fn new(path: String) -> Wire {
        let mut p: HashSet<Point> = HashSet::new();
        let directions = path.split(','); //.map(
            //|x| x.parse::<i32>().unwrap()).collect();
        for d in directions {
            let n = d[1..].parse::<i32>().unwrap(); 
            let get_next_pt = match d[0] {
                'U' => |&pt| pt.newUp(),
                'D' => |&pt| pt.newDown(),
                'L' => |&pt| pt.newLeft(),
                'R' => |&pt| pt.newRight(),
                _ => panic!("unsound input"),
            }
        }
        let w = Wire{ pts: p };
        return w;
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x: x, y: y}
    }

    fn newUp(&self) -> Point {
        Self::new(self.x, self.y+1)
    }
    fn newDown(&self) -> Point {
        Self::new(self.x, self.y-1)
    }
    fn newLeft(&self) -> Point {
        Self::new(self.x-1, self.y)
    }
    fn newRight(&self) -> Point {
        Self::new(self.x+1, self.y)
    }
}

pub fn blah() {
    println!("borto");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
