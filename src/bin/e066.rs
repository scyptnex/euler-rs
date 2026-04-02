use std::collections::HashSet;

fn diophantine(d: usize) -> (usize, usize) {
    (2..)
        .map(|x| (x, (x * x - 1)))
        //.inspect(|v| {
        //    println!("  {:?}", v);
        //})
        .filter(|(_, r)| r % d == 0)
        .map(|(x, r)| (x, r / d))
        //.inspect(|v| {
        //    println!("  {} -> {:?}", d, v);
        //})
        .filter(|(_, y)| y.isqrt() * y.isqrt() == *y)
        .next()
        .unwrap()
}

fn old_min_dio(d_in: usize) -> usize {
    (2..=d_in)
        .filter(|d| d.isqrt() * d.isqrt() != *d)
        .map(|d| (d, diophantine(d)))
        .inspect(|d| {
            dbg!(d);
        })
        .max_by_key(|(_, dph)| dph.0)
        .unwrap()
        .0
}

fn min_dio(d_in: usize) -> usize {
    let mut remaining = (2..=d_in)
        .filter(|d| d.isqrt() * d.isqrt() != *d)
        .collect::<HashSet<usize>>();
    for x in 2.. {
        let target = x * x - 1;
        let mut remv = Vec::new();
        for r in &remaining {
            if target % r != 0 {
                continue;
            }
            let ysq = target / r;
            if ysq.isqrt() * ysq.isqrt() != ysq {
                continue;
            }
            println!(
                "{}^2 - {} * {}^2 = 1   ({} remaining)",
                x,
                r,
                ysq.isqrt(),
                remaining.len()
            );
            remv.push(*r);
        }
        for r in remv {
            remaining.remove(&r);
            if remaining.is_empty() {
                return r;
            }
        }
    }
    panic!();
}

fn solve() -> usize {
    min_dio(1000)
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(min_dio(7), 5);
    }
}
