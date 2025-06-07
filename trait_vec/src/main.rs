use trait_vec::{self, tvec, AlgebraTrait, Expr, Vector, vec_expr, AddExpr, SubExpr};

fn main() {
    let a = tvec![1, 2, 3];
    let b = tvec![4, 5, 6];
    let c = tvec![4, 5, 6];

    let d = vec_expr!(a + b - c).to_vec();
    println!("{:?}", d);
}