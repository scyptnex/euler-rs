use itertools::Itertools;
use num::{BigInt, Num};

fn solution(n: usize, mgr: Vec<&usize>) -> Option<String> {
    let tripls = (0..n)
        .map(|i| (i + n, i, (i + 1) % n))
        .map(|(a, b, c)| (mgr[a], mgr[b], mgr[c]))
        .collect_vec();
    let trip_sum = |t: &(&usize, &usize, &usize)| t.0 + t.1 + t.2;
    let zeroth_sum = trip_sum(tripls.first().unwrap());
    let zeroth_out = tripls.first().unwrap().0;
    if tripls.iter().skip(1).map(trip_sum).all(|s| s == zeroth_sum)
        && tripls.iter().skip(1).map(|t| t.0).all(|o| o > zeroth_out)
    {
        Some(
            tripls
                .iter()
                .map(|t| format!("{}{}{}", t.0, t.1, t.2))
                .collect(),
        )
    } else {
        None
    }
}

fn solve(n: usize) -> String {
    let v = (1..=(2 * n)).collect_vec();
    v.iter()
        .permutations(2 * n)
        .flat_map(|v| solution(n, v))
        .filter(|s| s.len() <= 16)
        .max_by_key(|s| BigInt::from_str_radix(s, 10).unwrap())
        .unwrap()
}

fn main() {
    println!("{}", solve(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(3), "432621513");
        //assert_eq!(solve(5), "6531031914842725");
    }
}
