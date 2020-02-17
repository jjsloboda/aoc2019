
mod intcode;
use intcode::{Processor, Resources, Status};

pub fn scan_immediate_area(mem: &Vec<isize>) -> u64 {
    let mut i = 0;
    let proc = Processor::new_intcode();
    for y in 0..50 {
        for x in 0..50 {
            let mut res = Resources::new(mem.clone());
            proc.execute(&mut res);
            res.write_input(x);
            res.write_input(y);
            proc.resume(&mut res);
            i += res.read_output().expect("expected output got none");
        }
    }
    i as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
