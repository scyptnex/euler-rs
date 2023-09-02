use num::rational::Ratio;

fn main() {
    for p2 in 1..=0 {
        for p3 in 0..=p2 {
            for p5 in 0..=p3 {
                for p7 in 0..=p5 {
                    let n = 2u64.pow(p2) * 3u64.pow(p3) * 5u64.pow(p5) * 7u64.pow(p7);
                    println!("{} {} {} {} = {} -> {}", p2, p3, p5, p7, n, count_fracs(n));
                }
            }
        }
    }
    let s = get_fracs(6)
        .map(|l| format!("{} + {}", l.0, l.1))
        .collect::<Vec<_>>()
        .join("   -   ");
    println!("{}", s);
}

fn count_fracs(n: u64) -> u64 {
    get_fracs(n).count() as u64
}

fn get_fracs(n: u64) -> impl Iterator<Item = (Ratio<u64>, Ratio<u64>)> {
    let nf = Ratio::new(1, n);
    (n + 1..=2 * n)
        .map(|x| Ratio::new(1, x))
        .map(move |xf| (xf, (nf - xf).reduced()))
        .filter(|(_, yf)| *yf.numer() == 1)
}

fn solve() -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 0);
    }
}
