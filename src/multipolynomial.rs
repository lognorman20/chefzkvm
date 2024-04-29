use std::cmp::max;
use std::collections::HashMap;
use std::{iter, ops};

use crate::algebra::FieldElement;

#[derive(Debug, Clone)]
pub struct MPolynomial {
    // vector of exponents : coefficients
    dictionary: HashMap<Vec<FieldElement>, FieldElement>,
}

impl ops::Add for MPolynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
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

        for (k, v) in self.dictionary.into_iter() {
            let pad_zeroes: Vec<FieldElement> = (0..num_variables - k.len())
                .map(|_| k[0].field.zero())
                .collect();
            let new_key = [k.as_slice(), pad_zeroes.as_slice()].concat();
            dictionary.insert(new_key, v);
        }

        for (k, v) in rhs.dictionary.into_iter() {
            let pad_zeroes: Vec<FieldElement> = (0..num_variables - k.len())
                .map(|_| k[0].field.zero())
                .collect();
            let new_key = &[k.as_slice(), pad_zeroes.as_slice()].concat();

            if dictionary.contains_key(new_key) {
                dictionary.insert(new_key.to_owned(), dictionary[new_key] + v);
            } else {
                dictionary.insert(new_key.to_owned(), v);
            }
        }

        MPolynomial { dictionary }
    }
}

impl ops::Mul for MPolynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Sub for MPolynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Neg for MPolynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl ops::BitXor for MPolynomial {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl MPolynomial {
    pub fn new(self, dictionary: HashMap<Vec<FieldElement>, FieldElement>) -> Self {
        MPolynomial { dictionary }
    }
}
