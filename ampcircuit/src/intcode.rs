use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Status {
    READY,
    RUNNING,
    SUSPENDED,
    TERMINATED,
}

#[derive(Clone)]
pub struct Resources {
    cursor: usize,
    status: Status,
    mem: Vec<isize>,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
}
impl Resources {
    pub fn new(mem: Vec<isize>) -> Resources {
        Resources{
            cursor: 0,
            status: Status::READY,
            mem: mem,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
    pub fn get_status(&self) -> Status {
        self.status
    }
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
    pub fn set_cursor(&mut self, loc: isize) {
        self.cursor = loc as usize;
    }
    pub fn inc_cursor(&mut self, offset: isize) {
        self.cursor += offset as usize;
    }
    pub fn read_mem_offset(&self, offset: isize) -> isize {
        self.mem[self.cursor + offset as usize]
    }
    pub fn read_input(&mut self) -> isize {
        match self.input.pop_front() {
            Some(x) => x,
            None => {
                self.set_status(Status::SUSPENDED);
                0
            },
        }
    }
    pub fn write_input(&mut self, i: isize) {
        self.input.push_back(i);
    }
    pub fn read_output(&mut self) -> isize {
        self.output.pop_front().expect("output underflow")
    }
    pub fn write_output(&mut self, i: isize) {
        self.output.push_back(i);
    }
    pub fn read_mem(&self, loc: isize) -> isize {
        self.mem[loc as usize]
    }
    pub fn write_mem(&mut self, loc: isize, val: isize) {
        self.mem[loc as usize] = val;
    }
}

enum Mode {
    POSITION,
    IMMEDIATE,
}

struct Parameter {
    value: isize,
    mode: Mode,
}
impl Parameter {
    pub fn val(&self, res: &Resources) -> isize {
        match self.mode {
            Mode::POSITION => res.read_mem(self.value),
            Mode::IMMEDIATE => self.value,
        }
    }
}

struct Instruction {
    opcode: isize,
    num_params: isize,
    exec_fn: fn(&mut Resources, &Vec<Parameter>) -> Option<isize>,
}
impl Instruction {
    fn get_params(&self, res: &Resources) -> Vec<Parameter> {
        let mut params: Vec<Parameter> = Vec::new();
        let mut modes = res.read_mem_offset(0) / 100;
        for i in 1..=self.num_params {
            params.push(Parameter{
                value: res.read_mem_offset(i),
                mode: match modes & 0x1 {
                    0 => Mode::POSITION,
                    1 => Mode::IMMEDIATE,
                    _ => panic!("bad parameter mode"),
                },
            });
            modes /= 10;
        }
        params
    }
    pub fn execute(&self, res: &mut Resources) {
        let params = self.get_params(res);
        let loc = (self.exec_fn)(res, &params);
        if res.get_status() == Status::RUNNING {
            match loc {
                Some(x) => res.set_cursor(x),
                None => res.inc_cursor(self.num_params + 1),
            }
        }
    }
}

const INTCODE: [Instruction; 9] = [
    Instruction{
        opcode: 1,
        num_params: 3,
        exec_fn: |res, params| {
            res.write_mem(params[2].value, params[0].val(&res) + params[1].val(&res));
            None
        },
    },
    Instruction{
        opcode: 2,
        num_params: 3,
        exec_fn: |res, params| {
            res.write_mem(params[2].value, params[0].val(&res) * params[1].val(&res));
            None
        },
    },
    Instruction{
        opcode: 3,
        num_params: 1,
        exec_fn: |res, params| {
            let input = res.read_input();
            res.write_mem(params[0].value, input);
            None
        },
    },
    Instruction{
        opcode: 4,
        num_params: 1,
        exec_fn: |res, params| {
            let output = params[0].val(&res);
            res.write_output(output);
            None
        },
    },
    Instruction{
        opcode: 5,
        num_params: 2,
        exec_fn: |res, params| {
            if params[0].val(&res) != 0 {
                Some(params[1].val(&res))
            } else {
                None
            }
        },
    },
    Instruction{
        opcode: 6,
        num_params: 2,
        exec_fn: |res, params| {
            if params[0].val(&res) == 0 {
                Some(params[1].val(&res))
            } else {
                None
            }
        },
    },
    Instruction{
        opcode: 7,
        num_params: 3,
        exec_fn: |res, params| {
            let value = if params[0].val(&res) < params[1].val(&res) { 1 } else { 0 };
            res.write_mem(params[2].value, value);
            None
        },
    },
    Instruction{
        opcode: 8,
        num_params: 3,
        exec_fn: |res, params| {
            let value = if params[0].val(&res) == params[1].val(&res) { 1 } else { 0 };
            res.write_mem(params[2].value, value);
            None
        },
    },
    Instruction{
        opcode: 99,
        num_params: 0,
        exec_fn: |res, _| {
            res.set_status(Status::TERMINATED);
            None
        },
    },
];

pub struct Processor {
    insts: HashMap<isize, &'static Instruction>,
}
impl Processor {
    fn new(insts: &'static [Instruction]) -> Processor {
        let mut inst_map = HashMap::with_capacity(insts.len());
        for inst in insts {
            inst_map.insert(inst.opcode, inst);
        }
        Processor{ insts: inst_map }
    }

    pub fn new_intcode() -> Processor {
        Self::new(&INTCODE)
    }

    fn run(&self, res: &mut Resources) {
        res.set_status(Status::RUNNING);
        while res.get_status() == Status::RUNNING {
            let opcode = res.read_mem_offset(0) % 100;
            let inst = self.insts.get(&opcode)
                .expect("instruction not found");
            inst.execute(res);
        }
    }

    pub fn execute(&self, res: &mut Resources) -> isize {
        if res.get_status() == Status::READY {
            self.run(res)
        }
        res.read_mem(0)
    }

    pub fn resume(&self, res: &mut Resources) -> isize {
        if res.get_status() == Status::SUSPENDED {
            self.run(res)
        }
        res.read_mem(0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Processor, Resources};

    #[test]
    fn old_examples() {
        let processor = Processor::new_intcode();
        assert_eq!(2, processor.execute(&mut Resources::new(vec![1,0,0,0,99])));
        assert_eq!(30, processor.execute(&mut Resources::new(vec![1,1,1,4,99,5,6,0,99])));
        assert_eq!(3500, processor.execute(&mut Resources::new(vec![1,9,10,3,2,3,11,0,99,30,40,50])));
    }

    #[test]
    fn new_examples() {
        let processor = Processor::new_intcode();
        assert_eq!(1002, processor.execute(&mut Resources::new(vec![1002,4,3,4,33])));
        assert_eq!(1101, processor.execute(&mut Resources::new(vec![1101,100,-1,4,0])));
    }
}
