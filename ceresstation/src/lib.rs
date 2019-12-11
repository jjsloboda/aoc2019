use num::integer::gcd;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
