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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let d: String = "1337".to_owned();
        assert_eq!(d.as_str().digs().iter().sum::<u32>(), 14);
    }
}
