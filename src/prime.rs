use std::iter::repeat;

/// Get the list of primes below |below|.
///
/// Example:
/// ```
/// use euler_rs::prime::sieve;
/// let primes = sieve(11);
/// assert_eq!(primes, vec![2,3,5,7]);
/// ```
pub fn sieve(below: usize) -> Vec<u64> {
    let mut sieve: Vec<bool> = repeat(true).take(below).collect();
    sieve[0] = false;
    sieve[1] = false;
    let mut primes = Vec::new();
    for i in 2..below {
        if !sieve[i] {
            continue;
        }
        primes.push(i as u64);
        for x in (2..).map(|j| j * i).take_while(|j| *j < below) {
            sieve[x] = false;
        }
    }
    primes
}
