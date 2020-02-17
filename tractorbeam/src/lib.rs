mod intcode;
use intcode::{Processor, Resources};

struct Beam<'a> {
    mem: &'a Vec<isize>,
    proc: Processor,
}
impl<'a> Beam<'a> {
    pub fn new(mem: &'a Vec<isize>) -> Self {
        Beam{ mem: mem, proc: Processor::new_intcode() }
    }
    pub fn check(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            false
        } else {
            let mut res = Resources::new(self.mem.clone());
            self.proc.execute(&mut res);
            res.write_input(x);
            res.write_input(y);
            self.proc.resume(&mut res);
            let out = res.read_output().expect("expected output got none");
            out == 1
        }
    }
}

pub fn scan_immediate_area(mem: &Vec<isize>) -> u64 {
    let mut i = 0;
    let beam = Beam::new(mem);
    for y in 0..50 {
        for x in 0..50 {
            if beam.check(x, y) {
                i += 1;
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    i as u64
}

fn upper_bound_next_point(beam: &Beam, x: isize, y: isize) -> (isize, isize) {
    let (mut nx, mut ny) = (x, y);
    nx += 1;
    while !beam.check(nx, ny) { ny += 1; }
    println!("beam top edge: ({}, {})", nx, ny);
    (nx, ny)
}

pub fn scan_for_ship_size(mem: &Vec<isize>) -> (isize, isize) {
    const N: isize = 100;
    let (mut x, mut y) = (6, 0);
    let beam = Beam::new(mem);
    loop {
        // Observed beam is above y = x line, so we just check upper bound
        let (tmpx, tmpy) = upper_bound_next_point(&beam, x, y);
        x = tmpx; y = tmpy;
        if beam.check(x - N + 1, y) && beam.check(x - N + 1, y + N - 1) {
            return (x - N + 1, y)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
