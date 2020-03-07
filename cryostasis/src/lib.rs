use std::io::{self, Read};

mod intcode;
use intcode::{Processor, Resources, Status};

pub fn run_droid(mem: &Vec<isize>) -> io::Result<()> {
    let proc = Processor::new_intcode();
    let mut res = Resources::new(mem.clone());
    proc.execute(&mut res);
    while res.get_status() != Status::TERMINATED {
        proc.resume(&mut res);
        print!("{}", res.read_output_line());
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        res.write_input_line(&buffer);
        proc.resume(&mut res);
    }
    println!("{}", res.read_output_line());
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
