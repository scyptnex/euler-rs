use euler_rs::decimal::Digs;

fn main() {
    println!("{}", solve(5));
}

fn solve(pow: u32) -> u64 {
    let upper = (1..)
        .map(|p| 10u64.pow(p) - 1)
        .take_while(|l| sod(*l, pow) >= *l)
        .last()
        .unwrap()
        * 10
        + 9;
    (2..upper).filter(|n| is_sum_of_dig_pow(*n, pow)).sum()
}

fn is_sum_of_dig_pow(n: u64, pow: u32) -> bool {
    sod(n, pow) == n
}

fn sod(n: u64, pow: u32) -> u64 {
    n.digs().iter().map(|d: &u64| d.pow(pow)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(is_sum_of_dig_pow(8208, 4));
        assert_eq!(solve(4), 19316);
    }
}
