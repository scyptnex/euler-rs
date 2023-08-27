use euler_rs::fib;

fn main() {
    println!("{}", even_fibsum());
}

fn even_fibsum() -> u64 {
    fib::Fibonacci::new(1, 2)
        .take_while(|x| x <= &4_000_000)
        .filter(|x| x % 2 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(even_fibsum(), 4613732);
    }
}
