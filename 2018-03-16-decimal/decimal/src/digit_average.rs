extern crate num_integer;
extern crate num_traits;

use std::ops::AddAssign;
use self::num_traits::identities::{One, Zero};
use self::num_integer::Integer;

///A less restrictive reimplementation of `num::range`
pub struct Range<T> {
    state: T,
    stop: T,
    one: T,
}

impl<T> Range<T>
where
    T: One,
{
    pub fn new(start: T, stop: T) -> Range<T> {
        Range {
            state: start,
            stop,
            one: T::one(),
        }
    }
}

impl<T> Range<T>
where
    T: Zero + One,
{
    pub fn upper(stop: T) -> Range<T> {
        Range::new(T::zero(), stop)
    }
}

impl<T> Iterator for Range<T>
where
    T: PartialOrd + AddAssign + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.state < self.stop {
            let result = self.state.clone();
            self.state += self.one.clone();
            Some(result)
        } else {
            None
        }
    }
}

///Sum the base digits of numer/denom up to precision (with rounding)
pub fn div_sum<T, U>(numer: T, denom: &T, precision: U, base: &T) -> (T, bool)
where
    T: Integer + AddAssign + Clone,
    U: Zero + One + PartialOrd + AddAssign + Clone,
{
    let mut total = T::zero();
    let mut rem = numer;
    let mut last_zero = false;

    for _ in Range::upper(precision) {
        let (q, r) = rem.div_rem(denom);
        last_zero = q == T::zero(); // last_zero = (q == T::zero()), if that's clearer
        total += q;
        rem = base.clone() * r;
    }

    if (T::one() + T::one()) * rem.div_floor(denom) >= *base {
        total += T::one()
    }

    (total, last_zero)
}

pub struct AvgDec<T> {
    base: T,
    one: T,
    zero: T,
    numer: T,
    denom: T,
}

impl<T> AvgDec<T>
where
    T: Zero + One,
{
    pub fn new(base: T) -> AvgDec<T> {
        AvgDec {
            base,
            one: T::one(),
            zero: T::zero(),
            numer: T::zero(),
            denom: T::one(),
        }
    }
}

impl<T> Iterator for AvgDec<T>
where
    T: Integer + AddAssign + Clone,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<(T, T)> {
        loop {
            if self.numer < self.denom.clone() * self.base.clone() {
                let (sum, last_zero) = div_sum(
                    self.numer.clone(),
                    &self.denom,
                    self.denom.clone(),
                    &self.base,
                );

                if sum == self.numer && !last_zero {
                    let result = Some((self.numer.clone(), self.denom.clone()));
                    self.numer += self.one.clone();
                    return result;
                }

                self.numer += self.one.clone();
            } else {
                self.numer = self.zero.clone();
                self.denom += self.one.clone();
            }
        }
    }
}
