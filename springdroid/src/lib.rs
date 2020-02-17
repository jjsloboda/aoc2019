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
    fn write_line(s: &String, res: &mut Resources) {
        for ch in s.chars() {
            res.write_input(ch as isize);
        }
        res.write_input(10);
    }
    pub fn run_program(&self, prgm: &Vec<String>) -> Result<isize, String> {
        let mut res = Resources::new(self.mem.clone());
        self.proc.execute(&mut res);
        for line in prgm.iter() {
            Self::write_line(line, &mut res);
        }
        Self::write_line(&String::from("WALK"), &mut res);
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
}

fn run_and_dump(mem: &Vec<isize>, prgm: &Vec<String>) {
    let ssi = SpringScriptInterpreter::new(mem);
    match ssi.run_program(prgm) {
        Ok(v) => println!("success, hull damage is: {}", v),
        Err(e) => println!("failed, output:\n{}", e),
    }
}

fn run_with_null_prgm(mem: &Vec<isize>) {
    run_and_dump(mem, &vec![]);
}

pub fn run_with_first_prgm(mem: &Vec<isize>) {
    run_and_dump(mem, &vec![
        String::from("NOT A T"),
        String::from("NOT B J"),
        String::from( "OR J T"),
        String::from("NOT C J"),
        String::from( "OR J T"),
        String::from("NOT D J"),
        String::from("NOT J J"),
        String::from("AND T J"),
    ]);
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
