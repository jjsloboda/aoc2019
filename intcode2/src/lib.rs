use std::collections::HashMap;
use std::io::{self, Write};

enum Mode {
    POSITION,
    IMMEDIATE,
}

struct Parameter {
    value: isize,
    mode: Mode,
}
impl Parameter {
    pub fn val(&self, mem: &Vec<isize>) -> isize {
        match self.mode {
            Mode::POSITION => mem[self.value as usize],
            Mode::IMMEDIATE => self.value,
        }
    }
}

struct Instruction {
    opcode: isize,
    num_params: usize,
    exec_fn: fn(&mut Vec<isize>, &Vec<Parameter>),
}
impl Instruction {
    fn get_params(&self, mem: &Vec<isize>, cursor: usize) -> Vec<Parameter> {
        let mut params: Vec<Parameter> = Vec::new();
        let mut modes = mem[cursor] / 100;
        for i in 1..=self.num_params as usize {
            params.push(Parameter{
                value: mem[cursor+i],
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
    pub fn execute(&self, mem: &mut Vec<isize>, cursor: usize) {
        let params = self.get_params(mem, cursor);
        (self.exec_fn)(mem, &params);
    }
}

fn input_int(mem: &mut Vec<isize>, params: &Vec<Parameter>) {
    print!("input: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");
    let input_num = input.trim().parse::<isize>().expect("parse fail");
    mem[params[0].value as usize] = input_num;
}

const INTCODE: [Instruction; 4] = [
    Instruction{
        opcode: 1,
        num_params: 3,
        exec_fn: |mem, params| {
            mem[params[2].value as usize] = params[0].val(&mem) + params[1].val(&mem);
        },
    },
    Instruction{
        opcode: 2,
        num_params: 3,
        exec_fn: |mem, params| {
            mem[params[2].value as usize] = params[0].val(&mem) * params[1].val(&mem);
        },
    },
    Instruction{
        opcode: 3,
        num_params: 1,
        exec_fn: input_int,
    },
    Instruction{
        opcode: 4,
        num_params: 1,
        exec_fn: |mem, params| {
            println!("output: {}", params[0].val(&mem));
        },
    },
];

pub struct InstructionSet {
    // TODO should derive Copy trait on instruction instead of static borrowing
    insts: HashMap<isize, &'static Instruction>,
}
impl InstructionSet {
    fn new(insts: &'static [Instruction]) -> InstructionSet {
        let mut inst_map = HashMap::with_capacity(insts.len());
        for inst in insts {
            inst_map.insert(inst.opcode, inst);
        }
        InstructionSet{ insts: inst_map }
    }

    pub fn new_intcode() -> InstructionSet {
        Self::new(&INTCODE)
    }

    pub fn execute(&self, mem: &mut Vec<isize>) -> isize {
        let mut cursor = 0;
        loop {
            let opcode = mem[cursor] % 100;
            if opcode == 99 {
                return mem[0];
            }
            let inst = self.insts.get(&opcode).expect("instruction not found");
            inst.execute(mem, cursor);
            cursor += inst.num_params + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InstructionSet;

    #[test]
    fn old_examples() {
        let processor = InstructionSet::new_intcode();
        assert_eq!(2, processor.execute(&mut vec![1,0,0,0,99]));
        assert_eq!(30, processor.execute(&mut vec![1,1,1,4,99,5,6,0,99]));
        assert_eq!(3500, processor.execute(&mut vec![1,9,10,3,2,3,11,0,99,30,40,50]));
    }

    #[test]
    fn new_examples() {
        let processor = InstructionSet::new_intcode();
        assert_eq!(1002, processor.execute(&mut vec![1002,4,3,4,33]));
        assert_eq!(1101, processor.execute(&mut vec![1101,100,-1,4,0]));
    }
}
