use trait_vec::{self, tvec, AlgebraTrait, Expr, Vector};

fn main() {
    let a = tvec![1, 2, 3];
    let b = tvec![4, 5, 6];

    let c = a.add(b).to_vec();
    println!("{:?}", c);
}