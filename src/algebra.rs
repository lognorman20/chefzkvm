use serde::ser::{Serialize, SerializeStruct};
use std::ops;

/// The Extended Euclidean Algorithm to calculate the multiplicate inverse of a
/// `FieldElement`.
pub fn xgcd(a: u128, b: u128) -> (u128, u128, u128) {
    assert!(b != 0);
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);

    let mut n = 0;
    while r1 != 0 {
        let q = r0 / r1;

        r0 = if r0 > q * r1 {
            r0 - q * r1
        } else {
            q * r1 - r0
        };
        (r0, r1) = (r1, r0);

        s0 = s0 + q * s1;
        (s0, s1) = (s1, s0);

        t0 = t0 + q *t1;
        (t0, t1) = (t1, t0);

        n += 1;
    }

    if n % 2 != 0 {
        s0 = b - s0;
    } else {
        t0 = a - t0;
    }

    (s0, t0, r0)
}

#[derive(Default, Copy, Clone, Debug)]
pub struct FieldElement {
    value: u128,
    field: Field,
}

impl ops::Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        self.field.add(self, rhs)
    }
}

impl ops::Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        self.field.multiply(self, rhs)
    }
}

impl ops::Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        self.field.subtract(self, rhs)
    }
}

impl ops::Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        self.field.divide(self, rhs)
    }
}

impl ops::Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        self.field.negate(self)
    }
}

/// Modular exponentiation, NOT bitwise xor fr
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
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("FieldElement", 2)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("field", &self.field)?;
        state.end()
    }
}

impl FieldElement {
    pub fn new(value: u128, field: Field) -> Self {
        Self { value, field }
    }

    fn inverse(&self) -> Self {
        self.field.inverse(*self)
    }

    fn modexp(&self, exponent: u128) -> Self {
        let mut c = 1;
        for _i in 0..exponent - 1 {
            c = (c * self.value) % self.field.p;
        }

        FieldElement {
            value: c,
            field: self.field,
        }
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Field {
    p: u128,
}

impl Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Field", 1)?;
        state.serialize_field("p", &self.p)?;
        state.end()
    }
}

impl Field {
    pub fn new(p: u128) -> Self {
        Field { p }
    }

    pub fn zero(&self) -> FieldElement {
        FieldElement {
            value: 0,
            field: *self,
        }
    }

    pub fn one(&self) -> FieldElement {
        FieldElement {
            value: 1,
            field: *self,
        }
    }

    pub fn add(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        FieldElement {
            value: (a.value + b.value) % self.p,
            field: *self,
        }
    }

    pub fn multiply(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        FieldElement {
            value: (a.value * b.value) % self.p,
            field: *self,
        }
    }

    pub fn subtract(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        FieldElement {
            value: (a.value - b.value) % self.p,
            field: *self,
        }
    }

    pub fn divide(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        assert!(!b.is_zero());
        let (s, _t, _r) = xgcd(a.value, b.value);

        FieldElement {
            value: (a.value * s) % self.p,
            field: *self,
        }
    }

    pub fn negate(&self, operand: FieldElement) -> FieldElement {
        FieldElement {
            value: (self.p - operand.value) % self.p,
            field: *self,
        }
    }

    pub fn inverse(&self, operand: FieldElement) -> FieldElement {
        let (a, _b, _g) = xgcd(operand.value, self.p);

        FieldElement {
            value: a,
            field: *self,
        }
    }
}
