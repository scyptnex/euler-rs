use std::collections::{HashMap, HashSet};

fn ordigs(n: usize) -> usize {
    let mut digs = vec![0; 10];
    let mut rem = n;
    while rem > 0 {
        let low = rem % 10;
        rem = rem / 10;
        digs[low] += 1;
    }
    let mut ret = 0;
    for (d, dc) in digs.iter().enumerate().rev() {
        for _ in 0..*dc {
            ret = ret * 10 + d;
        }
    }
    ret
}

fn solve_d(p: usize, d: u32) -> Option<usize> {
    let upper = 10usize.pow(d);
    let lower = upper / 10;
    let mut groups = HashMap::<usize, HashSet<usize>>::new();
    for (k, v) in (1..)
        .map(|n: usize| n.pow(3))
        .filter(|n| *n >= lower)
        .take_while(|n| *n < upper)
        .map(|n| (ordigs(n), n))
    {
        groups.entry(k).or_default().insert(v);
    }
    groups
        .iter()
        .filter(|(_, v)| v.len() == p)
        .next()
        .map(|(_, v)| *v.iter().min().unwrap())
}

fn solve(p: usize) -> usize {
    (3..).flat_map(|d| solve_d(p, d)).next().unwrap()
}

fn main() {
    println!("{}", solve(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(ordigs(9380127645), 9876543210);
        //assert_eq!(solve(3), 41063625);
        //assert_eq!(solve(5), 127035954683);
    }
}
