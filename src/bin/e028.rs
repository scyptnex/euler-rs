fn main() {
    println!("{}", solve(1001));
}

fn solve(sl: u64) -> u64 {
    let mut sum = 1;
    let mut count = 1;
    for ring in 1u64..(sl + 1) / 2 {
        for _ in 0..4 {
            count += 2 * ring;
            sum += count;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(5), 101);
    }
}
