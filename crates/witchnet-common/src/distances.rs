use std::rc::Rc;

use num_traits::ToPrimitive;

pub trait Distance {
    fn distance(&self, v: &Self) -> f64;
}

impl Distance for str {
    fn distance(&self, v: &str) -> f64  {
        if *self == *v { 0.0 } else { 1.0 }
    }
}

impl Distance for String {
    fn distance(&self, v: &String) -> f64  {
        if *self == *v { 0.0 } else { 1.0 }
    }
}

impl Distance for Rc<str> {
    fn distance(&self, v: &Rc<str>) -> f64  {
        if *self == *v { 0.0 } else { 1.0 }
    }
}

macro_rules! impl_distance {
    ( $($t:ty),* ) => {
        $( impl Distance for $t {
            fn distance(&self, v:&Self) -> f64 {
                unsafe { 
                    (Self::to_f64(self).unwrap_unchecked() - Self::to_f64(v).unwrap_unchecked()).abs()
                }
            }
        }) *
    }
}

impl_distance! { 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::Distance;

    #[test]
    fn distance_f32() {
        let d = 32f32.distance(&33f32);
        assert!(d < 1.00001);
        assert!(d > 0.99999);
    }

    #[test]
    fn distance_u64() {
        let d = 32u64.distance(&33u64);
        assert!(d < 1.00001);
        assert!(d > 0.99999);
    }

    #[test]
    fn distance_isize() {
        let d = 32isize.distance(&33isize);
        assert!(d < 1.00001);
        assert!(d > 0.99999);
    }

    #[test]
    fn distance_str() {
        assert_eq!("a".distance(&"a"), 0.0);
        assert_eq!("a".distance(&"b"), 1.0);
    }
}