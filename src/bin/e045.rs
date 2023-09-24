fn main() {
    println!("{}", solve(40755));
}

fn solve(threshold: u64) -> u64 {
    let mut pi = pents();
    let mut hi = hexes();
    let mut p = pi.next().unwrap();
    let mut h = hi.next().unwrap();
    for t in tris() {
        if p < t {
            p = pi.next().unwrap();
        }
        if h < t {
            h = hi.next().unwrap();
        }
        if t == h && t == p && t > threshold {
            return t;
        }
    }
    panic!();
}

fn tris() -> impl Iterator<Item = u64> {
    (1..).map(|n| (n * n + n) / 2)
}

fn pents() -> impl Iterator<Item = u64> {
    (1..).map(|n| n * (3 * n - 1) / 2)
}

fn hexes() -> impl Iterator<Item = u64> {
    (1..).map(|n| n * (2 * n - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(0), 1);
        assert_eq!(solve(1), 40755);
    }

    #[test]
    fn test_seq() {
        assert_eq!(tris().take(4).collect::<Vec<_>>(), vec![1, 3, 6, 10]);
        assert_eq!(pents().take(4).collect::<Vec<_>>(), vec![1, 5, 12, 22]);
        assert_eq!(hexes().take(4).collect::<Vec<_>>(), vec![1, 6, 15, 28]);
    }
}
