use crate::{algebra::{Field, FieldElement}, polynomials::Polynomial};

mod algebra;
mod polynomials;

fn main() {
    let p: u128 = 1 + 407 * ( 1 << 119 );
    let field = Field::new(p);

    let a = FieldElement::new(1, field);
    let b = FieldElement::new(2, field);
    let c = FieldElement::new(3, field);
    let d = FieldElement::new(4, field);
    let e = FieldElement::new(5, field);

    let f_coefficients = [a, b, c, d, e].to_vec();
    let g_coefficients = [a, b, c].to_vec();
    let f_x = Polynomial::new(f_coefficients);
    let g_x = Polynomial::new(g_coefficients);

    println!("{:?}", f_x + g_x);
}
