use itertools::Itertools;

fn cg(w: u64, h: u64) -> u64 {
    (1..=w)
        .cartesian_product(1..=h)
        .map(|(ww, hh)| (w - ww + 1) * (h - hh + 1))
        .sum()
}

fn tabl(w: u64, h: u64, t: &mut Vec<Vec<u64>>) -> u64 {
    if h > w {
        return tabl(h, w, t);
    }
    let wi = (w - 1) as usize;
    let hi = (h - 1) as usize;
    //dbg!(w, h, &t);
    assert!(wi <= t.len());
    if wi == t.len() {
        t.push(Vec::new());
    }
    assert!(hi <= t[wi].len());
    if hi == t[wi].len() {
        t[wi].push(cg(w, h));
    }
    t[wi][hi]
}

fn solve() -> u64 {
    let mut rec = Vec::new();
    let mut best = 0;
    let mut barea = 0;
    for w in 1.. {
        for h in 1..=w {
            let cur = tabl(w, h, &mut rec);
            let diff = 2000000 - (cur as i64);
            let best_diff = 2000000 - (best as i64);
            if diff.abs() < best_diff.abs() {
                best = cur;
                barea = w * h;
            }
            if cur > 2000000 {
                if h == 1 {
                    return barea;
                }
                break;
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
    fn test_cg() {
        assert_eq!(cg(3, 2), 18);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 2772);
    }
}
