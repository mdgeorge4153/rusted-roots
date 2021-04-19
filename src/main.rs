
trait OpBinaryHetero {
    type I1;
    type I2;
    type O;

    fn apply(a: Self::I1, b: Self::I2) -> Self::O;
}

trait OpBinary : OpBinaryHetero
where
    Self : OpBinaryHetero<I1 = <Self as OpBinary>::T>,
    Self : OpBinaryHetero<I2 = <Self as OpBinary>::T>,
    Self : OpBinaryHetero<O  = <Self as OpBinary>::T>,
{
    type T;
}

trait Relation : OpBinaryHetero
where
    Self : OpBinaryHetero<I1 = <Self as Relation>::T>,
    Self : OpBinaryHetero<I2 = <Self as Relation>::T>,
    Self : OpBinaryHetero<O  = bool>,
{
    type T;
}

trait Symmetric : Relation
where
    <Self as Relation>::T : Clone
{
    fn check(a: &Self::T, b: &Self::T) -> bool {
        Self::apply(a.clone(), b.clone()) == Self::apply(b.clone(), a.clone())
    }
}

trait Reflexive : Relation
where
    <Self as Relation>::T : Clone,
{
    fn check(a: &Self::T) -> bool {
        Self::apply(a.clone(), a.clone())
    }
}

trait Transitive : Relation
where
    <Self as Relation>::T : Clone,
{
    fn check(a: &Self::T, b : &Self::T, c : &Self::T) -> bool {
        if Self::apply(a.clone(), b.clone()) && Self::apply(b.clone(), c.clone()) {
            Self::apply(a.clone(), c.clone())
        } else {
            true
        }
    }
}

trait Equivalence : Symmetric + Reflexive + Transitive
where
    <Self as Relation>::T : Clone
{
    fn check(a: &Self::T, b: &Self::T, c : &Self::T) -> bool {
        <Self as Reflexive>::check(a) &&
        <Self as Symmetric>::check(a,b) &&
        <Self as Transitive>::check(a,b,c)
    }
}

/*
trait OpCommutative<Equality = StandardEquality<<Self as OpBinary>::O>>
where
    Self : OpBinary<I1 = <Self as OpBinary>::I2>,
    Equality : OpBinary<I1 = Self::O, I2 = Self::O, O = bool>,
    <Self as OpBinary>::I1 : Clone,
{
    fn check(a: &Self::I1, b: &Self::I2) -> bool {
        Equality::apply(Self::apply(a.clone(),b.clone()) , Self::apply(b.clone(),a.clone()))
    }
}

trait OpAssociative<Equality = StandardEquality<<Self as OpBinary>::O>>
where
    Self : OpBinary<I1 = <Self as OpBinary>::I2, O = <Self as OpBinary>::I2>,
    Equality : OpBinary<I1 = Self::O, I2 = Self::O, O = bool>,
    <Self as OpBinary>::I1 : Clone,
{
    fn check(a: &Self::I1, b: &Self::I2) -> bool {
        Equality::apply(Self::apply(a.clone(),b.clone()) , Self::apply(b.clone(),a.clone()))
    }
}


trait BinaryRelation<T> : OpBinary<I1 = T, I2 = T, O = bool> {}
impl <O : OpBinary<I1 = T, I2 = T, O = bool>> BinaryRelation<O> {}

/** Standard equality *********************************************************/

struct StandardEquality<T : Eq> { phantom: std::marker::PhantomData<T> }

impl <T : Eq> OpBinary for StandardEquality<T> {
    type I1 = T;
    type I2 = T;
    type O  = bool;

    fn apply(a: T, b: T) -> bool { a == b }
}

impl <T : Eq + Clone> OpCommutative for StandardEquality<T> {
}



/** Standard addition *********************************************************/

struct StandardAddition<L : std::ops::Add<R>, R> { phantom: std::marker::PhantomData<(L,R)> }

impl <L : std::ops::Add<R>,R> OpBinary for StandardAddition<L,R> {
    type I1 = L;
    type I2 = R;
    type O  = <L as std::ops::Add<R>>::Output;

    fn apply(a: L, b: R) -> Self::O { a + b }
}

impl <T> OpCommutative for StandardAddition<T,T>
where
    T : std::ops::Add<T> + Clone,
    <T as std::ops::Add<T>>::Output : Eq
{
}

*/

fn main() {
    println!("hello world");
}

