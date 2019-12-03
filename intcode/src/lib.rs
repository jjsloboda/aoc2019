
pub fn execute(mem: &mut Vec<i32>) -> i32 {
    let mut cursor = 0;
    loop {
        if mem[cursor] == 99 {
            return mem[0];
        }
        let loc1 = mem[cursor+1] as usize;
        let loc2 = mem[cursor+2] as usize;
        let loc3 = mem[cursor+3] as usize;
        match mem[cursor] {
            1 => mem[loc3] = mem[loc1] + mem[loc2],
            2 => mem[loc3] = mem[loc1] * mem[loc2],
            _ => panic!("something went wrong"),
        };
        cursor += 4;
    }
}

pub fn run_with_inputs(noun: i32, verb: i32, prgm: &Vec<i32>) -> i32 {
    let mut mem = prgm.clone();
    mem[1] = noun; mem[2] = verb;
    execute(&mut mem)
}

#[cfg(test)]
mod tests {
    use super::{execute, run_with_inputs};

    #[test]
    fn first_examples() {
        assert_eq!(2, execute(&mut vec![1,0,0,0,99]));
        assert_eq!(30, execute(&mut vec![1,1,1,4,99,5,6,0,99]));
        assert_eq!(3500, execute(&mut vec![1,9,10,3,2,3,11,0,99,30,40,50]));
    }

    #[test]
    fn try_inputs() {
        use std::fs::read_to_string;
        let input = read_to_string("input.txt").unwrap();
        let mem: Vec<i32> = input.trim().split(',').map(
            |x| x.parse::<i32>().unwrap()).collect();

        assert_eq!(10566835, run_with_inputs(12, 2, &mem));
    }
}
