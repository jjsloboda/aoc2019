use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}
impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self{ x: x, y: y, z: z }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Body {
    pos: Point,
    vel: Point,
}
impl Body {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self{
            pos: Point::new(x, y, z),
            vel: Point::new(0, 0, 0),
        }
    }
    pub fn apply_gravity(&mut self, pt: &Point) {
        // x
        if self.pos.x > pt.x {
            self.vel.x -= 1;
        } else if self.pos.x < pt.x {
            self.vel.x += 1;
        }

        // y
        if self.pos.y > pt.y {
            self.vel.y -= 1;
        } else if self.pos.y < pt.y {
            self.vel.y += 1;
        }

        // z
        if self.pos.z > pt.z {
            self.vel.z -= 1;
        } else if self.pos.z < pt.z {
            self.vel.z += 1;
        }
    }
    pub fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
    pub fn energy(&self) -> i32 {
        let potential = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        let kinetic = self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();
        potential * kinetic
    }
    pub fn pos(&self) -> &Point {
        &self.pos
    }
}
impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}\n", self.pos, self.vel)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct System {
    bodies: Vec<Body>,
}
impl System {
    pub fn new(bodies: Vec<Body>) -> Self {
        System{ bodies: bodies }
    }
    pub fn body(&self, i: usize) -> &Body {
        &self.bodies[i]
    }
    pub fn step_forward(&mut self) {
        // apply gravity
        for i in 0..self.bodies.len()-1 {
            for j in i+1..self.bodies.len() {
                let pti = self.bodies[i].pos;
                let ptj = self.bodies[j].pos;
                self.bodies[i].apply_gravity(&ptj);
                self.bodies[j].apply_gravity(&pti);
            }
        }
        // apply velocity
        for b in &mut self.bodies {
            b.apply_velocity();
        }
    }
    pub fn energy(&self) -> i32 {
        self.bodies.iter().map(|x| x.energy()).sum()
    }
    pub fn pos_by_dim(&self) -> (Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
        let xs: Vec<_> = self.bodies.iter().map(|b| (b.pos.x, b.vel.x)).collect();
        let ys: Vec<_> = self.bodies.iter().map(|b| (b.pos.y, b.vel.y)).collect();
        let zs: Vec<_> = self.bodies.iter().map(|b| (b.pos.z, b.vel.z)).collect();
        (xs, ys, zs)
    }
}
impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.bodies {
            write!(f, "{}", b)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn system_step_works() {
        use super::{System, Body, Point};
        let mut system = System::new(vec![
            Body::new(-1, 0, 2),
            Body::new(2, -10, -7),
            Body::new(4, -8, 8),
            Body::new(3, 5, -1),
        ]);
        system.step_forward();

        assert_eq!(Point::new( 2, -1,  1), system.bodies[0].pos);
        assert_eq!(Point::new( 3, -7, -4), system.bodies[1].pos);
        assert_eq!(Point::new( 1, -7,  5), system.bodies[2].pos);
        assert_eq!(Point::new( 2,  2,  0), system.bodies[3].pos);

        assert_eq!(Point::new( 3, -1, -1), system.bodies[0].vel);
        assert_eq!(Point::new( 1,  3,  3), system.bodies[1].vel);
        assert_eq!(Point::new(-3,  1, -3), system.bodies[2].vel);
        assert_eq!(Point::new(-1, -3,  1), system.bodies[3].vel);
    }

    #[test]
    fn check_energy() {
        use super::{System, Body, Point};
        let mut system = System::new(vec![
            Body::new(-1, 0, 2),
            Body::new(2, -10, -7),
            Body::new(4, -8, 8),
            Body::new(3, 5, -1),
        ]);

        for _ in 0..10 { system.step_forward(); }
        assert_eq!(179, system.energy());
    }
}
