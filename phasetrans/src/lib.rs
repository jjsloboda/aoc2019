use std::cmp::{max, min};

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

struct RowMask {
    i: usize,
    i_cycle: usize,
    cycle_len: usize,
    repeat: usize,
    offset: usize,
}
impl RowMask {
    pub fn new(r: usize, o: usize) -> Self {
        Self{ offset: o, repeat: r, i: 0, i_cycle: 4 * r - 1, cycle_len: 4 * r }
    }
    pub fn masked_sum(&mut self, a: &Vec<i32>) -> i32 {
        a.iter().filter(|_| self.next().unwrap()).sum::<i32>()
    }
}
impl Iterator for RowMask {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        if self.i <= self.offset {
            Some(false)
        } else {
            self.i_cycle += 1;
            if self.i_cycle == self.cycle_len {
                self.i_cycle = 0;
            }
            Some(self.i_cycle < self.repeat)
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

fn make_xform_matrix(n: usize) -> Vec<Vec<i32>> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(PhaseRowPattern::new(i+1, n).collect());
    }
    out
}

fn calculate_next_phase(a: &Vec<i32>, x: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut out = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        let p = dot_product(a, &x[i]);
        out.push(p.abs() % 10);
    }
    out
}

pub fn calculate_phases(n: i32, a: &Vec<i32>) -> Vec<i32> {
    let xform = make_xform_matrix(a.len());
    let mut phase = a.clone();
    for i in 0..n {
        println!("calculating phase {}...", i+1);
        phase = calculate_next_phase(&phase, &xform);
    }
    phase
}

fn calculate_next_phase_in_place(a: &mut Vec<i32>) {
    for i in 0..a.len() {
        let pos_sum = RowMask::new(i+1, i).masked_sum(a);
        let neg_sum = RowMask::new(i+1, (i+1)*3-1).masked_sum(a);
        a[i] = (pos_sum - neg_sum).abs() % 10;
    }
}

pub fn calculate_phases_2(n: i32, a: &Vec<i32>) -> Vec<i32> {
    let mut phase = a.clone();
    for i in 0..n {
        println!("calculating phase {}...", i+1);
        calculate_next_phase_in_place(&mut phase);
    }
    phase
}

enum Sign {
    POS,
    NEG,
}

struct BandRangeSumIter<'a> {
    a: &'a Vec<i64>,
    band: usize,
    sign: Sign,
    cur_row: usize,
    cur_sum: i64,
}
impl<'a> BandRangeSumIter<'a> {
    pub fn new(a: &'a Vec<i64>, band: usize) -> Self {
        BandRangeSumIter{
            a: a,
            band: band,
            sign: if band % 4 == 1 { Sign::POS } else { Sign::NEG },
            cur_row: 1,
            cur_sum: 0,
        }
    }
    fn index_range(&self, row: usize) -> (usize, usize) {
        let start = self.band * row;
        (min(start - 1, self.a.len()), min(start + row - 1, self.a.len()))
    }
    fn signed_sum(&self) -> i64 {
        match self.sign {
            Sign::POS => self.cur_sum,
            Sign::NEG => -self.cur_sum,
        }
    }
}
impl<'a> Iterator for BandRangeSumIter<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, end) = self.index_range(self.cur_row);
        if self.cur_row == 1 {
            self.cur_sum += self.a[self.band - 1];
            self.cur_row += 1;
            Some(self.signed_sum())
        } else if start < self.a.len() {
            let (prev_start, prev_end) = self.index_range(self.cur_row - 1);
            for i in prev_start..min(prev_end, start) {
                self.cur_sum -= self.a[i];
            }
            for i in max(prev_end, start)..end {
                self.cur_sum += self.a[i];
            }
            self.cur_row += 1;
            Some(self.signed_sum())
        } else {
            None
        }
    }
}

pub fn calculate_phases_3(n: i64, a: &Vec<i64>) -> Vec<i64> {
    let mut phase = a.clone();
    for i in 0..n {
        println!("calculating phase {}...", i+1);
        let mut bands = Vec::new();
        for i in 0..phase.len() {
            if i % 2 == 0 {
                bands.push(BandRangeSumIter::new(&phase, i+1));
            }
        }
        let mut next_phase = Vec::new();
        for d in 0..phase.len() {
            let mut d_sum = 0;
            for b in bands.iter_mut() {
                if let Some(x) = b.next() {
                    d_sum += x;
                }
            }
            bands.truncate(phase.len() / ((d+1)*2) + 1 as usize);
            next_phase.push(d_sum.abs() % 10);
        }
        phase = next_phase;
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
    fn check_row_mask() {
        use super::RowMask;
        let mut rm1 = RowMask::new(1, 0);
        assert_eq!(true, rm1.next().unwrap());
        assert_eq!(false, rm1.next().unwrap());
        assert_eq!(false, rm1.next().unwrap());
        assert_eq!(false, rm1.next().unwrap());
        assert_eq!(true, rm1.next().unwrap());
        assert_eq!(false, rm1.next().unwrap());
        assert_eq!(false, rm1.next().unwrap());
        assert_eq!(false, rm1.next().unwrap());
        assert_eq!(true, rm1.next().unwrap());

        let mut rm2 = RowMask::new(2, 9);
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(true, rm2.next().unwrap());
        assert_eq!(true, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(false, rm2.next().unwrap());
        assert_eq!(true, rm2.next().unwrap());
    }

    #[test]
    fn check_dot_product() {
        use super::dot_product;
        assert_eq!(62, dot_product(&vec![9, 8, 7, 6, 5], &vec![1, 2, 3, 1, 2]));
    }

    #[test]
    fn check_next_phase() {
        use super::{calculate_next_phase, make_xform_matrix};
        let x = make_xform_matrix(8);
        assert_eq!(vec![4, 8, 2, 2, 6, 1, 5, 8],
            calculate_next_phase(&vec![1, 2, 3, 4, 5, 6, 7, 8], &x));
        assert_eq!(vec![3, 4, 0, 4, 0, 4, 3, 8],
            calculate_next_phase(&vec![4, 8, 2, 2, 6, 1, 5, 8], &x));
        assert_eq!(vec![0, 3, 4, 1, 5, 5, 1, 8],
            calculate_next_phase(&vec![3, 4, 0, 4, 0, 4, 3, 8], &x));
        assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8],
            calculate_next_phase(&vec![0, 3, 4, 1, 5, 5, 1, 8], &x));
    }

    #[test]
    fn check_calculate_phases_basic() {
        use super::{calculate_phases, calculate_phases_2, calculate_phases_3};
        assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8],
            calculate_phases(4, &vec![1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8],
            calculate_phases_2(4, &vec![1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8],
            calculate_phases_3(4, &vec![1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn check_calculate_phases_large_inputs() {
        //use super::{calculate_phases, calculate_phases_2};
        use super::{calculate_phases, calculate_phases_3};
        /*
        assert_eq!(vec![2, 4, 1, 7, 6, 1, 7, 6],
            calculate_phases(100, &vec![
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0,
                8, 3, 2, 1, 8, 6, 4, 5, 5, 9, 5,
            ])[..8].iter().map(|&x| x).collect::<Vec<_>>());
        assert_eq!(vec![2, 4, 1, 7, 6, 1, 7, 6],
            calculate_phases_2(100, &vec![
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0,
                8, 3, 2, 1, 8, 6, 4, 5, 5, 9, 5,
            ])[..8].iter().map(|&x| x).collect::<Vec<_>>());
        */
        assert_eq!(vec![2, 4, 1, 7, 6, 1, 7, 6],
            calculate_phases_3(100, &vec![
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0,
                8, 3, 2, 1, 8, 6, 4, 5, 5, 9, 5,
            ])[..8].iter().map(|&x| x).collect::<Vec<_>>());
    }

    #[test]
    fn check_band_range_sum_iter() {
        use super::BandRangeSumIter;
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(vec![1, 2+3, 3+4+5, 4+5+6+7, 5+6+7+8, 6+7+8, 7+8, 8],
            BandRangeSumIter::new(&a, 1).collect::<Vec<_>>());
        assert_eq!(vec![-3, -13],
            BandRangeSumIter::new(&a, 3).collect::<Vec<_>>());
    }
}
