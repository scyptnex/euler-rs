use euler_rs::sqrt::find_continued_period;

fn solve(ul: usize) -> usize {
    (1..=ul)
        .map(|x| find_continued_period(x).1.len())
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
        assert_eq!(solve(13), 4);
    }
}
