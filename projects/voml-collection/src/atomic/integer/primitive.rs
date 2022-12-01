use super::*;
use num_traits::{FromPrimitive, ToPrimitive};

macro_rules! from_integer {
    ($T:ty) => {
        impl From<$T> for Integer {
            fn from(value: $T) -> Self {
                Self { hint: "".to_string(), value: IBig::from(value) }
            }
        }
    };
    ($($T:ty), +) => {
        $(from_integer!($T);)+
    };
}

impl FromPrimitive for Integer {
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(Integer { hint: "".to_string(), value: IBig::from(n) })
    }
    #[inline]
    fn from_f32(_: f32) -> Option<Self> {
        todo!()
    }
    #[inline]
    fn from_f64(_: f64) -> Option<Self> {
        todo!()
    }
}

impl ToPrimitive for Integer {
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.value.to_isize()
    }
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.value.to_i8()
    }
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.value.to_i16()
    }
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.value.to_i32()
    }
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.value.to_i128()
    }
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.value.to_usize()
    }
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.value.to_u8()
    }
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.value.to_u16()
    }
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.value.to_u32()
    }
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        self.value.to_u128()
    }
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        todo!()
    }
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        todo!()
    }
}

from_integer![u8, u16, u32, u64, u128, usize];
from_integer![i8, i16, i32, i64, i128, isize];
