

mod intcode;
use intcode::{Processor, Resources};

struct Network {
    nodes: Vec<Resources>,
    proc: Processor,
}
impl Network {
    pub fn new(mem: &Vec<isize>) -> Self {
        let proc = Processor::new_intcode();
        let mut nodes = Vec::new();
        for i in 0..50 {
            let mut res = Resources::new(mem.clone());
            res.write_input(i);
            proc.execute(&mut res);
            nodes.push(res);
        }
        Network{ nodes: nodes, proc: proc, }
    }
    fn read_packet(&mut self, i: usize) -> Option<(isize, isize, isize)> {
        let res = &mut self.nodes[i];
        let dst = res.read_output()?;
        let x = res.read_output()?;
        let y = res.read_output()?;
        Some((dst, x, y))
    }
    fn send_packet(&mut self, dst: isize, x: isize, y: isize) {
        let res = &mut self.nodes[dst as usize];
        res.write_input(x);
        res.write_input(y);
    }
    pub fn loop_once(&mut self) -> Result<(), (isize, isize, isize)> {
        for i in 0..self.nodes.len() {
            self.nodes[i].write_input(-1);
            self.proc.resume(&mut self.nodes[i]);
            while let Some((dst, x, y)) = self.read_packet(i) {
                if dst >= 50 {
                    return Err((dst, x, y));
                }
                self.send_packet(dst, x, y);
            }
        }
        Ok(())
    }
    pub fn loop_until_err(&mut self) -> Result<(), (isize, isize, isize)> {
        loop {
            self.loop_once()?;
        }
    }
}

pub fn y_val_of_255_packet(mem: &Vec<isize>) -> isize {
    let mut net = Network::new(&mem);
    if let Err((_, _, y)) = net.loop_until_err() {
        y
    } else {
        panic!("expected err");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
