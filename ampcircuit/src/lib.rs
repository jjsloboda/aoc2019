pub mod intcode;
use intcode::{Processor, Resources};

pub fn run_phase_settings(proc: &Processor, pgrm: &Vec<isize>, settings: &Vec<usize>) -> isize {
    let mut signal = 0;
    for phase_setting in settings {
        let mut res = Resources::new(pgrm.clone());
        res.write_input(*phase_setting as isize);
        res.write_input(signal);
        println!("phase setting: {}, signal in: {}", *phase_setting, signal);
        proc.execute(&mut res);
        signal = res.read_output();
        println!("signal out: {}", signal);
    }
    signal
}

#[cfg(test)]
mod tests {
    use super::{run_phase_settings, Processor};

    #[test]
    fn example_program_1() {
        let proc = Processor::new_intcode();
        assert_eq!(43210, run_phase_settings(&proc, 
            &vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], &vec![4,3,2,1,0]));
    }

    #[test]
    fn example_program_2() {
        let proc = Processor::new_intcode();
        assert_eq!(54321, run_phase_settings(&proc, 
            &vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0],
            &vec![0,1,2,3,4]));
    }

    #[test]
    fn example_program_3() {
        let proc = Processor::new_intcode();
        assert_eq!(65210, run_phase_settings(&proc, 
            &vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,
                1,32,31,31,4,31,99,0,0,0],
            &vec![1,0,4,3,2]));
    }
}
