use bigint::U256;
use std::error::Error as StdError;
use std::hash::Hash;
use std::ops;

/// Implementation from (https://stackoverflow.com/a/70501399)
pub fn xgcd(a: &U256, b: &U256) -> (U256, U256, U256) {
    let zero: U256 = U256::zero();
    let one: U256 = U256::one();
    assert!(*b != zero);
    let (mut r0, mut r1) = (*a, *b);
    let (mut s0, mut s1) = (one, zero);
    let (mut t0, mut t1) = (zero, one);

    let mut n = 0;
    while r1 != U256::zero() {
        let q = r0 / r1;

        r0 = if r0 > q * r1 {
            r0 - q * r1
        } else {
            q * r1 - r0
        };
        (r0, r1) = (r1, r0);

        s0 = s0 + q * s1;
        (s0, s1) = (s1, s0);

        t0 = t0 + q * t1;
        (t0, t1) = (t1, t0);

        n += 1;
    }

    if n % 2 != 0 {
        s0 = *b - s0;
    } else {
        t0 = *a - t0;
    }

    (s0, t0, r0)
}

#[derive(Default, Copy, Clone, Debug)]
pub struct FieldElement {
    pub value: U256,
    pub field: Field,
}

impl ops::Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        self.field.add(&self, &rhs)
    }
}

impl ops::AddAssign for FieldElement {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        self.field.multiply(&self, &rhs)
    }
}

impl ops::MulAssign for FieldElement {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        self.field.subtract(&self, &rhs)
    }
}

impl ops::Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        self.field.divide(&self, &rhs)
    }
}

impl ops::Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        self.field.negate(self)
    }
}

impl Hash for FieldElement {
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state)
        }
    }
    
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.field.hash(state);
    }
}

impl Eq for FieldElement {}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl FieldElement {
    pub fn new(value: U256, field: Field) -> Self {
        Self { value, field }
    }

    pub fn inverse(&self) -> Self {
        self.field.inverse(*self)
    }

    pub fn modexp(&self, exponent: usize) -> Self {
        let one: U256 = U256::one();
        let mut acc = FieldElement::new(one, self.field);
        let val = FieldElement::new(self.value, self.field);

        let binary_str = format!("{:b}", exponent);
        for i in (0..binary_str.len() - 2).rev() {
            acc = acc * acc;
            if (1 << i) & exponent != 0 {
                acc = acc * val;
            }
        }

        acc
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    pub fn bytes(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Field {
    p: U256,
}

#[derive(Debug)]
pub struct FieldError {
    message: String,
}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for FieldError {}

impl Hash for Field {
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state)
        }
    }
    
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.p.hash(state);
    }
}

impl Field {
    pub fn new(p: U256) -> Self {
        Field { p }
    }

    pub fn zero(&self) -> FieldElement {
        FieldElement {
            value: U256::zero(),
            field: *self,
        }
    }

    pub fn one(&self) -> FieldElement {
        FieldElement {
            value: U256::one(),
            field: *self,
        }
    }

    pub fn add(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        FieldElement {
            value: (a.value + b.value) % self.p,
            field: *self,
        }
    }

    pub fn multiply(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        FieldElement {
            value: (a.value * b.value) % self.p,
            field: *self,
        }
    }

    pub fn subtract(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        FieldElement {
            value: (a.value - b.value) % self.p,
            field: *self,
        }
    }

    pub fn divide(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        assert!(!b.is_zero());
        let (s, _t, _r) = xgcd(&b.value, &self.p);

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
        let (a, _b, _g) = xgcd(&operand.value, &self.p);

        FieldElement {
            value: a,
            field: *self,
        }
    }

    pub fn generator(&self) -> FieldElement {
        let generator: U256 = U256::from_dec_str("85408008396924667383611388730472331217").unwrap();
        let field_size: U256 =
            U256::from_dec_str("270497897142230380135924736767050121217").unwrap();
        assert!(self.p == field_size, "bro what field is that");
        return FieldElement {
            value: generator,
            field: *self,
        };
    }

    /// Ensures STARK property that the subgroup of power-of-two order exists by
    /// generating the "primitive nth root"
    pub fn primite_nth_root(&self, n: &U256) -> Result<FieldElement, FieldError> {
        let zero: U256 = U256::zero();
        let one: U256 = U256::one();
        let generator: U256 = U256::from_dec_str("85408008396924667383611388730472331217").unwrap();
        let field_size: U256 =
            U256::from_dec_str("270497897142230380135924736767050121217").unwrap();
        if self.p == field_size {
            assert!(
                *n <= one << 119 && (*n & (*n - one)) == zero,
                "field doesn't have the nth root of unity bro"
            );
            let mut root = FieldElement::new(generator, *self);
            let mut order: U256 = one << 119;
            while order != *n {
                root = root.modexp(2);
                order = order / U256::from(2);
            }
            Ok(root)
        } else {
            Err(FieldError { message: "u don't even know the field, how am i supposed to get the return root of unity???".to_string() })
        }
    }

    /// Gets random bytes and turn them into a field element.
    pub fn sample(&self, byte_array: &[U256]) -> FieldElement {
        let zero: U256 = U256::zero();
        let mut acc: U256 = zero;
        for b in byte_array {
            acc = (acc << 8) ^ *b;
        }

        FieldElement {
            value: acc % self.p,
            field: *self,
        }
    }
}
