use std::sync::Arc;

use num_traits::ToPrimitive;

use DistanceChecked::*;

#[derive(Debug, Copy, Clone)]
pub enum DistanceChecked {
    Comparable(f64),
    Incomparable
}

pub trait Distance {
    fn distance(&self, v: &Self) -> f64;
    fn distance_checked(&self, v: &Self) -> DistanceChecked;
}

macro_rules! impl_distance_numerical {
    ( $($t:ty),* ) => {
        $( impl Distance for $t {
            fn distance(&self, v:&Self) -> f64 {
                unsafe {
                    let lhs = Self::to_f64(self).unwrap_unchecked();
                    let rhs = Self::to_f64(v).unwrap_unchecked();
                    (lhs - rhs).abs()
                }
            }

            fn distance_checked(&self, v:&Self) -> DistanceChecked {
                unsafe {
                    let lhs = Self::to_f64(self).unwrap_unchecked();
                    let rhs = Self::to_f64(v).unwrap_unchecked();
                    Comparable((lhs - rhs).abs())
                }
            }
        }) *
    }
}

impl_distance_numerical! { 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

macro_rules! impl_distance_categorical {
    ( $($t:ty),* ) => {
        $( impl Distance for $t {
            fn distance(&self, v:&Self) -> f64 {
                if *self == *v { 0.0 } else { 1.0 }
            }

            fn distance_checked(&self, v:&Self) -> DistanceChecked {
                if *self == *v { Comparable(0.0) } else { Incomparable }
            }
        }) *
    }
}

impl_distance_categorical! { Arc<str>, String, &str }

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::Distance;

    #[test]
    fn distance_f32() {
        let d = 32f32.distance(&33f32);
        assert!(d < 1.00001);
        assert!(d > 0.99999);

        let d = 33f32.distance(&32f32);
        assert!(d < 1.00001);
        assert!(d > 0.99999);
    }

    #[test]
    fn distance_u64() {
        let d = 32u64.distance(&33u64);
        assert!(d < 1.00001);
        assert!(d > 0.99999);

        let d = 33u64.distance(&32u64);
        assert!(d < 1.00001);
        assert!(d > 0.99999);
    }

    #[test]
    fn distance_isize() {
        let d = 32isize.distance(&33isize);
        assert!(d < 1.00001);
        assert!(d > 0.99999);

        let d = 33isize.distance(&32isize);
        assert!(d < 1.00001);
        assert!(d > 0.99999);
    }

    #[test]
    fn distance_str() {
        assert_eq!("a".distance(&"a"), 0.0);
        assert_eq!("a".distance(&"b"), 1.0);
        assert_eq!("b".distance(&"a"), 1.0);
    }
}