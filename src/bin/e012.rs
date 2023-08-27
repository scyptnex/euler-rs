fn main() {
    println!("{}", solve(500));
}

fn solve(count: u64) -> u64 {
    let mut s: u64 = 0;
    for t in 1.. {
        s += t;
        if count_divs(s) > count {
            return s;
        }
    }
    panic!();
}

fn count_divs(n: u64) -> u64 {
    (1..(n as f64).sqrt() as u64).filter(|x| n % x == 0).count() as u64 * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(count_divs(28), 6);
        assert_eq!(solve(5), 28);
    }
}
