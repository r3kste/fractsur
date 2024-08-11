use std::{fmt, ops};

#[derive(Clone, Copy, Debug)]
pub struct Frac {
    numerator: i128,
    denominator: i128,
}

impl Frac {
    fn gcd(a: i128, b: i128) -> i128 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn new(numerator: i128, denominator: i128) -> Self {
        Frac {
            numerator,
            denominator,
        }
    }

    pub fn new_reduced(numerator: i128, denominator: i128) -> Self {
        let g = Self::gcd(numerator, denominator);
        Self::new(numerator / g, denominator / g)
    }

    pub fn pow(self, exp: u32) -> Self {
        Self::new(self.numerator.pow(exp), self.denominator.pow(exp))
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl ops::Neg for Frac {
    type Output = Frac;

    fn neg(self) -> Self::Output {
        Frac::new(-self.numerator, self.denominator)
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }
}

impl Eq for Frac {}

impl From<Frac> for f64 {
    fn from(value: Frac) -> Self {
        (value.numerator as f64) / (value.denominator as f64)
    }
}

impl ops::Add<Frac> for Frac {
    type Output = Frac;

    fn add(self, rhs: Frac) -> Self::Output {
        let (a, b) = (self.numerator, self.denominator);
        let (c, d) = (rhs.numerator, rhs.denominator);

        Frac::new_reduced(a * d + b * c, b * d)
    }
}

impl ops::AddAssign<Frac> for Frac {
    fn add_assign(&mut self, rhs: Frac) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Frac> for Frac {
    type Output = Frac;

    fn sub(self, rhs: Frac) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::SubAssign<Frac> for Frac {
    fn sub_assign(&mut self, rhs: Frac) {
        *self = *self - rhs;
    }
}

impl ops::Mul<Frac> for Frac {
    type Output = Frac;

    fn mul(self, rhs: Frac) -> Self::Output {
        let (a, b) = (self.numerator, self.denominator);
        let (c, d) = (rhs.numerator, rhs.denominator);

        Frac::new_reduced(a * c, b * d)
    }
}

impl ops::MulAssign<Frac> for Frac {
    fn mul_assign(&mut self, rhs: Frac) {
        *self = *self * rhs;
    }
}

impl ops::Div<Frac> for Frac {
    type Output = Frac;

    fn div(self, rhs: Frac) -> Self::Output {
        let (a, b) = (self.numerator, self.denominator);
        let (c, d) = (rhs.numerator, rhs.denominator);

        Frac::new_reduced(a * d, b * c)
    }
}

impl ops::DivAssign<Frac> for Frac {
    fn div_assign(&mut self, rhs: Frac) {
        *self = *self / rhs;
    }
}

macro_rules! impl_ops {
    ($($t:ty)*) => {
        $(
            impl From<$t> for Frac {
                fn from(value: $t) -> Self {
                    Frac::new(value as i128, 1)
                }
            }

            impl ops::Add<$t> for Frac {
                type Output = Frac;

                fn add(self, rhs: $t) -> Self::Output {
                    self + Frac::from(rhs)
                }
            }

            impl ops::Add<Frac> for $t {
                type Output = Frac;

                fn add(self, rhs: Frac) -> Self::Output {
                    Frac::from(self) + rhs
                }
            }

            impl ops::AddAssign<$t> for Frac {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + Frac::from(rhs);
                }
            }

            impl ops::Sub<$t> for Frac {
                type Output = Frac;

                fn sub(self, rhs: $t) -> Self::Output {
                    self - Frac::from(rhs)
                }
            }

            impl ops::Sub<Frac> for $t {
                type Output = Frac;

                fn sub(self, rhs: Frac) -> Self::Output {
                    Frac::from(self) - rhs
                }
            }

            impl ops::SubAssign<$t> for Frac {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - Frac::from(rhs);
                }
            }

            impl ops::Mul<$t> for Frac {
                type Output = Frac;

                fn mul(self, rhs: $t) -> Self::Output {
                    self * Frac::from(rhs)
                }
            }

            impl ops::Mul<Frac> for $t {
                type Output = Frac;

                fn mul(self, rhs: Frac) -> Self::Output {
                    Frac::from(self) * rhs
                }
            }

            impl ops::MulAssign<$t> for Frac {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * Frac::from(rhs);
                }
            }

            impl ops::Div<$t> for Frac {
                type Output = Frac;

                fn div(self, rhs: $t) -> Self::Output {
                    self / Frac::from(rhs)
                }
            }

            impl ops::Div<Frac> for $t {
                type Output = Frac;

                fn div(self, rhs: Frac) -> Self::Output {
                    Frac::from(self) / rhs
                }
            }

            impl ops::DivAssign<$t> for Frac {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / Frac::from(rhs);
                }
            }
        )*
    };
}

impl_ops!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128);
