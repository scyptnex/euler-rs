use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn poly_num(p: isize, n: isize) -> isize {
    let coef = p - 2;
    let term = 4 - p;
    n * (coef * n + term) / 2
}

fn pgen(p: isize) -> HashMap<isize, HashSet<isize>> {
    let mut ret = HashMap::<isize, HashSet<isize>>::new();
    for n in (1..)
        .map(|n| poly_num(p, n))
        .filter(|n| *n >= 1000)
        .take_while(|n| *n < 10000)
    {
        let k = n / 100;
        ret.entry(k).or_default().insert(n);
    }
    ret
}

fn cycle(
    target: isize,
    current: isize,
    avail: &mut HashSet<isize>,
    maps: &HashMap<isize, HashMap<isize, HashSet<isize>>>,
) -> Option<Vec<isize>> {
    let end = current % 100;
    if avail.len() == 1 {
        let map = &maps[&avail.iter().next().unwrap()];
        if !map.contains_key(&end) {
            return None;
        }
        return map[&end]
            .iter()
            .find(|last| *last % 100 == target)
            .map(|last| vec![*last, current]);
    }
    let options = avail.iter().copied().collect_vec();
    for o in options {
        if !maps[&o].contains_key(&end) {
            continue;
        }
        avail.remove(&o);
        for stems in maps[&o][&end].iter() {
            if let Some(mut cy) = cycle(target, *stems, avail, &maps) {
                cy.push(current);
                return Some(cy);
            }
        }
        avail.insert(o);
    }
    None
}

fn solve() -> isize {
    let maps = (3..=8)
        .map(|v| (v, pgen(v)))
        .collect::<HashMap<isize, HashMap<isize, HashSet<isize>>>>();
    let mut avail = (4..=8).collect::<HashSet<isize>>();
    for (t, tri) in maps[&3].iter() {
        for ri in tri.iter() {
            if let Some(cyc) = cycle(*t, *ri, &mut avail, &maps) {
                return cyc.iter().sum();
            }
        }
    }
    panic!();
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(poly_num(3, 2), 3);
        assert_eq!(poly_num(3, 3), 6);
        assert_eq!(poly_num(3, 4), 10);
        assert_eq!(poly_num(7, 2), 7);
        assert_eq!(poly_num(7, 3), 18);
        assert_eq!(poly_num(7, 4), 34);
        assert_eq!(solve(), 28684);
    }
}
