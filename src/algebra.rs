use std::ops;
use serde::{Deserialize, Serialize};

/// The Extended Euclidean Algorithm to calculate the multiplicate inverse of a
/// `FieldElement`.
pub fn xgcd(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        /// Error handle with division by zero?
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_s, old_t, old_r)
}
#[derive(Default, Debug)]
struct FieldElement {
    value: u128,
    field: Field
}

impl ops::Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Div for FieldElement {
    type Output = FieldElement;
    
    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Neg for FieldElement {
    type Output = FieldElement;
    
    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl ops::BitXor for FieldElement {
    type Output = FieldElement;
    
    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Serialize for FieldElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}

impl FieldElement {
    pub fn new(value: u128, field: Field) -> Self{
        Self { value, field}
    }

    fn inverse(&self) -> Self {

    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

#[derive(Default, Debug)]
struct Field {
    p: u128
}

impl Field {
    fn zero() -> FieldElement {

    }

    fn one() -> FieldElement {

    }

    fn add(a: FieldElement, b: FieldElement) -> FieldElement {

    }

    fn multiply(a: FieldElement, b: FieldElement) -> FieldElement {

    }

    fn subtract(a: FieldElement, b: FieldElement) -> FieldElement {

    }

    fn divide(a: FieldElement, b: FieldElement) -> FieldElement {

    }

    fn negate(operand: FieldElement) -> FieldElement {

    }

    fn inverse(operand: FieldElement) -> FieldElement {

    }
}
