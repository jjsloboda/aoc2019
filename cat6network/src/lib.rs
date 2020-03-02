mod intcode;
use intcode::{Processor, Resources};

struct Network {
    nodes: Vec<Resources>,
    proc: Processor,
    nat_packet: (isize, isize),
    last_y_delivered: isize,
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
        Network{
            nodes: nodes, proc: proc, nat_packet: (0, 0),
            last_y_delivered: 0,
        }
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
    pub fn loop_once(&mut self) -> Result<(), isize> {
        let mut idle = true;
        for i in 0..self.nodes.len() {
            if self.nodes[i].input_len() == 0 {
                self.nodes[i].write_input(-1);
            } else {
                idle = false;
            }
            self.proc.resume(&mut self.nodes[i]);
            while let Some((dst, x, y)) = self.read_packet(i) {
                idle = false;
                if dst == 255 {
                    self.nat_packet = (x, y);
                } else {
                    self.send_packet(dst, x, y);
                }
            }
        }
        if idle {
            let (x, y) = self.nat_packet;
            if y == self.last_y_delivered {
                return Err(y);
            }
            self.send_packet(0, x, y);
            self.last_y_delivered = y;
        }
        Ok(())
    }
    pub fn loop_until_err(&mut self) -> Result<(), isize> {
        loop {
            self.loop_once()?;
        }
    }
}

pub fn y_val_of_err_packet(mem: &Vec<isize>) -> isize {
    let mut net = Network::new(&mem);
    if let Err(y) = net.loop_until_err() {
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
