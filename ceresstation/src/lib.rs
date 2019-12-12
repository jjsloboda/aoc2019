use std::io::BufRead;

use num::integer::gcd;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point{ x: x, y: y }
    }
    pub fn new_slope_vector(origin: &Point, p: &Point) -> Point {
        let orig_x = p.x - origin.x;
        let orig_y = p.y - origin.y;
        let d = gcd(orig_x, orig_y);
        Point{ x: orig_x / d, y: orig_y / d }
    }
}

pub fn points_from_data<T: BufRead>(data: T) -> Vec<Point> {
    let mut asteriods = Vec::new();
    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.expect("line read failed").trim().chars().enumerate() {
            if ch == '#' {
                asteriods.push(Point::new(x as i32, y as i32));
            }
        }
    }
    asteriods
}

pub fn max_asteriod_visibility(asteriods: &Vec<Point>) -> i32 {
    // Highest visibility asteriod has most unique slope vectors
    let mut all_visibility = Vec::new();
    for p1 in asteriods {
        let mut p1_visibility = Vec::new();
        for p2 in asteriods {
            if p1 != p2 {
                p1_visibility.push(Point::new_slope_vector(&p1, &p2));
            }
        }
        all_visibility.push(p1_visibility);
    }
    all_visibility.iter()
        .map(|x| x.into_iter().unique().count() as i32)
        .max()
        .expect("no visibility")
}

#[cfg(test)]
mod tests {
    use super::{max_asteriod_visibility, points_from_data};

    #[test]
    fn provided_examples_part_1() {
        let example1 = b"\n\
        ......#.#.\n\
        #..#.#....\n\
        ..#######.\n\
        .#.#.###..\n\
        .#..#.....\n\
        ..#....#.#\n\
        #..#....#.\n\
        .##.#..###\n\
        ##...#..#.\n\
        .#....####\n\
        " as &[u8];
        assert_eq!(33, max_asteriod_visibility(&points_from_data(example1)));

        let example2 = b"\n\
        #.#...#.#.\n\
        .###....#.\n\
        .#....#...\n\
        ##.#.#.#.#\n\
        ....#.#.#.\n\
        .##..###.#\n\
        ..#...##..\n\
        ..##....##\n\
        ......#...\n\
        .####.###.\n\
        " as &[u8];
        assert_eq!(35, max_asteriod_visibility(&points_from_data(example2)));

        let example3 = b"\n\
        .#..#..###\n\
        ####.###.#\n\
        ....###.#.\n\
        ..###.##.#\n\
        ##.##.#.#.\n\
        ....###..#\n\
        ..#.#..#.#\n\
        #..#.#.###\n\
        .##...##.#\n\
        .....#.#..\n\
        " as &[u8];
        assert_eq!(41, max_asteriod_visibility(&points_from_data(example3)));

        let example4 = b"\n\
        .#..##.###...#######\n\
        ##.############..##.\n\
        .#.######.########.#\n\
        .###.#######.####.#.\n\
        #####.##.#.##.###.##\n\
        ..#####..#.#########\n\
        ####################\n\
        #.####....###.#.#.##\n\
        ##.#################\n\
        #####.##.###..####..\n\
        ..######..##.#######\n\
        ####.##.####...##..#\n\
        .#####..#.######.###\n\
        ##...#.##########...\n\
        #.##########.#######\n\
        .####.#.###.###.#.##\n\
        ....##.##.###..#####\n\
        .#.#.###########.###\n\
        #.#.#.#####.####.###\n\
        ###.##.####.##.#..##\n\
        " as &[u8];
        assert_eq!(210, max_asteriod_visibility(&points_from_data(example4)));
    }
}
