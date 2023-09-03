pub trait Digs<T> {
    fn digs(&self) -> Vec<T>;
}

impl<T: From<u32>> Digs<T> for &str {
    fn digs(&self) -> Vec<T> {
        self.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap().into())
            .collect()
    }
}

impl Digs<u64> for u64 {
    fn digs(&self) -> Vec<u64> {
        if *self < 10 {
            vec![*self]
        } else {
            let mut sub = (self / 10).digs();
            sub.push(self % 10);
            sub
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!("1337".digs().iter().sum::<u32>(), 14);
        assert_eq!(42.digs(), vec![4, 2]);
    }
}
