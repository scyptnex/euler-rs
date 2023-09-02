fn main() {
    println!("{}", solve(1000));
}

fn solve(digs: usize) -> usize {
    let mut m1 = num::BigUint::from(1u64);
    let mut m2 = m1.clone();
    for idx in 3.. {
        let m3 = m1 + &m2;
        if m3.to_string().len() >= digs {
            return idx;
        }
        m1 = m2;
        m2 = m3;
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(3), 12);
    }
}
