use euler_rs::prime::PrimeSieve;
use itertools::Itertools;

fn solve() -> u64 {
    let bound = 100;
    let ps = PrimeSieve::new(2 * bound).iter().collect_vec();
    // tbl [p][l] -> primes below and equal to p and a limit l.
    let mut tbl = vec![vec![0; bound]; ps.len()];

    // Using only the smallest prime we can make the multiples of it
    for lim in 0..bound {
        if lim % ps[0] == 0 {
            tbl[0][lim] = 1;
        }
    }

    for pi in 1..ps.len() {
        let p = ps[pi];
        // there is always 1 way to get to 0.
        tbl[pi][0] = 1;
        for lim in 1..bound {
            for used_p in 0.. {
                let taken = p * used_p;
                if taken > lim {
                    break;
                }
                let rem = lim - taken;
                tbl[pi][lim] += tbl[pi - 1][rem];
            }
        }
    }

    //tbl.iter().for_each(|v| {
    //    println!("{v:?}");
    //});
    (0..bound)
        .find(|lim| tbl.iter().any(|v| v[*lim] > 5000))
        .unwrap() as u64
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 71);
    }
}
