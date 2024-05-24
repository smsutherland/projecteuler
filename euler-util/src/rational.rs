use std::ops::{Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Rational<T> {
    pub numerator: T,
    pub denominator: T,
}

impl<T> Rational<T> {
    pub fn new(numerator: T, denominator: T) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl Rational<u64> {
    pub fn reduce(&mut self) {
        let gcd = super::gcd(self.numerator, self.denominator);

        self.numerator /= gcd;
        self.denominator /= gcd;
    }
}

impl PartialEq for Rational<u64> {
    fn eq(&self, other: &Self) -> bool {
        let mut a = *self;
        let mut o = *other;

        a.reduce();
        o.reduce();

        a.numerator == o.numerator && a.denominator == o.denominator
    }
}

impl Eq for Rational<u64> {}

impl<T> Mul for Rational<T>
where
    T: Mul,
{
    type Output = Rational<<T as Mul>::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl<T> MulAssign for Rational<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
    }
}
