fn main() {
    println!("{}", palli(1000));
}

fn palli(n: u32) -> u32 {
    (1..n)
        .flat_map(|x| (1..n).map(move |y| x * y))
        .filter(is_palli)
        .max()
        .unwrap()
}

fn is_palli(n: &u32) -> bool {
    revi(n) == *n
}

fn revi(n: &u32) -> u32 {
    let mut rem: u32 = *n;
    let mut ans = 0;
    while rem != 0 {
        ans = ans * 10 + rem % 10;
        rem /= 10;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(palli(100), 9009);
    }

    #[test]
    fn test_revi() {
        assert_eq!(revi(&10), 1);
        assert_eq!(revi(&18), 81);
        assert_eq!(revi(&180), 81);
        assert_eq!(revi(&1801), 1081);
    }
}
