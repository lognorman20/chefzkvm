use crate::algebra::FieldElement;
use std::{
    cmp::max,
    ffi::NulError,
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
        let (quo, rem) = self.divide(&self, &rhs).unwrap();
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

#[derive(Debug)]
enum PolynomialError {
    DivByZero(String),
}

impl Polynomial {
    pub fn new(coefficients: Vec<FieldElement>) -> Self {
        Polynomial { coefficients }
    }

    /// Returns the index of where the last non zero `FieldElement` is.
    pub fn degree(&self) -> i128 {
        let zero = self.coefficients[0].field.zero();
        let non_zero_coeff_cnt = self
            .coefficients
            .iter()
            .filter(|fe| fe != &&zero)
            .collect::<Vec<_>>()
            .len();

        if non_zero_coeff_cnt == 0 || self.coefficients.is_empty() {
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

    /// Divides two `Polynomial`s and returns their `(Quotient, Remainder)`.
    fn divide(
        &self,
        numerator: &Self,
        denominator: &Self,
    ) -> Result<(Self, Self), PolynomialError> {
        if denominator.degree() == -1 {
            Err(PolynomialError::DivByZero(String::from(
                "can't divide by zero big bro",
            )))
        } else if numerator.degree() < denominator.degree() {
            Ok((Polynomial::new(Vec::new()), numerator.clone()))
        } else {
            let field = denominator.coefficients[0].field;
            let mut remainder = numerator.clone();
            let mut quotient_coefficients: Vec<FieldElement> =
                (0..numerator.degree() - denominator.degree() + 1)
                    .map(|_| field.zero())
                    .collect();
            for _ in 0..numerator.degree() - denominator.degree() + 1 {
                if remainder.degree() < denominator.degree() {
                    break;
                } else {
                    let coefficient =
                        remainder.leading_coefficient() / denominator.leading_coefficient();
                    let shift: usize = (remainder.degree() - denominator.degree())
                        .try_into()
                        .unwrap();
                    let mut sub_coeff = (0..shift)
                        .map(|_| field.zero())
                        .collect::<Vec<FieldElement>>();
                    sub_coeff.push(coefficient);
                    let subtractee: Polynomial = Polynomial::new(sub_coeff) * (denominator.clone());
                    quotient_coefficients[shift] = coefficient;
                    remainder = remainder - subtractee;
                }
            }

            let quotient = Polynomial::new(quotient_coefficients);
            Ok((quotient, remainder))
        }
    }

    pub fn modulo(&self, rhs: &Self) -> Self {
        let (_quo, rem) = self.divide(self, rhs).unwrap();
        rem
    }

    pub fn modexp(&self, exponent: i128) -> Self {
        if self.is_zero() {
            Polynomial::new(Vec::new())
        } else if exponent == 0 {
            Polynomial::new(vec![self.coefficients[0].field.one()])
        } else {
            let mut acc = Polynomial::new(vec![self.coefficients[0].field.one()]);
            let binary_str = format!("{:b}", exponent);
            for i in (0..binary_str.len() - 2).rev() {
                acc = acc * acc;
                if (1 << i) & exponent != 0 {
                    acc = acc * val;
                }
            }

            acc
        }
    }
}
