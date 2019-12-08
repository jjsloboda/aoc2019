

pub fn permutations_heap_method<T: Clone>(a: &mut Vec<T>) -> Vec<Vec<T>> {
    let mut output = Vec::new();
    permutations_heap_method_helper(&mut output, a.len(), a);
    output
}
fn permutations_heap_method_helper<T: Clone>(output: &mut Vec<Vec<T>>, k: usize, a: &mut Vec<T>) {
    if k == 1 {
        output.push(a.clone());
    } else {
        permutations_heap_method_helper(output, k-1, a);
        for i in 0..k-1 {
            if k % 2 == 0 {
                a.swap(i, k-1);
            } else {
                a.swap(0, k-1);
            }
            permutations_heap_method_helper(output, k-1, a);
        }
    }
}

pub fn permutation_indices(n: usize) -> Vec<Vec<usize>> {
    if n == 0 { return vec![vec![]]; }
    let mut a = Vec::new();
    for i in 0..n {
        a.push(i);
    }
    permutations_heap_method(&mut a)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::permutation_indices;

    #[test]
    fn correct_index_counts() {
        assert_eq!(1, permutation_indices(1).len());
        assert_eq!(2, permutation_indices(2).len());
        assert_eq!(6, permutation_indices(3).len());
        assert_eq!(24, permutation_indices(4).len());
        assert_eq!(120, permutation_indices(5).len());
        assert_eq!(720, permutation_indices(6).len());
    }

    #[test]
    fn exhaustive_indices_for_3() {
        let mut actual = HashSet::new();
        for p in permutation_indices(3) {
            actual.insert(p);
        }
        assert!(actual.contains(&vec![0, 1, 2]));
        assert!(actual.contains(&vec![0, 2, 1]));
        assert!(actual.contains(&vec![1, 0, 2]));
        assert!(actual.contains(&vec![1, 2, 0]));
        assert!(actual.contains(&vec![2, 0, 1]));
        assert!(actual.contains(&vec![2, 1, 0]));
        assert_eq!(6, actual.len());
    }
}
