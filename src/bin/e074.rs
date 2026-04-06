use std::collections::{HashMap, HashSet};

use euler_rs::decimal::Digs;

struct Cnrp {
    facts: [u64; 10],
    memo: HashMap<u64, u64>,
}

impl Cnrp {
    fn new() -> Self {
        Self {
            facts: [
                1,
                1,
                2,
                6,
                24,
                120,
                720,
                720 * 7,
                720 * 7 * 8,
                720 * 7 * 8 * 9,
            ],
            memo: HashMap::new(),
        }
    }

    fn fac(&self, d: u64) -> u64 {
        self.facts[d as usize]
    }

    fn nxt(&self, n: u64) -> u64 {
        n.digs().into_iter().map(|d| self.fac(d)).sum()
    }

    fn calc(&mut self, n: u64) {
        let mut path = Vec::<u64>::new();
        let mut seen = HashMap::<u64, usize>::new();
        let mut cur = n;
        let mut complete: u64;
        loop {
            if seen.contains_key(&cur) {
                //cycle
                let seen_idx = *seen.get(&cur).unwrap();
                let cyc_size = (path.len() - seen_idx) as u64;
                loop {
                    let cyc = path.pop().unwrap();
                    self.memo.insert(cyc, cyc_size);
                    if cyc == cur {
                        break;
                    }
                }
            }
            if self.memo.contains_key(&cur) {
                complete = 1 + *self.memo.get(&cur).unwrap();
                break;
            }
            seen.insert(cur, path.len());
            path.push(cur);
            cur = self.nxt(cur);
        }
        while let Some(p) = path.pop() {
            self.memo.insert(p, complete);
            complete += 1;
        }
    }

    fn cnrp(&mut self, n: u64) -> u64 {
        if !self.memo.contains_key(&n) {
            self.calc(n);
        }
        *self.memo.get(&n).unwrap()
    }
}

fn solve() -> usize {
    let mut c = Cnrp::new();
    (1..=1_000_000)
        .map(|n| c.cnrp(n))
        .filter(|n| *n == 60)
        .count()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reuse() {
        let mut c = Cnrp::new();
        assert_eq!(c.cnrp(45360), 3);
        assert_eq!(c.cnrp(78), 4);
    }

    #[test]
    fn test_solve() {
        assert_eq!(Cnrp::new().cnrp(145), 1);
        assert_eq!(Cnrp::new().cnrp(169), 3);
        assert_eq!(Cnrp::new().cnrp(1454), 3);
        assert_eq!(Cnrp::new().cnrp(45361), 2);
        assert_eq!(Cnrp::new().cnrp(69), 5);
        //assert_eq!(solve(), 402);
    }
}
