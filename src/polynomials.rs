use crate::algebra::FieldElement;
use std::{
    cmp::max,
    ops::{self, Add},
};

#[derive(Debug, Clone)]
pub struct Polynomial {
    coefficients: Vec<FieldElement>,
}

impl ops::Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        Polynomial {
            coefficients: self.coefficients.into_iter().map(|fe| fe.neg()).collect(),
        }
    }
}

impl ops::Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        if self.degree() == -1 {
            rhs
        } else if rhs.degree() == -1 {
            self
        } else {
            let field = self.coefficients[0].field;
            let mut acc: Vec<FieldElement> =
                (1..max(self.coefficients.len(), rhs.coefficients.len()))
                    .map(|_| field.zero())
                    .collect();

            for i in 0..self.coefficients.len() {
                acc[i] = acc[i] + self.coefficients[i];
            }

            for i in 0..rhs.coefficients.len() {
                acc[i] = acc[i] + rhs.coefficients[i];
            }

            Polynomial { coefficients: acc }
        }
    }
}

impl ops::Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}

impl ops::Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.coefficients.is_empty() || rhs.coefficients.is_empty() {
            Polynomial {
                coefficients: Vec::new(),
            }
        } else {
            let zero = self.coefficients[0];
            let mut buf: Vec<FieldElement> =
                (0..(self.coefficients.len() + rhs.coefficients.len() - 1))
                    .map(|_| zero)
                    .collect();

            for i in 0..self.coefficients.len() {
                if self.coefficients[i].is_zero() {
                    continue;
                } else {
                    for j in 0..rhs.coefficients.len() {
                        buf[i + j] = buf[i + j] + self.coefficients[i] + rhs.coefficients[j];
                    }
                }
            }

            Polynomial { coefficients: buf }
        }
    }
}

impl ops::Div for Polynomial {
    type Output = Polynomial;

    fn div(self, rhs: Self) -> Self::Output {
        let (quo, rem) = self.divide(self.clone(), rhs);
        assert!(rem.is_zero());
        quo
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, rhs: &Self) -> bool {
        self.coefficients == rhs.coefficients
    }

    fn ne(&self, rhs: &Self) -> bool {
        !self.eq(rhs)
    }
}

impl Polynomial {
    pub fn new(coefficients: Vec<FieldElement>) -> Self {
        Polynomial { coefficients }
    }

    pub fn degree(&self) -> i128 {
        let zero = self.coefficients[0].field.zero();
        let zero_coeff_cnt = self
            .coefficients
            .iter()
            .filter(|fe| fe != &&zero)
            .collect::<Vec<_>>()
            .len();

        if zero_coeff_cnt != 0 || self.coefficients.is_empty() {
            -1
        } else {
            let mut max_index = 0;
            for i in 0..self.coefficients.len() {
                if self.coefficients[i] != zero {
                    max_index = i;
                }
            }

            max_index.try_into().unwrap()
        }
    }

    pub fn is_zero(&self) -> bool {
        if self.degree() == -1 {
            true
        } else {
            false
        }
    }

    pub fn leading_coefficient(&self) -> FieldElement {
        if self.degree() == -1 {
            self.coefficients[0].field.zero()
        } else {
            let index: usize = self.degree().try_into().unwrap();
            self.coefficients[index]
        }
    }

    fn divide(&self, numerator: Self, denominator: Self) -> (Polynomial, Polynomial) {
        unimplemented!()
    }

    pub fn modulo(&self) -> Self {
        unimplemented!()
    }

    pub fn modexp(&self, exponent: Self) -> Self {
        unimplemented!()
    }
}
