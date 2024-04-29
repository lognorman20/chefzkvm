use crate::algebra::FieldElement;
use std::{
    cmp::max,
    ops::{self, Add},
};

// TODO: Implement the copy trait to get rid of the `clone()` calls
#[derive(Debug, Clone)]
pub struct UPolynomial {
    coefficients: Vec<FieldElement>,
}

impl ops::Neg for UPolynomial {
    type Output = UPolynomial;

    fn neg(self) -> Self::Output {
        UPolynomial {
            coefficients: self.coefficients.into_iter().map(|fe| fe.neg()).collect(),
        }
    }
}

impl ops::Add for UPolynomial {
    type Output = UPolynomial;

    fn add(self, rhs: Self) -> Self::Output {
        if self.degree() == -1 {
            rhs
        } else if rhs.degree() == -1 {
            self
        } else {
            let field = self.coefficients[0].field;
            let mut acc: Vec<FieldElement> =
                (0..max(self.coefficients.len(), rhs.coefficients.len()))
                    .map(|_| field.zero())
                    .collect();

            for i in 0..self.coefficients.len() {
                acc[i] = acc[i] + self.coefficients[i];
            }

            for i in 0..rhs.coefficients.len() {
                acc[i] = acc[i] + rhs.coefficients[i];
            }

            UPolynomial { coefficients: acc }
        }
    }
}

impl ops::Sub for UPolynomial {
    type Output = UPolynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}

impl ops::Mul for UPolynomial {
    type Output = UPolynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.coefficients.is_empty() || rhs.coefficients.is_empty() {
            UPolynomial {
                coefficients: Vec::new(),
            }
        } else {
            let zero = self.coefficients[0].field.zero();
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

            UPolynomial { coefficients: buf }
        }
    }
}

impl ops::Div for UPolynomial {
    type Output = UPolynomial;

    fn div(self, rhs: Self) -> Self::Output {
        let (quo, rem) = self.divide(&self, &rhs).unwrap();
        assert!(rem.is_zero());
        quo
    }
}

impl PartialEq for UPolynomial {
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

impl UPolynomial {
    pub fn new(coefficients: Vec<FieldElement>) -> Self {
        UPolynomial { coefficients }
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
            Ok((UPolynomial::new(Vec::new()), numerator.clone()))
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
                    let subtractee: UPolynomial = UPolynomial::new(sub_coeff) * (denominator.clone());
                    quotient_coefficients[shift] = coefficient;
                    remainder = remainder - subtractee;
                }
            }

            let quotient = UPolynomial::new(quotient_coefficients);
            Ok((quotient, remainder))
        }
    }

    pub fn modulo(&self, rhs: &Self) -> Self {
        let (_quo, rem) = self.divide(self, rhs).unwrap();
        rem
    }

    pub fn modexp(&self, exponent: i128) -> Self {
        if self.is_zero() {
            UPolynomial::new(Vec::new())
        } else if exponent == 0 {
            UPolynomial::new(vec![self.coefficients[0].field.one()])
        } else {
            let mut acc = UPolynomial::new(vec![self.coefficients[0].field.one()]);
            let binary_str = format!("{:b}", exponent);
            for i in (0..binary_str.len() - 2).rev() {
                let tmp = acc.clone();
                acc = acc * tmp;
                if (1 << i) & exponent != 0 {
                    acc = acc * (self.clone());
                }
            }

            acc
        }
    }

    pub fn evaluate(&self, point: &FieldElement) -> FieldElement {
        let mut xi = point.field.one();
        let mut value = point.field.zero();
        for c in &self.coefficients {
            value += *c * xi; // bad paradigm of dereferencing?
            xi *= *point; // bad paradigm of dereferencing?
        }

        value
    }

    pub fn evaluate_domain(&self, domain: &Vec<FieldElement>) -> Vec<FieldElement> {
        let mut res = Vec::new();
        for p in domain {
            let val = self.evaluate(p);
            res.push(val);
        }

        res
    }

    pub fn interpolate_domain(
        &self,
        domain: &Vec<FieldElement>,
        values: &Vec<FieldElement>,
    ) -> Self {
        assert!(
            domain.len() == values.len(),
            "domain and values not the same length big bro"
        );
        assert!(
            domain.len() > 0,
            "can't interpolate between two values big bro"
        );

        let field = domain[0].field;
        let x = UPolynomial::new(vec![field.zero(), field.one()]);
        let mut acc = UPolynomial::new(Vec::new());
        for i in 0..domain.len() {
            let mut prod = UPolynomial::new(vec![values[i]]);
            for j in 0..domain.len() {
                if j == i {
                    continue;
                } else {
                    let poly_a = UPolynomial::new(vec![domain[j]]);
                    let poly_b = UPolynomial::new(vec![(domain[i] - domain[j]).inverse()]);
                    prod = prod * ((x.clone() - poly_a) * poly_b); // bad paradigm for sureski
                }
            }
            acc = acc + prod;
        }

        acc
    }

    pub fn zeroifier_domain(&self, domain: &Vec<FieldElement>) -> Self {
        let field = domain[0].field;
        let x = UPolynomial::new(vec![field.zero(), field.one()]);
        let mut acc = UPolynomial::new(vec![field.one()]);
        for d in domain {
            acc = acc
                * (x.clone()
                    - UPolynomial {
                        coefficients: vec![*d],
                    }); // bad paradigm for sureski
        }
        acc
    }

    pub fn scale(&self, factor: &FieldElement) -> Self {
        UPolynomial {
            coefficients: (0..self.coefficients.len())
                .map(|i| (factor.modexp(i)) * self.coefficients[i])
                .collect(),
        }
    }

    pub fn test_colinearity(&self, points: &Vec<(FieldElement, FieldElement)>) -> bool {
        let (mut domain, mut values) = (Vec::new(), Vec::new());
        let _ = points.iter().map(|pair| {
            domain.push(pair.0);
            values.push(pair.1);
        });

        let polynomial = self.interpolate_domain(&domain, &values);
        polynomial.degree() <= 1
    }
}
