use std::f64;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BTreeMap, HashMap};
use std::fmt;
use std::io::BufRead;

use num::integer::gcd;
use itertools::Itertools;

fn float_cmp(a: &f64, b: &f64) -> Ordering {
    // A very paranoid choice of epsilon
    const EPSILON: f64 = 1e-5;
    let diff = a - b;
    if diff.abs() < EPSILON {
        Ordering::Equal
    } else if diff < 0.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
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
    pub fn dist_manhattan(&self, rhs: &Point) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Eq, PartialEq, Debug)]
struct ClockPoint(Point);
impl ClockPoint {
    pub fn new_slope_vector(origin: &Point, p: &Point) -> ClockPoint {
        ClockPoint(Point::new_slope_vector(origin, p))
    }
    // Treating this point as a slope vector, what is the angle it forms with
    // the positive Y axis as zero, increasing clockwise, in range [0, 2*pi)?
    pub fn clock_angle(&self) -> f64 {
        // This full-circle arctan returns ranges from (-pi, pi), with zero
        // on the x-axis, counterclockwise as the positive direction, and pi
        // instead of -pi at the opposition
        // Also the y-axis is positive downwards
        let (xf, yf) = (self.0.x as f64, self.0.y as f64);
        let orig_angle = yf.atan2(xf);
        (orig_angle + f64::consts::FRAC_PI_2)
            .rem_euclid(2.0*f64::consts::PI)
    }
}
impl Ord for ClockPoint {
    fn cmp(&self, rhs: &Self) -> Ordering {
        float_cmp(&self.clock_angle(), &rhs.clock_angle())
    }
}
impl PartialOrd for ClockPoint {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}

pub fn points_from_data<T: BufRead>(data: T) -> BTreeSet<Point> {
    let mut asteriods = BTreeSet::new();
    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.expect("line read failed").trim().chars().enumerate() {
            if ch == '#' {
                asteriods.insert(Point::new(x as i32, y as i32));
            }
        }
    }
    asteriods
}

pub fn asteriod_visibility(asteriods: &BTreeSet<Point>) -> HashMap<&Point, Vec<Point>> {
    // Highest visibility asteriod has most unique slope vectors
    let mut all_visibility = Vec::new();
    for p1 in asteriods {
        let mut p1_visibility = Vec::new();
        for p2 in asteriods {
            if p1 != p2 {
                p1_visibility.push(Point::new_slope_vector(&p1, &p2));
            }
        }
        all_visibility.push((p1, p1_visibility));
    }
    all_visibility.into_iter().collect()
}

pub fn max_asteriod_visibility(asteriods: &BTreeSet<Point>) -> (Point, i32) {
    asteriod_visibility(asteriods)
        .iter()
        .map(|(p, vis)| (*p.clone(), vis.into_iter().unique().count() as i32))
        .max_by(|(_p1, nv1), (_p2, nv2)| nv1.cmp(&nv2))
        .expect("no asteriods")
}

pub fn laser_blast_order<'a>(origin: &Point, other_asteriods: &'a BTreeSet<Point>, seek_index: usize) -> Option<&'a Point> {
    // Map asteriods to (slope vector, asteriod) tuples
    let asteriods_by_svs: Vec<_> = other_asteriods.iter()
        .map(|ast| (ClockPoint::new_slope_vector(origin, &ast), ast))
        .collect();

    // Build a map, ordered by clock angle, of slope vectors to vecs
    // of points with that slope vector (e.g. aligned asteriods)
    let mut ast_map = BTreeMap::new();
    for (sv, ast) in asteriods_by_svs {
        ast_map.entry(sv).or_insert(Vec::new()).push(ast);
    }

    // Sort vecs of aligned asts by distance to the origin
    for aligned_asts in ast_map.values_mut() {
        aligned_asts.sort_by(
            // Farthest to closest so we can pop() the closest ones
            |a, b| b.dist_manhattan(origin).cmp(&a.dist_manhattan(origin)));
    }

    // Scan the map vertically to iterate over the asteriods in blast order
    let mut done = true;
    let mut blast_index = 1;
    loop {
        for aligned_asts in ast_map.values_mut() {
            let cur_ast = aligned_asts.pop();
            match cur_ast {
                Some(ast) => {
                    if blast_index == seek_index {
                        return Some(ast)
                    }
                    blast_index += 1;
                    done = false;
                },
                None => {},
            }
        }
        if done {
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::f64;
    use std::collections::BTreeSet;
    use super::{ClockPoint, Point, max_asteriod_visibility, points_from_data, laser_blast_order};

    const EXAMPLE1: &[u8] =
      b"......#.#.\n\
        #..#.#....\n\
        ..#######.\n\
        .#.#.###..\n\
        .#..#.....\n\
        ..#....#.#\n\
        #..#....#.\n\
        .##.#..###\n\
        ##...#..#.\n\
        .#....####" as &[u8];

    const EXAMPLE2: &[u8] =
      b"#.#...#.#.\n\
        .###....#.\n\
        .#....#...\n\
        ##.#.#.#.#\n\
        ....#.#.#.\n\
        .##..###.#\n\
        ..#...##..\n\
        ..##....##\n\
        ......#...\n\
        .####.###." as &[u8];

    const EXAMPLE3: &[u8] =
      b".#..#..###\n\
        ####.###.#\n\
        ....###.#.\n\
        ..###.##.#\n\
        ##.##.#.#.\n\
        ....###..#\n\
        ..#.#..#.#\n\
        #..#.#.###\n\
        .##...##.#\n\
        .....#.#.." as &[u8];

    const BIG_EXAMPLE: &[u8] =
      b".#..##.###...#######\n\
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
        ###.##.####.##.#..##" as &[u8];

    #[test]
    fn provided_examples_part_1() {
        let (pt1, max1) = max_asteriod_visibility(&points_from_data(EXAMPLE1));
        let (pt2, max2) = max_asteriod_visibility(&points_from_data(EXAMPLE2));
        let (pt3, max3) = max_asteriod_visibility(&points_from_data(EXAMPLE3));
        let (bpt, bmax) = max_asteriod_visibility(&points_from_data(BIG_EXAMPLE ));
        assert_eq!(Point::new(5, 8), pt1);
        assert_eq!(Point::new(1, 2), pt2);
        assert_eq!(Point::new(6, 3), pt3);
        assert_eq!(Point::new(11, 13), bpt);
        assert_eq!(33, max1);
        assert_eq!(35, max2);
        assert_eq!(41, max3);
        assert_eq!(210, bmax);
    }

    fn assert_between(a: f64, b: f64, c: f64) {
        assert!(a < b);
        assert!(b < c);
    }

    pub fn new_cp(x: i32, y: i32) -> ClockPoint {
        ClockPoint(Point::new(x, y))
    }

    #[test]
    fn clock_angle_within_range() {
        assert_between(
            0.0 * f64::consts::FRAC_PI_4,
            ClockPoint::new(1, -7).clock_angle(),
            1.0 * f64::consts::FRAC_PI_4,
        );
        assert_between(
            1.0 * f64::consts::FRAC_PI_4,
            ClockPoint::new(7, -1).clock_angle(),
            2.0 * f64::consts::FRAC_PI_4,
        );
        assert_between(
            2.0 * f64::consts::FRAC_PI_4,
            ClockPoint::new(7, 1).clock_angle(),
            3.0 * f64::consts::FRAC_PI_4,
        );
        assert_between(
            3.0 * f64::consts::FRAC_PI_4,
            ClockPoint::new(1, 7).clock_angle(),
            4.0 * f64::consts::FRAC_PI_4,
        );
        assert_between(
            4.0 * f64::consts::FRAC_PI_4,
            ClockPoint::new(-1, 7).clock_angle(),
            5.0 * f64::consts::FRAC_PI_4,
        );
        assert_between(
            5.0 * f64::consts::FRAC_PI_4,
            ClockPoint::new(-7, 1).clock_angle(),
            6.0 * f64::consts::FRAC_PI_4,
        );
        assert_between(
            6.0 * f64::consts::FRAC_PI_4,
            ClockPoint::new(-7, -1).clock_angle(),
            7.0 * f64::consts::FRAC_PI_4,
        );
        assert_between(
            7.0 * f64::consts::FRAC_PI_4,
            ClockPoint::new(-1, -7).clock_angle(),
            8.0 * f64::consts::FRAC_PI_4,
        );
        assert_eq!(0.0, ClockPoint::new(0, -1).clock_angle());
    }

    #[test]
    fn check_dist_manhattan() {
        assert_eq!(2, Point::new(1, 1).dist_manhattan(&Point::new(2, 0)));
        assert_eq!(7, Point::new(-3, 0).dist_manhattan(&Point::new(0, 4)));
    }

    #[test]
    fn check_blast_order() {
        let mut asteriods = points_from_data(BIG_EXAMPLE);
        let origin = Point::new(11, 13);
        asteriods.remove(&origin);
        assert_eq!(
            Some(&Point::new(11, 12)),
            laser_blast_order(&origin, &asteriods, 1));
        assert_eq!(
            Some(&Point::new(12, 1)),
            laser_blast_order(&origin, &asteriods, 2));
        assert_eq!(
            Some(&Point::new(12, 2)),
            laser_blast_order(&origin, &asteriods, 3));
        assert_eq!(
            Some(&Point::new(12, 8)),
            laser_blast_order(&origin, &asteriods, 10));
        assert_eq!(
            Some(&Point::new(16, 0)),
            laser_blast_order(&origin, &asteriods, 20));
        assert_eq!(
            Some(&Point::new(16, 9)),
            laser_blast_order(&origin, &asteriods, 50));
        assert_eq!(
            Some(&Point::new(10, 16)),
            laser_blast_order(&origin, &asteriods, 100));
        assert_eq!(
            Some(&Point::new(8, 2)),
            laser_blast_order(&origin, &asteriods, 200));
    }
}
