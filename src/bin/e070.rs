use euler_rs::totient::totient_sieve;

fn digit_frequency_histogram(n: u64) -> [usize; 10] {
    let mut ret = [0; 10];
    let mut n = n;
    if n == 0 {
        ret[0] += 1;
    }
    while n > 0 {
        let dig = n % 10;
        ret[dig as usize] += 1;
        n /= 10;
    }
    ret
}

fn solve(lim: u32) -> u32 {
    totient_sieve(lim as u64)
        .into_iter()
        .enumerate()
        .skip(2)
        .filter(|(n, phin)| {
            digit_frequency_histogram(*n as u64) == digit_frequency_histogram(*phin)
        })
        //.inspect(|x| {
        //    println!("{x:?}");
        //})
        .map(|(n, phin)| (n, (n as f64) / (phin as f64)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .0 as u32
}

fn main() {
    println!("{}", solve(10_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(90_000), 75841); // phi(n) == 75184
    }
}
