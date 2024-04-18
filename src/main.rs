use bigint::U256;

use crate::{
    algebra::{Field, FieldElement},
    polynomials::Polynomial,
};

mod algebra;
mod polynomials;

fn main() {
    let p: U256 = U256::from_dec_str("270497897142230380135924736767050121217").unwrap();
    let field = Field::new(p);

    let a = FieldElement::new(U256::from(1), field);
    let b = FieldElement::new(U256::from(2), field);
    let c = FieldElement::new(U256::from(3), field);
    let d = FieldElement::new(U256::from(4), field);
    let e = FieldElement::new(U256::from(5), field);

    let f_coefficients = [a, b, c, d, e].to_vec();
    let g_coefficients = [a, b, c, d, e].to_vec();
    let f_x = Polynomial::new(f_coefficients);
    let g_x = Polynomial::new(g_coefficients);

    println!("{:#?}", f_x / g_x);
}
