use std::cmp::{max, min};

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

pub fn calculate_phases(n: i64, a: &Vec<i64>) -> Vec<i64> {
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
