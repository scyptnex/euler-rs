use num::BigUint;

pub trait Factorial {
    fn factorial(&self) -> BigUint;
}

impl Factorial for u64 {
    fn factorial(&self) -> BigUint {
        let mut ret = BigUint::from(1u64);
        for i in 2..=*self {
            ret *= BigUint::from(i);
        }
        ret
    }
}
