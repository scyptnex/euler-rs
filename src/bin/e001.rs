fn main() {
    println!("{}", sum_mults_below(1000));
}

fn sum_mults_below(n: u32) -> u32 {
    (3..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum_mults_below(10), 23);
    }
}
