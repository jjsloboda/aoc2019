pub mod intcode;
use intcode::{Processor, Resources, Status};

pub fn run_phase_settings(proc: &Processor, pgrm: &Vec<isize>, settings: &Vec<usize>) -> isize {
    let mut signal = 0;
    for phase_setting in settings {
        let mut res = Resources::new(pgrm.clone());
        res.write_input(*phase_setting as isize);
        res.write_input(signal);
        proc.execute(&mut res);
        signal = res.read_output();
    }
    signal
}

pub fn run_feedback_loop(proc: &Processor, pgrm: &Vec<isize>, settings: &Vec<usize>) -> isize {
    let mut signal = 0;
    let mut amps = vec![Resources::new(pgrm.clone()); settings.len()];
    for i in 0..settings.len() {
        amps[i].write_input(settings[i] as isize);
        proc.execute(&mut amps[i]);
    }
    while amps[settings.len()-1].get_status() != Status::TERMINATED {
        for a in &mut amps {
            a.write_input(signal);
            proc.resume(a);
            signal = a.read_output();
        }
    }
    signal
}

#[cfg(test)]
mod tests {
    use super::{run_phase_settings, run_feedback_loop, Processor};

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

    #[test]
    fn feedback_example_1() {
        let proc = Processor::new_intcode();
        assert_eq!(139629729, run_feedback_loop(&proc,
            &vec![
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],
            &vec![9,8,7,6,5]));
    }

    #[test]
    fn feedback_example_2() {
        let proc = Processor::new_intcode();
        assert_eq!(18216, run_feedback_loop(&proc,
            &vec![
            3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,
            54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,
            53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10],
            &vec![9,7,8,5,6]));
    }
}
