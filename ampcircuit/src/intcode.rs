use std::collections::{HashMap, VecDeque};

pub struct Resources {
    mem: Vec<isize>,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
}
impl Resources {
    pub fn new(mem: Vec<isize>) -> Resources {
        Resources{
            mem: mem,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
    pub fn read_input(&mut self) -> isize{
        self.input.pop_front().expect("input underflow")
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
    fn get_params(&self, res: &Resources, cursor: isize) -> Vec<Parameter> {
        let mut params: Vec<Parameter> = Vec::new();
        let mut modes = res.read_mem(cursor) / 100;
        for i in 1..=self.num_params {
            params.push(Parameter{
                value: res.read_mem(cursor+i),
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
    pub fn execute(&self, res: &mut Resources, cursor: isize) -> isize {
        let params = self.get_params(res, cursor);
        match (self.exec_fn)(res, &params) {
            Some(x) => x,
            None => cursor + self.num_params + 1,
        }
    }
}

const INTCODE: [Instruction; 8] = [
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
            res.write_mem(params[0].val(&res), input);
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

    pub fn execute(&self, res: &mut Resources) -> isize {
        let mut cursor = 0;
        loop {
            let opcode = res.read_mem(cursor) % 100;
            if opcode == 99 {
                return res.read_mem(0);
            }
            let inst = self.insts.get(&opcode).expect("instruction not found");
            cursor = inst.execute(res, cursor);
        }
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
