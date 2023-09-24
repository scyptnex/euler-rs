use itertools::*;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (0..=9)
        .permutations(10)
        .filter(has_prop)
        .map(|v| to_num(v.as_slice()))
        .sum()
}

fn has_prop(v: &Vec<u64>) -> bool {
    let vs: Vec<_> = v.windows(3).map(to_num).skip(1).collect();
    for (i, v) in [2, 3, 5, 7, 11, 13, 17].into_iter().enumerate() {
        if vs[i] % v != 0 {
            return false;
        }
    }
    true
}

fn to_num(v: &[u64]) -> u64 {
    v.iter()
        .rev()
        .enumerate()
        .map(|(i, d)| 10u64.pow(i as u32) * d)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(to_num(&[0, 4, 2]), 42);
        assert_eq!(to_num(&[0]), 0);

        assert!(has_prop(&vec![1, 4, 0, 6, 3, 5, 7, 2, 8, 9]));
        assert!(!has_prop(&vec![1, 4, 0, 6, 3, 5, 7, 2, 9, 8]));
    }
}
