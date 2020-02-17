mod intcode;
use intcode::{Processor, Resources};

struct SpringScriptInterpreter<'a> {
    mem: &'a Vec<isize>,
    proc: Processor,
}
impl<'a> SpringScriptInterpreter<'a> {
    pub fn new(mem: &'a Vec<isize>) -> Self {
        SpringScriptInterpreter{ mem: mem, proc: Processor::new_intcode() }
    }
    fn write_line(s: &str, res: &mut Resources) {
        for ch in s.chars() {
            res.write_input(ch as isize);
        }
        res.write_input(10);
    }
    pub fn exec(&self, prgm: &Vec<&str>, mode: &str) -> Result<isize, String> {
        let mut res = Resources::new(self.mem.clone());
        self.proc.execute(&mut res);
        for line in prgm.iter() {
            Self::write_line(line, &mut res);
        }
        Self::write_line(mode, &mut res);
        self.proc.resume(&mut res);

        let mut result = Vec::new();
        while let Some(out) = res.read_output() {
            result.push(out);
        }
        if *result.last().unwrap_or(&0) >= 256 {
            Ok(*result.last().unwrap())
        } else {
            Err(result.iter().map(|&i| i as u8 as char).collect())
        }
    }
    pub fn walk_program(&self, prgm: &Vec<&str>) -> Result<isize, String> {
        self.exec(prgm, "WALK")
    }
    pub fn run_program(&self, prgm: &Vec<&str>) -> Result<isize, String> {
        self.exec(prgm, "RUN")
    }
}

fn exec_and_dump(mem: &Vec<isize>, prgm: &Vec<&str>, mode: &str) {
    let ssi = SpringScriptInterpreter::new(mem);
    match ssi.exec(prgm, mode) {
        Ok(v) => println!("success, hull damage is: {}", v),
        Err(e) => println!("failed, output:\n{}", e),
    }
}

fn walk_with_null_prgm(mem: &Vec<isize>) {
    exec_and_dump(mem, &vec![], "WALK");
}

pub fn walk_with_first_prgm(mem: &Vec<isize>) {
    exec_and_dump(mem, &vec![
        "NOT A T",
        "NOT B J",
         "OR J T",
        "NOT C J",
         "OR J T",
        "NOT D J",
        "NOT J J",
        "AND T J",
    ], "WALK");
}

pub fn run_with_second_prgm(mem: &Vec<isize>) {
    exec_and_dump(mem, &vec![
        "NOT A T",
        "NOT B J",
         "OR J T",
        "NOT C J",
         "OR J T",
        "NOT D J",
        "NOT J J",
        "AND H J",
        "AND T J",
        "NOT A T",
         "OR T J",
    ], "RUN");
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_fall_in_hole() {
        use std::io;
        use std::fs::read_to_string;
        use super::run_with_null_prgm;
        let input = read_to_string("input.txt").unwrap();
        let mem: Vec<isize> = input.trim().split(',')
            .map(|x| x.parse::<isize>().expect("failed to parse input"))
            .collect();
        run_with_null_prgm(&mem);
    }
}
