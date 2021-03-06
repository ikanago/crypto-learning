use num_bigint::BigUint;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Galois field with `order`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    value: BigUint,
    order: BigUint,
}

impl Field {
    pub fn new(value: BigUint, order: BigUint) -> Self {
        Self {
            value: value % &order,
            order,
        }
    }

    fn assert_order(&self, rhs: &Self) -> bool {
        self.order == rhs.order
    }
}

impl Add for Field {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.assert_order(&rhs));
        let mut value = self.value + rhs.value;
        if value >= self.order {
            value -= &self.order;
        }
        Field {
            value,
            order: self.order,
        }
    }
}

impl AddAssign for Field {
    fn add_assign(&mut self, rhs: Self) {
        assert!(self.assert_order(&rhs));
        self.value += rhs.value;
        if self.value >= self.order {
            self.value -= &self.order;
        }
    }
}

impl Sub for Field {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.assert_order(&rhs));
        let value = if self.value < rhs.value {
            &self.order + self.value - rhs.value
        } else {
            self.value - rhs.value
        };
        Field {
            value,
            order: self.order,
        }
    }
}

impl SubAssign for Field {
    fn sub_assign(&mut self, rhs: Self) {
        assert!(self.assert_order(&rhs));
        if self.value < rhs.value {
            self.value += &self.order;
        }
        self.value -= rhs.value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn addition() {
        let mut x = Field::new(3.to_biguint().unwrap(), 8.to_biguint().unwrap());
        let y = Field::new(5.to_biguint().unwrap(), 8.to_biguint().unwrap());
        let z = x.clone() + y.clone();
        assert_eq!(
            Field::new(0.to_biguint().unwrap(), 8.to_biguint().unwrap()),
            z
        );
        x += y;
        assert_eq!(x, z);
    }

    #[test]
    fn subtraction() {
        let mut x = Field::new(3.to_biguint().unwrap(), 8.to_biguint().unwrap());
        let y = Field::new(5.to_biguint().unwrap(), 8.to_biguint().unwrap());
        let z = x.clone() - y.clone();
        assert_eq!(
            Field::new(6.to_biguint().unwrap(), 8.to_biguint().unwrap()),
            z
        );
        x -= y;
        assert_eq!(x, z);
    }
}
