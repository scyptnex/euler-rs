use std::collections::HashSet;

use itertools::Itertools;

fn splus(s: &Vec<i64>) -> Option<Vec<i64>> {
    let mut r = s.clone();
    let t = r.pop().unwrap();
    *r.last_mut().unwrap() += t;
    Some(r)
}

fn sminus(s: &Vec<i64>) -> Option<Vec<i64>> {
    let mut r = s.clone();
    let t = r.pop().unwrap();
    *r.last_mut().unwrap() -= t;
    Some(r)
}

fn stimes(s: &Vec<i64>) -> Option<Vec<i64>> {
    let mut r = s.clone();
    let t = r.pop().unwrap();
    *r.last_mut().unwrap() *= t;
    Some(r)
}

fn sdivs(s: &Vec<i64>) -> Option<Vec<i64>> {
    let mut r = s.clone();
    let t = r.pop().unwrap();
    if t != 0 && r.last().unwrap() % t == 0 {
        *r.last_mut().unwrap() /= t;
        Some(r)
    } else {
        None
    }
}

fn cover_tree(cbl: &mut HashSet<i64>, nums: &Vec<&i64>, s: Vec<i64>, ops: usize) {
    let mut s = s;
    if ops == 3 && nums.len() == 0 {
        cbl.insert(s[0]);
        return;
    }
    if s.len() >= 2 {
        if let Some(ss) = splus(&s) {
            cover_tree(cbl, nums, ss, ops + 1);
        }
        if let Some(ss) = sminus(&s) {
            cover_tree(cbl, nums, ss, ops + 1);
        }
        if let Some(ss) = stimes(&s) {
            cover_tree(cbl, nums, ss, ops + 1);
        }
        if let Some(ss) = sdivs(&s) {
            cover_tree(cbl, nums, ss, ops + 1);
        }
    }
    if nums.len() > 0 {
        s.push(**nums.last().unwrap());
        cover_tree(
            cbl,
            &nums.iter().take(nums.len() - 1).copied().collect_vec(),
            s,
            ops,
        );
    }
}

fn covered(v: &Vec<i64>) -> i64 {
    let mut calculatable = HashSet::<i64>::new();
    for nums in v.iter().permutations(4) {
        cover_tree(&mut calculatable, &nums, Vec::new(), 0);
    }
    for i in 1.. {
        if !calculatable.contains(&i) {
            return i - 1;
        }
    }
    panic!()
}

fn solve() -> String {
    (1..=9)
        .combinations(4)
        .map(|v| (covered(&v), v))
        .max()
        .unwrap()
        .1
        .iter()
        .sorted()
        .map(|i| format!("{i}"))
        .join("")
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(covered(&vec![1, 2, 3, 4]), 28);
        assert_eq!(solve(), "1258");
    }
}
