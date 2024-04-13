use crate::algebra::{xgcd, Field, FieldElement};

mod algebra;

fn main() {
    let p: u128 = 1 + 407 * ( 1 << 119 );
    let field = Field::new(p);
    println!("{:?}", field.primite_nth_root(1024));
}
