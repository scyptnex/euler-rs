use itertools::Itertools;

fn totient_sieve(lim: u32) -> Vec<u32> {
    let mut v = (0..=lim).collect_vec();
    for idx in 2..=lim {
        // ignore composites
        if v[idx as usize] != idx {
            continue;
        }
        // idx is prime
        v[idx as usize] = idx - 1;
        (2..)
            .map(|m| m * idx)
            .take_while(|m| *m <= lim)
            .map(|m| m as usize)
            .for_each(|m| {
                v[m] = v[m] - (v[m] / idx);
            });
    }
    v
}

fn digit_frequency_histogram(n: u32) -> [usize; 10] {
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
    totient_sieve(lim)
        .into_iter()
        .enumerate()
        .skip(2)
        .filter(|(n, phin)| {
            digit_frequency_histogram(*n as u32) == digit_frequency_histogram(*phin)
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
    fn test_tot_sieve() {
        assert_eq!(totient_sieve(10), vec![0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(90_000), 75841); // phi(n) == 75184
    }
}
