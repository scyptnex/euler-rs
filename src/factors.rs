use num::integer;

use crate::prime::PrimeSieve;

/// Return the proper factors including 1, excluding n
///
/// Example:
/// ```
/// use euler_rs::factors::get_factors;
/// let mut factors = get_factors(6).collect::<Vec<_>>();
/// factors.sort();
/// assert_eq!(factors, vec![1, 2, 3]);
/// ```
pub fn get_factors(n: u64) -> impl Iterator<Item = u64> {
    (1..=integer::sqrt(n))
        .filter(move |f| n % f == 0)
        .flat_map(move |x| {
            if x * x == n {
                [x, n].into_iter()
            } else {
                [x, n / x].into_iter()
            }
        })
        .filter(move |x| *x != n)
}

/// Return the prime factorization of n, as a list
/// [(p1, k1)...] having prime factor p1^k1.
pub fn prime_factors(n: u64, p: &PrimeSieve) -> Vec<(u64, u32)> {
    let mut n = n;
    let mut nxtp = p.iter();
    let mut ret = Vec::new();
    loop {
        if n == 1 {
            return ret;
        }
        if p.is_prime(n as usize) {
            ret.push((n, 1));
            return ret;
        }
        let pf = nxtp.next().unwrap() as u64;
        let mut pfd = 0;
        while n % pf == 0 {
            n /= pf;
            pfd += 1;
        }
        if pfd > 0 {
            ret.push((pf, pfd));
        }
    }
}
