fn main() {
    println!("{}", solve(100));
}

fn solve(n: u64) -> u64 {
    let n_sum: u64 = (1..=n).sum();
    (n_sum * n_sum) - (1..=n).map(|x| x * x).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 2640);
    }
}
