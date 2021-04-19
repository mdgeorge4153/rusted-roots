extern crate alga;

#[macro_use]
extern crate alga_derive;

use alga::general::*;
use std::ops::{Add,Mul};

#[derive(Clone,Alga,Debug,Copy)]
#[alga_traits(Field(Additive,Multiplicative))]
struct Unit;

impl <O : Operator> Identity<O> for Unit {
    fn identity() -> Unit { Unit }
}

impl <O : Operator> TwoSidedInverse<O> for Unit {
    fn two_sided_inverse(&self) -> Unit { Unit }
}

impl PartialEq for Unit {
    fn eq(&self, _b: &Unit) -> bool { true }
}

impl <O : Operator> AbstractMagma<O> for Unit {
    fn operate(&self, _b: &Unit) -> Unit { Unit }
}

impl Add for Unit {
    type Output = Self;
    fn add(self, _b: Self) -> Self { Unit }
}

/** Fractions *****************************************************************/

#[derive(Clone,Alga)]
#[alga_traits(Field(Additive,Multiplicative), Where = "T : RingCommutative + Copy")]
struct Fraction<T> {
    num: T,
    den: T,
}

impl <T : RingCommutative + Add + Mul + Copy> AbstractMagma<Additive> for Fraction<T> {
    fn operate(&self, b: &Self) -> Self {
        Fraction { num: self.num * b.den + self.den * b.num, den: self.den * b.den }
    }
}

impl <T : RingCommutative + Add + Mul + Copy> AbstractMagma<Multiplicative> for Fraction<T> {
    fn operate(&self, b: &Self) -> Self {
        Fraction { num: self.num * b.num, den: self.den * b.den }
    }
}

impl <T : RingCommutative + Copy> PartialEq for Fraction<T> {
    fn eq(&self, b: &Self) -> bool {
        self.num * b.den == self.den * b.num
    }
}

impl <T : RingCommutative> Identity<Additive> for Fraction<T> {
    fn identity() -> Self {
        Fraction {
            num: <T as Identity<Additive>>::identity(),
            den: <T as Identity<Multiplicative>>::identity()
        }
    }
}

impl <T : RingCommutative + Copy> Identity<Multiplicative> for Fraction<T> {
    fn identity() -> Self {
        Fraction {
            num: <T as Identity<Multiplicative>>::identity(),
            den: <T as Identity<Multiplicative>>::identity()
        }
    }
}

impl <T : RingCommutative + Copy> TwoSidedInverse<Additive> for Fraction<T> {
    fn two_sided_inverse(&self) -> Self {
        Fraction {
            num: <T as TwoSidedInverse<Additive>>::two_sided_inverse(&self.num),
            den: self.den,
        }
    }
}

impl <T : Copy> TwoSidedInverse<Multiplicative> for Fraction<T> {
    fn two_sided_inverse(&self) -> Self {
        Fraction { num: self.den, den: self.num }
    }
}



fn main() {
    let a = Unit;
    println!("{:?}", a + a);
}
