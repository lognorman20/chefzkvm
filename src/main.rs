mod algebra;

fn main() {
    let res = algebra::xgcd(23891,89);
    println!("{:?}", res);
}
