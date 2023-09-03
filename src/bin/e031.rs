fn main() {
    println!("{}", solve(200));
}

fn solve(target: usize) -> u64 {
    let coins = [2, 5, 10, 20, 50, 100, 200];
    // This is the number of ways of making an amt using only 1p coins, i.e. there is always
    // exactly one way.
    let mut ways: Vec<u64> = vec![1; target + 1];
    for cur_coin in coins {
        ways = (0..=target)
            .map(|v| {
                // Sum the individual ways of making |v| with |cur_amt| of the current coin
                // assuming you use only coins less than that to make the remainder.
                (0..=v / cur_coin)
                    .map(|cur_amt| ways[v - (cur_amt * cur_coin)])
                    .sum()
            })
            .collect();
    }
    ways[target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(solve(10), 11);
    }
}
