
pub trait OpBinaryHetero {
    type I1;
    type I2;
    type O;

    fn apply(a: Self::I1, b: Self::I2) -> Self::O;
}

/** Relations *****************************************************************/

pub trait Relation : OpBinaryHetero
where
    Self : OpBinaryHetero<I1 = <Self as Relation>::T>,
    Self : OpBinaryHetero<I2 = <Self as Relation>::T>,
    Self : OpBinaryHetero<O  = bool>,
{
    type T;
}

pub trait Symmetric : Relation
{
    fn check(a: &Self::T, b: &Self::T) -> bool
    where Self::T : Clone
    {
        Self::apply(a.clone(), b.clone()) == Self::apply(b.clone(), a.clone())
    }
}

pub trait Reflexive : Relation
{
    fn check(a: &Self::T) -> bool
    where Self::T : Clone
    {
        Self::apply(a.clone(), a.clone())
    }
}

pub trait Transitive : Relation
{
    fn check(a: &Self::T, b : &Self::T, c : &Self::T) -> bool
    where Self::T : Clone
    {
        if Self::apply(a.clone(), b.clone()) && Self::apply(b.clone(), c.clone()) {
            Self::apply(a.clone(), c.clone())
        } else {
            true
        }
    }
}

pub trait Equivalence : Symmetric + Reflexive + Transitive
{
    fn check(a: &Self::T, b: &Self::T, c : &Self::T) -> bool
    where Self::T : Clone
    {
        <Self as Reflexive>::check(a)      &&
        <Self as Symmetric>::check(a,b)    &&
        <Self as Transitive>::check(a,b,c)
    }
}

/** Standard equality *********************************************************/

struct StandardEquality<T : Eq> { phantom: std::marker::PhantomData<T> }

impl <T : Eq> OpBinaryHetero for StandardEquality<T> {
    type I1 = T;
    type I2 = T;
    type O  = bool;

    fn apply(a: T, b: T) -> bool { a == b }
}

impl <T : Eq> Relation for StandardEquality<T> {
    type T = T;
}

impl <T : Eq> Reflexive   for StandardEquality<T> { }
impl <T : Eq> Symmetric   for StandardEquality<T> { }
impl <T : Eq> Transitive  for StandardEquality<T> { }
impl <T : Eq> Equivalence for StandardEquality<T> { }

/** Modular equality **********************************************************/

struct  ModularI64Equiv<const MODULUS : i64>;

impl <const N : i64> OpBinaryHetero for ModularI64Equiv<N> {
    type I1 = i64;
    type I2 = i64;
    type O  = bool;

    fn apply(a: i64, b: i64) -> bool { a % N == b % N }
}

impl <const N : i64> Relation for ModularI64Equiv<N> {
    type T = i64;
}

impl <const N : i64> Reflexive   for ModularI64Equiv<N> { }
impl <const N : i64> Symmetric   for ModularI64Equiv<N> { }
impl <const N : i64> Transitive  for ModularI64Equiv<N> { }
impl <const N : i64> Equivalence for ModularI64Equiv<N> { }

/** Setoid ********************************************************************/

pub trait Setoid {
    type T;
    type Equiv : Equivalence<T = Self::T>;

    fn eq(a: Self::T, b: Self::T) -> bool {
        Self::Equiv::apply(a,b)
    }

    fn neq(a: Self::T, b: Self::T) -> bool {
        ! Self::Equiv::apply(a,b)
    }

    fn check(a: &Self::T, b: &Self::T, c: &Self::T) -> bool
    where Self::T : Clone
    {
        <Self::Equiv as Equivalence>::check(a,b,c)
    }
}

/** Binary operations *********************************************************/

pub trait OpBinary : OpBinaryHetero
where
    Self : OpBinaryHetero<I1 = <Self as OpBinary>::T>,
    Self : OpBinaryHetero<I2 = <Self as OpBinary>::T>,
    Self : OpBinaryHetero<O  = <Self as OpBinary>::T>,
{
    type T;
}

/** Magma wrapper *************************************************************/

struct MagmaWrapper<O : OpBinary + ?Sized, E : Equivalence<T = O::T>>
{
    value: O::T,
    _phantom: std::marker::PhantomData<E>,
}

impl <O : OpBinary + ?Sized, E : Equivalence<T = O::T>> MagmaWrapper<O,E>
{
    fn new(v: O::T) -> Self
    {
        MagmaWrapper { value: v, _phantom: std::marker::PhantomData }
    }

    fn inv(self) -> Self
    where O : HasInverses<E>
    {
        Self::new(O::inverse(self.value))
    }
}

impl <O, E> PartialEq for MagmaWrapper<O, E>
where
    O : OpBinary + ?Sized,
    E : Equivalence<T = O::T>,
    O::T : Clone
{
    fn eq(&self, other: &Self) -> bool {
        E::apply(self.value.clone(), other.value.clone())
    }
}

impl <O,E> std::ops::BitAnd for MagmaWrapper<O, E>
where
    O : OpBinary + ?Sized,
    E : Equivalence<T = O::T>,
    O::T : Clone
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        MagmaWrapper { value : O::apply(self.value.clone(), other.value.clone()), _phantom: std::marker::PhantomData }
    }
}

/** Marker traits for BinaryOps ***********************************************/

pub trait Commutative<Equiv> : OpBinary
where
    Equiv : Equivalence<T = <Self as OpBinary>::T>
{
    fn check(a: &Self::T, b: &Self::T) -> bool
    where Self::T : Clone
    {
        let w = |a : &Self::T| MagmaWrapper::<Self,Equiv>::new(a.clone());

        w(a) & w(b) == w(b) & w(a)

        // Equiv::apply(Self::apply(a.clone(),b.clone()) , Self::apply(b.clone(),a.clone()))
    }
}

pub trait Associative<Equiv> : OpBinary
where
    Equiv : Equivalence<T = <Self as OpBinary>::T>
{
    fn check(a: &Self::T, b: &Self::T, c: &Self::T) -> bool
    where Self::T : Clone
    {
        let w = |a : &Self::T| MagmaWrapper::<Self,Equiv>::new(a.clone());

        w(a) & (w(b) & w(c)) == (w(a) & w(b)) & w(c)
    }
}

pub trait HasIdentity<Equiv> : OpBinary
where
    Equiv : Equivalence<T = <Self as OpBinary>::T>
{
    const IDENTITY : Self::T;

    fn check(a: &Self::T) -> bool
    where Self::T : Clone
    {
        let w = |a : &Self::T| MagmaWrapper::<Self,Equiv>::new(a.clone());

        w(a) & w(&Self::IDENTITY) == w(a)
        &&
        w(&Self::IDENTITY) & w(a) == w(a)
    }
}

pub trait HasInverses<Equiv> : HasIdentity<Equiv>
where
    Equiv : Equivalence<T = <Self as OpBinary>::T>
{
    fn inverse(a: Self::T) -> Self::T;

    fn check(a: &Self::T) -> bool
    where Self::T : Clone
    {
        let w = |a : &Self::T| MagmaWrapper::<Self,Equiv>::new(a.clone());

        <Self as HasIdentity<Equiv>>::check(a)
        &&
        w(a) & w(a).inv() == w(&Self::IDENTITY)
        &&
        w(a).inv() & w(a) == w(&Self::IDENTITY)
    }
}

/** Group-like structures *****************************************************/

pub trait Magma : Setoid {
    type Op : OpBinary<T = Self::T>;
}

pub trait Semigroup : Magma
where Self::Op : Associative<Self::Equiv>
{}

// TODO: why do I need to repeat where clause from Semigroup here?  Should be inherited?
// 
// according to someone on discord, this is a shortcoming in the way the compiler checks where
// clauses; there's no fundamental problem.
pub trait Monoid : Semigroup
where Self::Op : Associative<Self::Equiv> + HasIdentity<Self::Equiv>
{}

pub trait Group : Monoid
where Self::Op : Associative<Self::Equiv> + HasIdentity<Self::Equiv> + HasInverses<Self::Equiv>
{}

pub trait GroupAbelian : Group
where Self::Op : Associative<Self::Equiv> + HasIdentity<Self::Equiv> + HasInverses<Self::Equiv> + Commutative<Self::Equiv>
{}

/** Standard addition *********************************************************/

struct StandardAddition<L : std::ops::Add<R>, R> { phantom: std::marker::PhantomData<(L,R)> }

impl <L : std::ops::Add<R>,R> OpBinaryHetero for StandardAddition<L,R> {
    type I1 = L;
    type I2 = R;
    type O  = <L as std::ops::Add<R>>::Output;

    fn apply(a: L, b: R) -> Self::O { a + b }
}

impl <T> OpBinary for StandardAddition<T,T>
where
    T : std::ops::Add<T,Output=T>
{
    type T = T;
}

impl <T> Commutative<StandardEquality<T>> for StandardAddition<T,T>
where
    T : std::ops::Add<T,Output=T>,
    T : Eq
{}

impl <T> Associative<StandardEquality<T>> for StandardAddition<T,T>
where
    T : std::ops::Add<T,Output=T>,
    T : Eq
{}

/*
trait Zero : std::ops::Add<Self,Output = Self> {
    const ZERO : Self;
}

impl <T> HasIdentity<StandardEquality<T>> for StandardAddition<T,T>
where
    T : Zero,
    T : Eq,
{
    const IDENTITY : T = T::ZERO;
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

