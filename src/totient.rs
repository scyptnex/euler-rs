use crate::{factors::prime_factors, prime::PrimeSieve};

// Get the totient of n.
pub fn totient(n: u64, ps: &PrimeSieve) -> u64 {
    let mut ret = n;
    for (p, _) in prime_factors(n, ps).iter() {
        ret = ret - (ret / p);
    }
    ret
}

// Get all the totients up to and including n, the totient of 0 is 0, and 1 is 1.
pub fn totient_sieve(n: u64) -> Vec<u64> {
    let mut v: Vec<u64> = (0..=n).collect();
    for idx in 2..=n {
        // ignore composites
        if v[idx as usize] != idx {
            continue;
        }
        // idx is prime
        v[idx as usize] = idx - 1;
        (2..)
            .map(|m| m * idx)
            .take_while(|m| *m <= n)
            .map(|m| m as usize)
            .for_each(|m| {
                v[m] = v[m] - (v[m] / idx);
            });
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tot() {
        let ps = PrimeSieve::new(20);
        assert_eq!(totient(6, &ps), 2);
        assert_eq!(totient(11, &ps), 10);
    }

    #[test]
    fn test_tot_sieve() {
        assert_eq!(totient_sieve(10), vec![0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
    }
}
