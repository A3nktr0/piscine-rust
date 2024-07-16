use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar: Sized + Add + Sub + Mul + Div {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

macro_rules! impl_scalar {
	($($t:ty),*) => { //expects zero or more type tokens (ty) separated by commas
		$(impl Scalar for $t {
			type Item = $t;
			fn zero() -> Self::Item{
				0 as $t
			}

			fn one() -> Self::Item{
				1 as $t
			}
		})* // should be repeated zero or more times
	}
}

impl_scalar!{u32, u64, i32, i64, f32, f64}