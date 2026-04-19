pub struct Dumped<T> {
    t: T,
}

impl<T> Iterator for Dumped<T>
where
    T: Iterator,
    T::Item: std::fmt::Debug,
{
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.t.next() {
            Some(x) => {
                println!("{:?}", x);
                Some(x)
            }
            None => None,
        }
    }
}

pub trait Dumpable: Iterator {
    fn dumply(self) -> Dumped<Self>
    where
        Self::Item: std::fmt::Debug,
        Self: Sized,
    {
        Dumped { t: self }
    }
}

impl<T> Dumpable for T where T: Iterator + Sized {}

#[cfg(test)]
mod tests {
    use super::Dumpable;

    #[test]
    fn test_dump() {
        (1..10).dumply();
    }
}
