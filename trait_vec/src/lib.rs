pub trait Expr<T> {
    fn eval(&self, index: usize) -> T;
    fn len(&self) -> usize;
    fn to_vec(&self) -> Vector<T> { // Change return type to Vector<T>
        let mut result = Vector::<T>::new();
        for i in 0..self.len() {
            result.push(self.eval(i));
        }
        return result;
    }
}

#[derive(Debug)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: Clone> Expr<T> for Vector<T> {
    fn eval(&self, index: usize) -> T {
        self.data[index].clone()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

pub struct AddExpr<L, R> {
    left: L,
    right: R,
}

impl<T, L: Expr<T>, R: Expr<T>> Expr<T> for AddExpr<L, R>
where
    T: std::ops::Add<Output = T>,
{
    fn eval(&self, index: usize) -> T {
        return self.left.eval(index) + self.right.eval(index);
    }

    fn len(&self) -> usize {
        self.left.len()
    }
}

pub trait AlgebraTrait<R> {
    type OutputAdd;
    fn add(self, right: R) -> Self::OutputAdd;
    type OutputSub;
    fn sub(self, right: R) -> Self::OutputSub;
}

impl<L, R> AlgebraTrait<R> for L {
    type OutputAdd = AddExpr<L, R>;
    type OutputSub = SubExpr<L, R>;

    fn add(self, rhs: R) -> Self::OutputAdd {
        AddExpr {
            left: self,
            right: rhs,
        }
    }

    fn sub(self, right: R) -> Self::OutputSub {
        SubExpr {
            left: self,
            right,
        }
    }
}

// can't do this due to the orphan rule
// -> the std::ops::Add trait is defined outside of this trait, and
// R is not a local type.
// The orphan rules says:
// -> either the trait is local, and the type is known
// -> the type is local and the
// impl<L, R> std::ops::Add<R> for L {
//     type Output = AddExpr<L, R>;
//
//     fn add(self, rhs: R) -> Self::Output {
//         AddExpr {
//             left: self,
//             right: rhs,
//         }
//     }
// }

pub struct SubExpr<L, R> {
    left: L,
    right: R,
}
impl<T, L, R> Expr<T> for SubExpr<L, R>
where
    L: Expr<T>,
    R: Expr<T>,
    T: std::ops::Sub<Output = T>,
{
    fn eval(&self, index: usize) -> T {
        assert!(self.left.len() == self.right.len());
        self.left.eval(index) - self.right.eval(index)
    }

    fn len(&self) -> usize {
        assert!(self.left.len() == self.right.len());
        self.left.len()
    }

}

#[macro_export]
macro_rules! tvec {
    ( $( $x:expr ),* ) => {
        {
            let mut vec = Vector::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
}
