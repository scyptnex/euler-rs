use std::cmp::max;
use std::collections::{HashMap, HashSet};

use euler_rs::decimal::Digs;
use euler_rs::infile;
use itertools::Itertools;

fn ptsb(lim: usize) -> Vec<HashSet<u64>> {
    let mut ret = Vec::new();
    let mut boundary = 1;
    for i in 1.. {
        let sq = i * i;
        if sq >= boundary {
            boundary *= 10;
            if ret.len() == lim {
                return ret;
            }
            ret.push(HashSet::new());
        }
        ret.last_mut().unwrap().insert(sq);
    }
    unreachable!()
}

fn cmap(w: &str, sq: u64) -> Option<[u64; 26]> {
    let ds = sq.digs();
    let mut c_to_d = [10u64; 26];
    let mut d_to_c = [0; 10];
    for (ci, c) in w.as_bytes().iter().enumerate() {
        let cus = (c - b'A') as usize;
        let d = ds[ci] as usize;
        if d_to_c[d] != 0 && d_to_c[d] != *c {
            return None;
        }
        if c_to_d[cus] != 10 && c_to_d[cus] != d as u64 {
            return None;
        }
        d_to_c[d] = *c;
        c_to_d[cus] = d as u64;
    }
    Some(c_to_d)
}

fn can_group(w1: &str, w2: &str, squares: &HashSet<u64>) -> u64 {
    // skip palindromes.
    if w1 == w2.chars().rev().collect::<String>() {
        return 0;
    }
    let mut mx = 0;
    for sq in squares {
        if let Some(map) = cmap(w1, *sq) {
            let mut w2d = 0;
            for c in w2.as_bytes() {
                let mi = (c - b'A') as usize;
                w2d *= 10;
                w2d += map[mi];
            }
            if squares.contains(&w2d) {
                mx = max(mx, *sq);
                mx = max(mx, w2d);
            }
        }
    }
    mx
}

fn solve() -> u64 {
    let inf = infile!();
    let words = inf
        .split(',')
        .map(|qs| qs.trim_start_matches('"').trim_end_matches('"'))
        .collect_vec();
    let mut groups = HashMap::<Vec<char>, Vec<&str>>::new();
    for w in &words {
        let cv = w.chars().sorted().collect_vec();
        groups.entry(cv).or_default().push(w);
    }
    groups = groups.into_iter().filter(|(_, v)| v.len() > 1).collect();
    let maxl = groups.keys().map(|k| k.len()).max().unwrap();
    let squares = ptsb(maxl);
    let mut largest = 0;
    for (_, group) in groups {
        for i in 0..group.len() - 1 {
            for j in i + 1..group.len() {
                let w1 = group[i];
                let w2 = group[j];
                let cg = can_group(w1, w2, &squares[w1.len() - 1]);
                largest = max(cg, largest);
            }
        }
    }
    largest
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let grp = ptsb(4);
        assert_eq!(can_group("RACE", "CARE", &grp[3]), 9216);
        assert_eq!(solve(), 18769);
    }
}
