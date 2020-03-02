use std::io::BufRead;
use modinverse::modinverse;
use mod_exp::mod_exp;

enum Technique {
    NewStack,
    Cut(i64),
    DealInc(usize),
}

fn load_techniques<T: BufRead>(data: T) -> Vec<Technique> {
    let mut techniques = Vec::new();
    for line in data.lines() {
        let li = line.expect("error reading line from file");
        techniques.push(if li.starts_with("deal into new stack") {
            Technique::NewStack
        } else if li.starts_with("cut") {
            let x = li[4..].parse::<i64>().expect("bad cut int");
            Technique::Cut(x)
        } else { // if line.starts_with("deal with increment") {
            let x = li[20..].parse::<usize>().expect("bad dealinc int");
            Technique::DealInc(x)
        });
    }
    techniques
}

#[derive(Debug)]
struct Xform {
    a: i128,
    b: i128,
    m: i128,
}
impl Xform {
    fn add_to(&mut self, a2: i128, b2: i128) {
        let (a1, b1) = (self.a, self.b);
        self.a = (a1 * a2).rem_euclid(self.m);
        self.b = (a2 * b1 + b2).rem_euclid(self.m);
    }
    fn tqn_fwd(&self, tqn: &Technique) -> (i128, i128) {
        match tqn {
            Technique::NewStack => (-1, -1),
            Technique::Cut(k) => (1, -k as i128),
            Technique::DealInc(k) => (*k as i128, 0),
        }
    }
    fn tqn_rev(&self, tqn: &Technique) -> (i128, i128) {
        match tqn {
            Technique::NewStack => (-1, -1),
            Technique::Cut(k) => (1, *k as i128),
            Technique::DealInc(k) => (
                modinverse(*k as i128, self.m).expect("no inverse"), 0),
        }
    }
    pub fn from_tqns_fwd<
            'a, T: IntoIterator<Item=&'a Technique>>(m: i128, tqns: T) -> Self {
        let mut xf = Self{ a: 1, b: 0, m: m, };
        for tqn in tqns.into_iter() {
            let (a, b) = xf.tqn_fwd(&tqn);
            xf.add_to(a, b);
        }
        xf
    }
    pub fn from_tqns_rev<
            'a, T: IntoIterator<Item=&'a Technique>>(m: i128, tqns: T) -> Self {
        let mut xf = Self{ a: 1, b: 0, m: m, };
        for tqn in tqns.into_iter() {
            let (a, b) = xf.tqn_rev(&tqn);
            xf.add_to(a, b);
        }
        xf
    }
    pub fn compose(&self, n: usize) -> Self {
        let a_n = mod_exp(self.a, n as i128, self.m);
        let b_denom_inv = modinverse(1 - self.a, self.m).expect("no inv");
        let b_numer = (self.b * (1 - a_n)).rem_euclid(self.m);
        let b = b_numer * b_denom_inv;
        Xform{ a: a_n, b: b, m: self.m, }
    }
    pub fn apply_to(&self, c: i128) -> i128 {
        (self.a * c + self.b).rem_euclid(self.m)
    }
}

pub fn fully_shuffle_card<T: BufRead>(data: T,
        card: u128, num_cards: u128, iters: u128) -> u128 {
    let tqns = load_techniques(data);
    let xf = Xform::from_tqns_fwd(num_cards as i128, &tqns);
    let mut c = card;
    for _ in 0..iters {
        c = xf.apply_to(c as i128).rem_euclid(num_cards as i128) as u128
    }
    c
}

pub fn fully_deshuffle_card<T: BufRead>(data: T,
        card: u128, num_cards: u128, iters: usize) -> u128 {
    let mut tqns = load_techniques(data);
    tqns.reverse();
    let xf = Xform::from_tqns_rev(num_cards as i128, &tqns);
    let xfc = xf.compose(iters);
    xfc.apply_to(card as i128).rem_euclid(num_cards as i128) as u128
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_shuffle_1() {
        use super::{shuffle, Technique};
        let mut cards = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let tqns = vec![
            Technique::DealInc(7),
            Technique::NewStack,
            Technique::NewStack,
        ];
        for tqn in tqns.iter() {
            shuffle(&mut cards, tqn);
        }
        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], cards);
    }

    #[test]
    fn basic_shuffle_2() {
        use super::{shuffle, Technique};
        let mut cards = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let tqns = vec![
            Technique::Cut(6),
            Technique::DealInc(7),
            Technique::NewStack,
        ];
        for tqn in tqns.iter() {
            shuffle(&mut cards, tqn);
        }
        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], cards);
    }

    #[test]
    fn basic_shuffle_3() {
        use super::{shuffle, Technique};
        let mut cards = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let tqns = vec![
            Technique::DealInc(7),
            Technique::DealInc(9),
            Technique::Cut(-2),
        ];
        for tqn in tqns.iter() {
            shuffle(&mut cards, tqn);
        }
        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], cards);
    }

    #[test]
    fn basic_shuffle_4() {
        use super::{shuffle, Technique};
        let mut cards = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let tqns = vec![
            Technique::NewStack,
            Technique::Cut(-2),
            Technique::DealInc(7),
            Technique::Cut(8),
            Technique::Cut(-4),
            Technique::DealInc(7),
            Technique::Cut(3),
            Technique::DealInc(9),
            Technique::DealInc(3),
            Technique::Cut(-1),
        ];
        for tqn in tqns.iter() {
            shuffle(&mut cards, tqn);
        }
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], cards);
    }

    #[test]
    fn deshuffle_new_stack() {
        use super::{deshuffle_card, Technique};
        assert_eq!(6, deshuffle_card(3, 10, &Technique::NewStack));
        assert_eq!(3, deshuffle_card(6, 10, &Technique::NewStack));
        assert_eq!(0, deshuffle_card(9, 10, &Technique::NewStack));
        assert_eq!(9, deshuffle_card(0, 10, &Technique::NewStack));
    }

    #[test]
    fn deshuffle_cut() {
        use super::{deshuffle_card, Technique};
        assert_eq!(0, deshuffle_card(3, 10, &Technique::Cut(3)));
        assert_eq!(1, deshuffle_card(4, 10, &Technique::Cut(3)));
        assert_eq!(2, deshuffle_card(5, 10, &Technique::Cut(3)));
        assert_eq!(3, deshuffle_card(6, 10, &Technique::Cut(3)));
        assert_eq!(4, deshuffle_card(7, 10, &Technique::Cut(3)));
        assert_eq!(5, deshuffle_card(8, 10, &Technique::Cut(3)));
        assert_eq!(6, deshuffle_card(9, 10, &Technique::Cut(3)));
        assert_eq!(7, deshuffle_card(0, 10, &Technique::Cut(3)));
        assert_eq!(8, deshuffle_card(1, 10, &Technique::Cut(3)));
        assert_eq!(9, deshuffle_card(2, 10, &Technique::Cut(3)));

        assert_eq!(0, deshuffle_card(3, 7, &Technique::Cut(-4)));
        assert_eq!(1, deshuffle_card(4, 7, &Technique::Cut(-4)));
        assert_eq!(2, deshuffle_card(5, 7, &Technique::Cut(-4)));
        assert_eq!(3, deshuffle_card(6, 7, &Technique::Cut(-4)));
        assert_eq!(4, deshuffle_card(0, 7, &Technique::Cut(-4)));
        assert_eq!(5, deshuffle_card(1, 7, &Technique::Cut(-4)));
        assert_eq!(6, deshuffle_card(2, 7, &Technique::Cut(-4)));
    }

    #[test]
    fn deshuffle_deal_inc() {
        use super::{deshuffle_card, Technique};
        assert_eq!(0, deshuffle_card(0, 10, &Technique::DealInc(7)));
        assert_eq!(1, deshuffle_card(7, 10, &Technique::DealInc(7)));
        assert_eq!(2, deshuffle_card(4, 10, &Technique::DealInc(7)));
        assert_eq!(3, deshuffle_card(1, 10, &Technique::DealInc(7)));
        assert_eq!(4, deshuffle_card(8, 10, &Technique::DealInc(7)));
        assert_eq!(5, deshuffle_card(5, 10, &Technique::DealInc(7)));
        assert_eq!(6, deshuffle_card(2, 10, &Technique::DealInc(7)));
        assert_eq!(7, deshuffle_card(9, 10, &Technique::DealInc(7)));
        assert_eq!(8, deshuffle_card(6, 10, &Technique::DealInc(7)));
        assert_eq!(9, deshuffle_card(3, 10, &Technique::DealInc(7)));
    }

    #[test]
    fn xform_basic_1() {
        use super::{Technique, Xform};
        let mut tqns = vec![
            Technique::DealInc(7),
            Technique::DealInc(9),
            Technique::Cut(-2),
        ];

        let xff = Xform::from_tqns_fwd(&tqns);
        assert_eq!(2, xff.apply_to(0).rem_euclid(10));
        assert_eq!(5, xff.apply_to(1).rem_euclid(10));
        assert_eq!(8, xff.apply_to(2).rem_euclid(10));
        assert_eq!(1, xff.apply_to(3).rem_euclid(10));
        assert_eq!(4, xff.apply_to(4).rem_euclid(10));
        assert_eq!(7, xff.apply_to(5).rem_euclid(10));
        assert_eq!(0, xff.apply_to(6).rem_euclid(10));
        assert_eq!(3, xff.apply_to(7).rem_euclid(10));
        assert_eq!(6, xff.apply_to(8).rem_euclid(10));
        assert_eq!(9, xff.apply_to(9).rem_euclid(10));

        tqns.reverse();
        let xfr = Xform::from_tqns_rev(10, &tqns);
        assert_eq!(0, xfr.apply_to(2).rem_euclid(10));
        assert_eq!(1, xfr.apply_to(5).rem_euclid(10));
        assert_eq!(2, xfr.apply_to(8).rem_euclid(10));
        assert_eq!(3, xfr.apply_to(1).rem_euclid(10));
        assert_eq!(4, xfr.apply_to(4).rem_euclid(10));
        assert_eq!(5, xfr.apply_to(7).rem_euclid(10));
        assert_eq!(6, xfr.apply_to(0).rem_euclid(10));
        assert_eq!(7, xfr.apply_to(3).rem_euclid(10));
        assert_eq!(8, xfr.apply_to(6).rem_euclid(10));
        assert_eq!(9, xfr.apply_to(9).rem_euclid(10));
    }

    #[test]
    fn xform_basic_2() {
        use super::{Technique, Xform};
        let mut tqns = vec![
            Technique::NewStack,
            Technique::Cut(-2),
            Technique::DealInc(7),
            Technique::Cut(8),
            Technique::Cut(-4),
            Technique::DealInc(7),
            Technique::Cut(3),
            Technique::DealInc(9),
            Technique::DealInc(3),
            Technique::Cut(-1),
        ];

        let xff = Xform::from_tqns_fwd(&tqns);
        assert_eq!(7, xff.apply_to(0).rem_euclid(10));
        assert_eq!(4, xff.apply_to(1).rem_euclid(10));
        assert_eq!(1, xff.apply_to(2).rem_euclid(10));
        assert_eq!(8, xff.apply_to(3).rem_euclid(10));
        assert_eq!(5, xff.apply_to(4).rem_euclid(10));
        assert_eq!(2, xff.apply_to(5).rem_euclid(10));
        assert_eq!(9, xff.apply_to(6).rem_euclid(10));
        assert_eq!(6, xff.apply_to(7).rem_euclid(10));
        assert_eq!(3, xff.apply_to(8).rem_euclid(10));
        assert_eq!(0, xff.apply_to(9).rem_euclid(10));

        tqns.reverse();
        let xfr = Xform::from_tqns_rev(10, &tqns);
        assert_eq!(0, xfr.apply_to(7).rem_euclid(10));
        assert_eq!(1, xfr.apply_to(4).rem_euclid(10));
        assert_eq!(2, xfr.apply_to(1).rem_euclid(10));
        assert_eq!(3, xfr.apply_to(8).rem_euclid(10));
        assert_eq!(4, xfr.apply_to(5).rem_euclid(10));
        assert_eq!(5, xfr.apply_to(2).rem_euclid(10));
        assert_eq!(6, xfr.apply_to(9).rem_euclid(10));
        assert_eq!(7, xfr.apply_to(6).rem_euclid(10));
        assert_eq!(8, xfr.apply_to(3).rem_euclid(10));
        assert_eq!(9, xfr.apply_to(0).rem_euclid(10));
    }
}
