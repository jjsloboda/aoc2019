

struct PhaseRowPattern {
    i: usize,
    len: usize,
    repeat: usize,
}
impl PhaseRowPattern {
    pub fn new(repeat: usize, len: usize) -> Self {
        Self{
            i: 0,
            len: len,
            repeat: repeat,
        }
    }
}
impl Iterator for PhaseRowPattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.len {
            self.i += 1;
            let cycle_index = self.i % (self.repeat * 4);
            let num = match cycle_index / self.repeat {
                0 => 0,
                1 => 1,
                2 => 0,
                3 => -1,
                _ => panic!("division logic error"),
            };
            Some(num)
        } else {
            None
        }
    }
}

fn dot_product(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    if a.len() != b.len() {
        panic!("a and b must have same length");
    }
    let mut total = 0;
    for i in 0..a.len() {
        total += a[i] * b[i];
    }
    total
}

fn calculate_next_phase(a: &Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();
    for i in 0..a.len() {
        let p = dot_product(a, &PhaseRowPattern::new(i+1, a.len()).collect());
        out.push(p.abs() % 10);
    }
    out
}

pub fn calculate_phases(n: i32, a: &Vec<i32>) -> Vec<i32> {
    let mut phase = a.clone();
    for _ in 0..n {
        phase = calculate_next_phase(&phase);
    }
    phase
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_row_patterns() {
        use super::PhaseRowPattern;
        assert_eq!(vec![1, 0, -1, 0], PhaseRowPattern::new(1, 4).collect::<Vec<_>>());
        assert_eq!(vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1],
            PhaseRowPattern::new(2, 10).collect::<Vec<_>>());
    }

    #[test]
    fn check_dot_product() {
        use super::dot_product;
        assert_eq!(62, dot_product(&vec![9, 8, 7, 6, 5], &vec![1, 2, 3, 1, 2]));
    }

    #[test]
    fn check_next_phase() {
        use super::calculate_next_phase;
        assert_eq!(vec![4, 8, 2, 2, 6, 1, 5, 8],
            calculate_next_phase(&vec![1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(vec![3, 4, 0, 4, 0, 4, 3, 8],
            calculate_next_phase(&vec![4, 8, 2, 2, 6, 1, 5, 8]));
        assert_eq!(vec![0, 3, 4, 1, 5, 5, 1, 8],
            calculate_next_phase(&vec![3, 4, 0, 4, 0, 4, 3, 8]));
        assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8],
            calculate_next_phase(&vec![0, 3, 4, 1, 5, 5, 1, 8]));
    }

    #[test]
    fn check_calculate_phases_basic() {
        use super::calculate_phases;
        assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8],
            calculate_phases(4, &vec![1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn check_calculate_phases_large_inputs() {
        use super::calculate_phases;
        assert_eq!(vec![2, 4, 1, 7, 6, 1, 7, 6],
            calculate_phases(100, &vec![
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0,
                8, 3, 2, 1, 8, 6, 4, 5, 5, 9, 5,
            ])[..8].iter().map(|&x| x).collect::<Vec<_>>());
    }
}
