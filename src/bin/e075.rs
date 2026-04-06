use std::collections::HashMap;

use num::integer::gcd;

type Trips = HashMap<u64, u64>;

fn track_trips_with_numer(numer: u64, lim: u64, trips: &mut Trips) {
    for den_in in 1.. {
        let denom = if numer % 2 == 1 {
            numer - 1 + den_in * 2
        } else {
            numer + den_in
        };
        let a = denom * denom - numer * numer;
        let b = 2 * denom * numer;
        let c = denom * denom + numer * numer;
        let perim = a + b + c;
        if perim > lim {
            break;
        }
        if gcd(numer, denom) != 1 {
            continue;
        }
        for mult in 1.. {
            let pm = mult * perim;
            if pm > lim {
                break;
            }
            //println!("{numer}/{denom} | {a} {b} {c} | {pm}");
            *trips.entry(pm).or_default() += 1;
        }
    }
}

fn solve() -> u64 {
    let lim = 1_500_000;
    let mut t = Trips::new();
    for numer in 1..=lim {
        track_trips_with_numer(numer, lim, &mut t);
    }
    t.iter().filter(|(_, v)| **v == 1).count() as u64
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 161667);
    }
}
