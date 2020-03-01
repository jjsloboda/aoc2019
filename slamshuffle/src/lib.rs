use std::io::BufRead;

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

fn shuffle(cards: &mut Vec<u64>, tqn: &Technique) {
    match tqn {
        Technique::NewStack => cards.reverse(),
        Technique::Cut(x) => if *x >= 0 {
            cards.rotate_left(*x as usize);
        } else {
            cards.rotate_right(-x as usize);
        },
        Technique::DealInc(x) => {
            let len = cards.len();
            let mut c = vec![0u64; len];
            for i in 0..len {
                c[i * x % len] = cards[i];
            }
            cards.swap_with_slice(&mut c);
        },
    };
}

pub fn shuffle_cards<T: BufRead>(data: T) -> Vec<u64> {
    let tqns = load_techniques(data);
    let mut cards = Vec::with_capacity(10007);
    let mut i = 0u64;
    cards.resize_with(10007, || { i += 1; i - 1});
    for t in tqns {
        shuffle(&mut cards, &t);
    }
    cards
}

fn deshuffle_card(card: u128, num_cards: u128, tqn: &Technique) -> u128 {
    //println!("c: {}, nc: {}", card, num_cards);
    match tqn {
        Technique::NewStack => num_cards - card - 1,
        Technique::Cut(x) => (
            card as i128 - *x as i128).rem_euclid(num_cards as i128) as u128,
        Technique::DealInc(x) => (
            -(card as i128 * *x as i128)).rem_euclid(num_cards as i128) as u128,
    }
}

pub fn fully_deshuffle_card<T: BufRead>(data: T,
        mut card: u128, num_cards: u128, iters: u128) -> u128 {
    let mut tqns = load_techniques(data);
    tqns.reverse();
    for i in 0..iters {
        if card == 2020 {
            println!("i: {} c: {}", i, card);
        }
        for tqn in tqns.iter() {
            card = deshuffle_card(card, num_cards, tqn);
        }
    }
    card
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
}
