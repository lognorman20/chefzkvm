use crate::algebra::{Field, FieldElement};

mod algebra;

fn main() {
    let field = Field::new(5);
    let a = FieldElement::new(10, field);
    let b = FieldElement::new(3, field);
    let res = a + b;
    println!("{:?}", res);
}
