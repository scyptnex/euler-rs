use std::collections::{hash_map::Entry, HashMap};

// a(sqrt(x) + b)/c
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Numi {
    a: isize,
    x: usize,
    b: isize,
    c: isize,
}

impl Numi {
    fn start(x: usize) -> Self {
        Self {
            a: 1,
            x,
            b: 0,
            c: 1,
        }
    }

    // t + remainder == original
    fn pull_term(&self) -> (isize, Self) {
        let t = self.estimate();
        let abmtc = self.a * self.b - self.c * t;
        assert!(abmtc % self.a == 0);
        let new_b = abmtc / self.a;
        (
            t,
            Self {
                a: self.a,
                x: self.x,
                b: new_b,
                c: self.c,
            },
        )
    }

    fn to_f64(&self) -> f64 {
        let estim = self.x as f64;
        let estim = estim.sqrt() + (self.b as f64);
        let estim = estim * (self.a as f64);
        estim / (self.c as f64)
    }

    fn estimate(&self) -> isize {
        (self.to_f64().floor()) as isize
    }

    // self (0,1) -> 1/ ret(>1)
    fn inv(&self) -> Self {
        let a = self.c;
        let b = -self.b;
        let c = self.a * (self.x as isize) - self.a * self.b * self.b;
        assert!(c % a == 0);
        let c = c / a;
        let a = 1;
        Self { a, x: self.x, b, c }
    }
}

fn frac_period(n: usize) -> (Vec<isize>, usize) {
    if n.isqrt() * n.isqrt() == n {
        return (Vec::new(), 0);
    }
    let mut cur = Numi::start(n);
    let mut ret = Vec::new();
    let mut memo = HashMap::<Numi, usize>::new();
    loop {
        let (addi, remi) = cur.pull_term();
        ret.push(addi);
        let entry = memo.entry(remi);
        if let Entry::Occupied(en) = entry {
            return (ret, *en.get());
        }
        entry.or_insert(ret.len());
        let remf = remi.to_f64();
        assert!(remf > 0f64 && remf < 1f64);
        cur = remi.inv();
    }
}

fn solve(ul: usize) -> usize {
    (1..=ul)
        .map(frac_period)
        .map(|(v, r)| v.len() - r)
        .filter(|p| p % 2 == 1)
        .count()
}

fn main() {
    println!("{}", solve(10000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(frac_period(23), (vec![4, 1, 3, 1, 8], 1));
        assert_eq!(solve(13), 4);
    }
}
