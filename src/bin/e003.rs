use euler_rs::prime;

fn main() {
    println!("{}", lpf(600851475143));
}

fn lpf(n: u64) -> u64 {
    let max_f = (n as f64).sqrt() as u64 + 1;
    let sieve = prime::sieve(max_f as usize);
    let mut rem = n;
    for p in sieve {
        while rem % p == 0 {
            rem /= p;
        }
        if rem == 1 {
            return p;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(lpf(13195), 29);
    }
}
