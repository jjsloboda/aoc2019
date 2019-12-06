use std::collections::vec_deque::VecDeque;

fn split_num(mut i: i32) -> Vec<i32> {
    let mut digits: VecDeque<i32> = VecDeque::new();
    while i > 0 {
        digits.push_front(i % 10);
        i /= 10;
    }
    Vec::from(digits)
}

fn has_double_digit(i: i32) -> bool {
    let digits = split_num(i);
    for d in 0..digits.len()-1 {
        if digits[d] == digits[d+1] {
            return true
        }
    }
    false
}

fn has_monotonic_digit_increase(i: i32) -> bool {
    let digits = split_num(i);
    for d in 0..digits.len()-1 {
        if digits[d] > digits[d+1] {
            return false
        }
    }
    true
}

fn has_exactly_double_digits(i:i32) -> bool {
    #[derive(PartialEq)]
    enum State {
        INITIAL,
        DOUBLE,
        OVER_DOUBLE,
    };
    let mut state = State::INITIAL;
    let digits = split_num(i);
    for d in 1..digits.len() {
        match state {
            State::INITIAL => {
                if digits[d-1] == digits[d] {
                    state = State::DOUBLE;
                }
            },
            State::DOUBLE => {
                if digits[d-1] == digits[d] {
                    state = State::OVER_DOUBLE;
                } else {
                    return true
                }
            },
            State::OVER_DOUBLE => {
                if digits[d-1] != digits[d] {
                    state = State::INITIAL;
                }
            },
        };
    }
    state == State::DOUBLE
}

fn is_valid(i: i32) -> bool {
    has_double_digit(i)
        && has_monotonic_digit_increase(i)
}

fn is_more_valid(i: i32) -> bool {
    is_valid(i)
        && has_exactly_double_digits(i)
}

fn num_possible_codes(start: i32, end: i32, validity_fn: &Fn(i32) -> bool) -> i32 {
    let mut count = 0;
    for i in start..=end {
        count += if validity_fn(i) { 1 } else { 0 };
    }
    count
}

pub fn num_initial_possible_codes(start: i32, end: i32) -> i32 {
    num_possible_codes(start, end, &is_valid)
}

pub fn num_actual_possible_codes(start: i32, end: i32) -> i32 {
    num_possible_codes(start, end, &is_more_valid)
}

pub fn num_proper_possible_codes(start: i32, end: i32) -> i32 {
    let mut count = 0;
    for i in start..=end {
        count += if is_more_valid(i) { 1 } else { 0 };
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{has_double_digit, has_monotonic_digit_increase, is_valid, is_more_valid};

    #[test]
    fn double_digit() {
        assert_eq!(true, has_double_digit(16388529));
        assert_eq!(true, has_double_digit(9943112));
        assert_eq!(true, has_double_digit(47854977));
        assert_eq!(true, has_double_digit(11));
        assert_eq!(false, has_double_digit(12438276));
        assert_eq!(false, has_double_digit(9876543));
        assert_eq!(false, has_double_digit(2321323));
        assert_eq!(false, has_double_digit(12));
    }

    #[test]
    fn monotonic_increase() {
        assert_eq!(true, has_monotonic_digit_increase(11));
        assert_eq!(true, has_monotonic_digit_increase(12));
        assert_eq!(true, has_monotonic_digit_increase(1357999));
        assert_eq!(false, has_monotonic_digit_increase(111110));
        assert_eq!(false, has_monotonic_digit_increase(999991));
        assert_eq!(false, has_monotonic_digit_increase(1236787));
    }

    #[test]
    fn validity() {
        assert_eq!(true, is_valid(111111));
        assert_eq!(false, is_valid(223450));
        assert_eq!(false, is_valid(123789));
    }

    #[test]
    fn more_validity() {
        assert_eq!(true, is_more_valid(111122));
        assert_eq!(true, is_more_valid(11));
        assert_eq!(false, is_more_valid(111111));
        assert_eq!(false, is_more_valid(112112));
    }
}
