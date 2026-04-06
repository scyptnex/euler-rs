fn solve(n: usize) -> u64 {
    let mut tbl = vec![vec![0; n + 1]; n + 1];
    // tbl [i][j] <- sum to i with highest number j.

    // There is 1 way of summing to each number using only 1s.
    for sum_to in 1..=n {
        tbl[sum_to][1] = 1;
    }
    // There is 1 way to sum to zero using only zeroes.
    for at_most in 0..=n {
        tbl[0][at_most] = 1;
    }

    for at_most in 2..=n {
        for sum_to in 1..=n {
            for used_am in 0.. {
                let taken = used_am * at_most;
                if taken > sum_to {
                    break;
                }
                let rem = sum_to - taken;
                tbl[sum_to][at_most] += tbl[rem][at_most - 1];
            }
        }
    }

    // -1 because n = n doesn't count.
    tbl[n][n] - 1
}

fn main() {
    println!("{}", solve(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(5), 6);
    }
}
