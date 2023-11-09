use num::BigUint;

fn main() {
    println!("{}", solve(1000));
}

fn solve(lim: u32) -> String {
    let bui: BigUint = (1..=lim).map(|i| BigUint::from(i).pow(i)).sum();
    let s = format!("{}", bui);
    s.split_at(s.len() - 10).1.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), "0405071317");
    }
}
