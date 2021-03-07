use lazy_static::lazy_static;
use num_bigint::{BigUint, ToBigUint};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::rc::Rc;
use std::sync::RwLock;

thread_local! {
    static MODULUS: Rc<RefCell<BigUint>> = Rc::new(RefCell::new(2.to_biguint().unwrap()));
}

pub type Field = GaloisField<DefaultModulus>;

/// Galois field with `order`. In other words, ℤ/mℤ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GaloisField<M> {
    value: BigUint,
    phantom: PhantomData<M>,
}

impl<M: Modulus> GaloisField<M> {
    pub fn new(value: BigUint) -> Self {
        // let value = value % *Self::modulus().borrow();
        Self {
            value,
            phantom: PhantomData,
        }
    }

    pub fn modulus() -> Rc<RefCell<BigUint>> {
        M::modulus()
    }

    pub fn update_modulus(modulus: BigUint) {
        M::update(modulus)
    }
}

trait Modulus {
    fn modulus() -> Rc<RefCell<BigUint>>;
    fn update(modulus: BigUint);
}

#[derive(Clone, Debug)]
pub struct DefaultModulus;

impl Modulus for DefaultModulus {
    fn modulus() -> Rc<RefCell<BigUint>> {
        MODULUS.with(|rc| rc.clone())
    }

    fn update(modulus: BigUint) {
        MODULUS.with(|m| *m.borrow_mut() = modulus);
    }
}

impl<M: Modulus> Add for GaloisField<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut value = self.value + rhs.value;
        if value >= *Self::modulus().borrow() {
            value -= *Self::modulus().borrow();
        }
        GaloisField {
            value,
            phantom: PhantomData,
        }
    }
}

impl<M: Modulus> AddAssign for GaloisField<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value >= *Self::modulus().borrow() {
            self.value -= *Self::modulus().borrow();
        }
    }
}

/*
impl<M: Modulus> Sub for GaloisField<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let value = if self.value < rhs.value {
            Self::modulus() + self.value - rhs.value
        } else {
            self.value - rhs.value
        };
        GaloisField {
            value,
            phantom: PhantomData,
        }
    }
}

impl<M: Modulus> SubAssign for GaloisField<M> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += Self::modulus();
        }
        self.value -= rhs.value;
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        Field::update_modulus(8.to_biguint().unwrap());
        let mut x = Field::new(3.to_biguint().unwrap());
        let y = Field::new(5.to_biguint().unwrap());
        let z = x.clone() + y.clone();
        assert_eq!(Field::new(0.to_biguint().unwrap()), z);
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
