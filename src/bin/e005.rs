use std::iter::*;

fn main() {
    // Brute force seems to work on my setup, took about 35s.
    //
    // Efficient would be to union the prime factors of 1..20.
    println!("{}", evdiv(20));
}

fn evdiv(n: u32) -> u64 {
    (n as u64..).find(|x| can_div(&n, x)).unwrap()
}

fn can_div(n: &u32, x: &u64) -> bool {
    (2..*n).all(|y| *x % (y as u64) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facs() {
        assert_eq!(evdiv(10), 2520);
    }
}
