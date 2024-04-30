use std::cmp::max;
use std::collections::HashMap;
use std::ops;

use crate::algebra::{Field, FieldElement};

#[derive(Debug, Clone)]
pub struct MPolynomial {
    // vector of exponents : coefficients
    dictionary: HashMap<Vec<u128>, FieldElement>,
}

impl ops::Add for MPolynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut dictionary: HashMap<Vec<u128>, FieldElement> = HashMap::new();
        let max_self = self
            .dictionary
            .keys()
            .into_iter()
            .map(|k| k.len())
            .max()
            .unwrap();
        let max_rhs = rhs
            .dictionary
            .keys()
            .into_iter()
            .map(|k| k.len())
            .max()
            .unwrap();
        let num_variables = max(max_self, max_rhs);

        for (k, v) in self.dictionary.iter() {
            let pad_zeroes: Vec<u128> = (0..num_variables - k.len())
                .map(|_| 0)
                .collect();
            let new_key = [k.as_slice(), pad_zeroes.as_slice()].concat();
            dictionary.insert(new_key, *v);
        }

        for (k, v) in rhs.dictionary.iter() {
            let pad_zeroes: Vec<u128> = (0..num_variables - k.len())
                .map(|_| 0)
                .collect();
            let new_key = &[k.as_slice(), pad_zeroes.as_slice()].concat();

            if dictionary.contains_key(new_key) {
                let new_val = dictionary[new_key] + *v;
                dictionary.insert(new_key.to_owned(), new_val);
            } else {
                dictionary.insert(new_key.to_owned(), v.to_owned());
            }
        }

        MPolynomial { dictionary }
    }
}

impl ops::Mul for MPolynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut dictionary = HashMap::new();
        let max_self = self
            .dictionary
            .keys()
            .into_iter()
            .map(|k| k.len())
            .max()
            .unwrap();
        let max_rhs = rhs
            .dictionary
            .keys()
            .into_iter()
            .map(|k| k.len())
            .max()
            .unwrap();
        let num_variables = max(max_self, max_rhs);

        for (k0, v0) in self.dictionary.iter() {
            for (k1, v1) in rhs.dictionary.iter() {
                let mut exponent: Vec<u128> =
                    (0..num_variables).map(|_| 0).collect();

                for k in 0..k0.len() {
                    exponent[k] += k0[k];
                }
                for k in 0..k1.len() {
                    exponent[k] += k1[k];
                }

                if dictionary.contains_key(&exponent) {
                    let new_val = dictionary[&exponent] + v0.to_owned() * v1.to_owned();
                    dictionary.insert(exponent, new_val);
                } else {
                    let new_val = v0.to_owned() * v1.to_owned();
                    dictionary.insert(exponent, new_val);
                }
            }
        }

        MPolynomial { dictionary }
    }
}

impl ops::Sub for MPolynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Neg for MPolynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut dictionary = HashMap::new();
        for (k, v) in self.dictionary.iter() {
            dictionary.insert(k.to_owned(), -(v.to_owned()));
        }

        MPolynomial { dictionary }
    }
}

impl ops::BitXor for MPolynomial {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            MPolynomial {
                dictionary: HashMap::new(),
            }
        } else {
            let pair = self.dictionary.iter().next().unwrap();
            let field = pair.1.field;
            let num_variables = pair.0.len();
            let exp: Vec<u128> = (0..num_variables).map(|_| 0).collect();

            let mut dictionary = HashMap::new();
            dictionary.insert(exp.to_owned(), field.one());
            let mut acc = MPolynomial { dictionary };

            for val in exp.iter().skip(2) {
                acc = acc.clone() * acc;
                if *val == 1 {
                    acc = acc * self.clone();
                }
            }

            acc
        }
    }
}

impl MPolynomial {
    pub fn new(self, dictionary: HashMap<Vec<u128>, FieldElement>) -> Self {
        MPolynomial { dictionary }
    }

    pub fn constant(&self, element: FieldElement) -> Self {
        let mut dictionary = HashMap::new();
        dictionary.insert(vec![0], element);

        MPolynomial { dictionary }
    }

    pub fn is_zero(&self) -> bool {
        if self.dictionary.is_empty() {
            true
        } else {
            for v in self.dictionary.values() {
                if v.is_zero() == false {
                    return false
                }
            }

            true
        }
    }

    pub fn variables(&self, num_variables: u128, field: &Field) -> Vec<Self> {
        todo!()
    }
}
