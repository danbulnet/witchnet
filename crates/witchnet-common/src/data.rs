use std::{
    sync::Arc,
    marker::PhantomData,
    fmt::{ Display, Formatter, Result as FmtResult }
};

use regex::Regex;

use enum_as_inner::EnumAsInner;

use crate::{
    distances::{ 
        Distance, 
        DistanceChecked::{ self, * }
    }
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumAsInner)]
pub enum DataCategory {
    Numerical,
    Categorical,
    Ordinal,
}

macro_rules! impl_numerical {
    ( $($t:ty),* ) => {
        $( impl From<&$t> for DataCategory {
            fn from(_data: &$t) -> DataCategory { DataCategory::Numerical }
        }
        impl From<&[$t]> for DataCategory {
            fn from(_data: &[$t]) -> DataCategory { DataCategory::Numerical }
        }
        impl From<&[Option<$t>]> for DataCategory {
            fn from(_data: &[Option<$t>]) -> DataCategory { DataCategory::Numerical }
        }) *
    }
}

macro_rules! impl_categorical {
    ( $($t:ty),* ) => {
        $( impl From<&$t> for DataCategory {
            fn from(_data: &$t) -> DataCategory { DataCategory::Categorical }
        }
        impl From<&[$t]> for DataCategory {
            fn from(_data: &[$t]) -> DataCategory { DataCategory::Categorical }
        }
        impl From<&[Option<$t>]> for DataCategory {
            fn from(_data: &[Option<$t>]) -> DataCategory { DataCategory::Categorical }
        }) *
    }
}

impl_numerical! { 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

impl_categorical! { String, Arc<str>, bool }

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumAsInner)]
pub enum DataType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    F32,
    F64,
    ArcStr,
    String,
    Unknown
}

#[derive(EnumAsInner, Clone, Debug, PartialEq, PartialOrd)]
pub enum DataTypeValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    F32(f32),
    F64(f64),
    ArcStr(Arc<str>),
    String(String),
    Unknown
}

impl DataTypeValue {
    pub fn is_type_same_as(&self, other: &DataTypeValue) -> bool {
        DataType::from(self) ==  DataType::from(other)
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            DataTypeValue::Bool(_) => None,
            DataTypeValue::U8(v) => Some(*v as f64),
            DataTypeValue::U16(v) => Some(*v as f64),
            DataTypeValue::U32(v) => Some(*v as f64),
            DataTypeValue::U64(v) => Some(*v as f64),
            DataTypeValue::U128(v) => Some(*v as f64),
            DataTypeValue::USize(v) => Some(*v as f64),
            DataTypeValue::I8(v) => Some(*v as f64),
            DataTypeValue::I16(v) => Some(*v as f64),
            DataTypeValue::I32(v) => Some(*v as f64),
            DataTypeValue::I64(v) => Some(*v as f64),
            DataTypeValue::I128(v) => Some(*v as f64),
            DataTypeValue::ISize(v) => Some(*v as f64),
            DataTypeValue::F32(v) => Some(*v as f64),
            DataTypeValue::F64(v) => Some(*v as f64),
            DataTypeValue::ArcStr(_) => None,
            DataTypeValue::String(_) => None,
            DataTypeValue::Unknown => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DataTypeValue::Bool(v) => v.to_string(),
            DataTypeValue::U8(v) => v.to_string(),
            DataTypeValue::U16(v) => v.to_string(),
            DataTypeValue::U32(v) => v.to_string(),
            DataTypeValue::U64(v) => v.to_string(),
            DataTypeValue::U128(v) => v.to_string(),
            DataTypeValue::USize(v) => v.to_string(),
            DataTypeValue::I8(v) => v.to_string(),
            DataTypeValue::I16(v) => v.to_string(),
            DataTypeValue::I32(v) => v.to_string(),
            DataTypeValue::I64(v) => v.to_string(),
            DataTypeValue::I128(v) => v.to_string(),
            DataTypeValue::ISize(v) => v.to_string(),
            DataTypeValue::F32(v) => v.to_string(),
            DataTypeValue::F64(v) => v.to_string(),
            DataTypeValue::ArcStr(v) => v.to_string(),
            DataTypeValue::String(v) => v.clone(),
            DataTypeValue::Unknown => String::from("")
        }
    }

    pub fn to_vec(&self) -> Vec<DataTypeValue> {
        match self {
            DataTypeValue::Bool(v) => vec![DataTypeValue::Bool(*v)],
            DataTypeValue::U8(v) => vec![DataTypeValue::U8(*v)],
            DataTypeValue::U16(v) => vec![DataTypeValue::U16(*v)],
            DataTypeValue::U32(v) => vec![DataTypeValue::U32(*v)],
            DataTypeValue::U64(v) => vec![DataTypeValue::U64(*v)],
            DataTypeValue::U128(v) => vec![DataTypeValue::U128(*v)],
            DataTypeValue::USize(v) => vec![DataTypeValue::USize(*v)],
            DataTypeValue::I8(v) => vec![DataTypeValue::I8(*v)],
            DataTypeValue::I16(v) => vec![DataTypeValue::I16(*v)],
            DataTypeValue::I32(v) => vec![DataTypeValue::I32(*v)],
            DataTypeValue::I64(v) => vec![DataTypeValue::I64(*v)],
            DataTypeValue::I128(v) => vec![DataTypeValue::I128(*v)],
            DataTypeValue::ISize(v) => vec![DataTypeValue::ISize(*v)],
            DataTypeValue::F32(v) => vec![DataTypeValue::F32(*v)],
            DataTypeValue::F64(v) => vec![DataTypeValue::F64(*v)],
            DataTypeValue::ArcStr(key) => {
                let key = key.to_string();
                if key.starts_with("[") && key.ends_with("]") {
                    let key = key.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
                    Regex::new(r"\s*,\s*")
                        .unwrap()
                        .split(key)
                        .map(|x| {
                            let string = Regex::new(r#"["']+"#).unwrap()
                                .split(x)
                                .filter(|x| *x != "")
                                .next()
                                .unwrap();
                            DataTypeValue::String(string.into())
                        }).collect()
                } else {
                    vec![DataTypeValue::ArcStr(key.into())]
                }
            },
            DataTypeValue::String(key) => {
                if key.starts_with("[") && key.ends_with("]") {
                    let key = key.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
                    Regex::new(r"\s*,\s*")
                        .unwrap()
                        .split(key)
                        .map(|x| {
                            let string = Regex::new(r#"["']+"#).unwrap()
                                .split(x)
                                .filter(|x| *x != "")
                                .next()
                                .unwrap()
                                .to_string();
                            DataTypeValue::String(string)
                        }).collect()
                } else {
                    vec![DataTypeValue::String(key.clone())]
                }
            },
            DataTypeValue::Unknown => vec![]
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Display for DataTypeValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl From<&DataTypeValue> for DataCategory {
    fn from(data: &DataTypeValue) -> DataCategory {
        match data {
            DataTypeValue::Bool(_) => DataCategory::Categorical,
            DataTypeValue::U8(_) => DataCategory::Numerical,
            DataTypeValue::U16(_) => DataCategory::Numerical,
            DataTypeValue::U32(_) => DataCategory::Numerical,
            DataTypeValue::U64(_) => DataCategory::Numerical,
            DataTypeValue::U128(_) => DataCategory::Numerical,
            DataTypeValue::USize(_) => DataCategory::Numerical,
            DataTypeValue::I8(_) => DataCategory::Numerical,
            DataTypeValue::I16(_) => DataCategory::Numerical,
            DataTypeValue::I32(_) => DataCategory::Numerical,
            DataTypeValue::I64(_) => DataCategory::Numerical,
            DataTypeValue::I128(_) => DataCategory::Numerical,
            DataTypeValue::ISize(_) => DataCategory::Numerical,
            DataTypeValue::F32(_) => DataCategory::Numerical,
            DataTypeValue::F64(_) => DataCategory::Numerical,
            DataTypeValue::ArcStr(_) => DataCategory::Categorical,
            DataTypeValue::String(_) => DataCategory::Categorical,
            DataTypeValue::Unknown => DataCategory::Categorical
        }
    }
}

impl From<&DataTypeValue> for DataType {
    fn from(data: &DataTypeValue) -> DataType {
        match data {
            DataTypeValue::Bool(_) => DataType::Bool,
            DataTypeValue::U8(_) => DataType::U8,
            DataTypeValue::U16(_) => DataType::U16,
            DataTypeValue::U32(_) => DataType::U32,
            DataTypeValue::U64(_) => DataType::U64,
            DataTypeValue::U128(_) => DataType::U128,
            DataTypeValue::USize(_) => DataType::USize,
            DataTypeValue::I8(_) => DataType::I8,
            DataTypeValue::I16(_) => DataType::I16,
            DataTypeValue::I32(_) => DataType::I32,
            DataTypeValue::I64(_) => DataType::I64,
            DataTypeValue::I128(_) => DataType::I128,
            DataTypeValue::ISize(_) => DataType::ISize,
            DataTypeValue::F32(_) => DataType::F32,
            DataTypeValue::F64(_) => DataType::F64,
            DataTypeValue::ArcStr(_) => DataType::ArcStr,
            DataTypeValue::String(_) => DataType::String,
            DataTypeValue::Unknown => DataType::Unknown
        }
    }
}

impl Distance for DataTypeValue {
    fn distance(&self, v: &DataTypeValue) -> f64 {
        match self {
            DataTypeValue::Bool(lhs) => {
                let rhs = match v.as_bool() { Some(v) => v, None => return 1.0 };
                if *lhs == *rhs { 0.0 } else { 1.0 }
            }
            DataTypeValue::U8(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::U16(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::U32(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::U64(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::U128(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::USize(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::I8(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::I16(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::I32(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::I64(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::I128(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::ISize(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::F32(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::F64(lhs) => {
                (*lhs as f64 - v.to_f64().unwrap()).abs()
            }
            DataTypeValue::ArcStr(lhs) => {
                match v.as_arc_str() { 
                    Some(rhs) => if *lhs == *rhs { 0.0 } else { 1.0 },
                    None => match v.as_string() {
                        Some(rhs) => if lhs.to_string() == *rhs { 0.0 } else { 1.0 },
                        None => 1.0
                    }
                }
            }
            DataTypeValue::String(lhs) => {
                match v.as_string() { 
                    Some(rhs) => if *lhs == *rhs { 0.0 } else { 1.0 },
                    None => match v.as_arc_str() {
                        Some(rhs) => if *lhs == rhs.to_string() { 0.0 } else { 1.0 },
                        None => 1.0
                    }
                }
            }
            DataTypeValue::Unknown => f64::NAN
        }
    }

    fn distance_checked(&self, v: &DataTypeValue) -> DistanceChecked {
        match self {
            DataTypeValue::Bool(lhs) => {
                let rhs = match v.as_bool() { Some(v) => v, None => return Incomparable };
                if *lhs == *rhs { Comparable(0.0) } else { Incomparable }
            }
            DataTypeValue::U8(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::U16(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::U32(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::U64(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::U128(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::USize(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::I8(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::I16(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::I32(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::I64(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::I128(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::ISize(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::F32(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::F64(lhs) => {
                Comparable((*lhs as f64 - v.to_f64().unwrap()).abs())
            }
            DataTypeValue::ArcStr(lhs) => {
                match v.as_arc_str() { 
                    Some(rhs) => if *lhs == *rhs { Comparable(0.0) } else { Incomparable },
                    None => match v.as_string() {
                        Some(rhs) => {
                            if lhs.to_string() == *rhs { Comparable(0.0) } else { Incomparable }
                        }
                        None => Incomparable
                    }
                }
            }
            DataTypeValue::String(lhs) => {
                match v.as_string() { 
                    Some(rhs) => if *lhs == *rhs { Comparable(0.0) } else { Incomparable },
                    None => match v.as_arc_str() {
                        Some(rhs) => {
                            if *lhs == rhs.to_string() { Comparable(0.0) } else { Incomparable }
                        }
                        None => Incomparable
                    }
                }
            }
            DataTypeValue::Unknown => Incomparable
        }
    }
}

impl From<bool> for DataTypeValue { 
    fn from(v: bool) -> DataTypeValue { DataTypeValue::Bool(v) } 
}

impl From<u8> for DataTypeValue { 
    fn from(v: u8) -> DataTypeValue { DataTypeValue::U8(v) } 
}

impl From<u16> for DataTypeValue { 
    fn from(v: u16) -> DataTypeValue { DataTypeValue::U16(v) } 
}

impl From<u32> for DataTypeValue { 
    fn from(v: u32) -> DataTypeValue { DataTypeValue::U32(v) } 
}

impl From<u64> for DataTypeValue { 
    fn from(v: u64) -> DataTypeValue { DataTypeValue::U64(v) } 
}

impl From<u128> for DataTypeValue { 
    fn from(v: u128) -> DataTypeValue { DataTypeValue::U128(v) } 
}

impl From<usize> for DataTypeValue { 
    fn from(v: usize) -> DataTypeValue { DataTypeValue::USize(v) } 
}

impl From<i8> for DataTypeValue { 
    fn from(v: i8) -> DataTypeValue { DataTypeValue::I8(v) } 
}

impl From<i16> for DataTypeValue { 
    fn from(v: i16) -> DataTypeValue { DataTypeValue::I16(v) } 
}

impl From<i32> for DataTypeValue { 
    fn from(v: i32) -> DataTypeValue { DataTypeValue::I32(v) } 
}

impl From<i64> for DataTypeValue { 
    fn from(v: i64) -> DataTypeValue { DataTypeValue::I64(v) } 
}

impl From<i128> for DataTypeValue { 
    fn from(v: i128) -> DataTypeValue { DataTypeValue::I128(v) } 
}

impl From<isize> for DataTypeValue { 
    fn from(v: isize) -> DataTypeValue { DataTypeValue::ISize(v) } 
}

impl From<f32> for DataTypeValue { 
    fn from(v: f32) -> DataTypeValue { DataTypeValue::F32(v) } 
}

impl From<f64> for DataTypeValue { 
    fn from(v: f64) -> DataTypeValue { DataTypeValue::F64(v) } 
}

impl From<Arc<str>> for DataTypeValue { 
    fn from(v: Arc<str>) -> DataTypeValue { DataTypeValue::ArcStr(v) } 
}

impl From<String> for DataTypeValue { 
    fn from(v: String) -> DataTypeValue { DataTypeValue::String(v) } 
}

impl From<DataTypeValue> for Option<bool> { 
    fn from(v: DataTypeValue) -> Option<bool> { v.into_bool().ok() } 
}

impl From<DataTypeValue> for Option<u8> { 
    fn from(v: DataTypeValue) -> Option<u8> { v.into_u8().ok() } 
}

impl From<DataTypeValue> for Option<u16> { 
    fn from(v: DataTypeValue) -> Option<u16> { v.into_u16().ok() } 
}

impl From<DataTypeValue> for Option<u32> { 
    fn from(v: DataTypeValue) -> Option<u32> { v.into_u32().ok() } 
}

impl From<DataTypeValue> for Option<u64> { 
    fn from(v: DataTypeValue) -> Option<u64> { v.into_u64().ok() } 
}

impl From<DataTypeValue> for Option<u128> { 
    fn from(v: DataTypeValue) -> Option<u128> { v.into_u128().ok() } 
}

impl From<DataTypeValue> for Option<usize> { 
    fn from(v: DataTypeValue) -> Option<usize> { v.into_u_size().ok() } 
}

impl From<DataTypeValue> for Option<i8> { 
    fn from(v: DataTypeValue) -> Option<i8> { v.into_i8().ok() } 
}

impl From<DataTypeValue> for Option<i16> { 
    fn from(v: DataTypeValue) -> Option<i16> { v.into_i16().ok() } 
}

impl From<DataTypeValue> for Option<i32> { 
    fn from(v: DataTypeValue) -> Option<i32> { v.into_i32().ok() } 
}

impl From<DataTypeValue> for Option<i64> { 
    fn from(v: DataTypeValue) -> Option<i64> { v.into_i64().ok() } 
}

impl From<DataTypeValue> for Option<i128> { 
    fn from(v: DataTypeValue) -> Option<i128> { v.into_i128().ok() } 
}

impl From<DataTypeValue> for Option<isize> { 
    fn from(v: DataTypeValue) -> Option<isize> { v.into_i_size().ok() } 
}

impl From<DataTypeValue> for Option<f32> { 
    fn from(v: DataTypeValue) -> Option<f32> { v.into_f32().ok() } 
}

impl From<DataTypeValue> for Option<f64> { 
    fn from(v: DataTypeValue) -> Option<f64> { v.into_f64().ok() } 
}

impl From<DataTypeValue> for Option<Arc<str>> { 
    fn from(v: DataTypeValue) -> Option<Arc<str>> { v.into_arc_str().ok() } 
}

impl From<DataTypeValue> for Option<String> { 
    fn from(v: DataTypeValue) -> Option<String> { v.into_string().ok() } 
}

pub struct DataTypeValueStr<'a>(pub &'a str);

impl<'a> DataTypeValueStr<'a> {
    pub fn data_type_value(&self, data_type: DataType) -> Option<DataTypeValue> {
        let result = match data_type {
            DataType::Bool => DataTypeValue::Bool(self.0.parse().ok()?),
            DataType::U8 => DataTypeValue::U8(self.0.parse().ok()?),
            DataType::U16 => DataTypeValue::U16(self.0.parse().ok()?),
            DataType::U32 => DataTypeValue::U32(self.0.parse().ok()?),
            DataType::U64 => DataTypeValue::U64(self.0.parse().ok()?),
            DataType::U128 => DataTypeValue::U128(self.0.parse().ok()?),
            DataType::USize => DataTypeValue::USize(self.0.parse().ok()?),
            DataType::I8 => DataTypeValue::I8(self.0.parse().ok()?),
            DataType::I16 => DataTypeValue::I16(self.0.parse().ok()?),
            DataType::I32 => DataTypeValue::I32(self.0.parse().ok()?),
            DataType::I64 => DataTypeValue::I64(self.0.parse().ok()?),
            DataType::I128 => DataTypeValue::I128(self.0.parse().ok()?),
            DataType::ISize => DataTypeValue::ISize(self.0.parse().ok()?),
            DataType::F32 => DataTypeValue::F32(self.0.parse().ok()?),
            DataType::F64 => DataTypeValue::F64(self.0.parse().ok()?),
            DataType::ArcStr => DataTypeValue::ArcStr(self.0.into()),
            DataType::String => DataTypeValue::String(self.0.parse().ok()?),
            DataType::Unknown => return None
        };
        Some(result)
    }
}

pub auto trait UnknownDataTypeMarker {}

impl !UnknownDataTypeMarker for bool {}
impl !UnknownDataTypeMarker for u8 {}
impl !UnknownDataTypeMarker for u16 {}
impl !UnknownDataTypeMarker for u32 {}
impl !UnknownDataTypeMarker for u64 {}
impl !UnknownDataTypeMarker for u128 {}
impl !UnknownDataTypeMarker for usize {}
impl !UnknownDataTypeMarker for i8 {}
impl !UnknownDataTypeMarker for i16 {}
impl !UnknownDataTypeMarker for i32 {}
impl !UnknownDataTypeMarker for i64 {}
impl !UnknownDataTypeMarker for i128 {}
impl !UnknownDataTypeMarker for isize {}
impl !UnknownDataTypeMarker for f32 {}
impl !UnknownDataTypeMarker for f64 {}
impl !UnknownDataTypeMarker for Arc<str> {}
impl !UnknownDataTypeMarker for String {}

impl !UnknownDataTypeMarker for PhantomData<bool> {}
impl !UnknownDataTypeMarker for PhantomData<u8> {}
impl !UnknownDataTypeMarker for PhantomData<u16> {}
impl !UnknownDataTypeMarker for PhantomData<u32> {}
impl !UnknownDataTypeMarker for PhantomData<u64> {}
impl !UnknownDataTypeMarker for PhantomData<u128> {}
impl !UnknownDataTypeMarker for PhantomData<usize> {}
impl !UnknownDataTypeMarker for PhantomData<i8> {}
impl !UnknownDataTypeMarker for PhantomData<i16> {}
impl !UnknownDataTypeMarker for PhantomData<i32> {}
impl !UnknownDataTypeMarker for PhantomData<i64> {}
impl !UnknownDataTypeMarker for PhantomData<i128> {}
impl !UnknownDataTypeMarker for PhantomData<isize> {}
impl !UnknownDataTypeMarker for PhantomData<f32> {}
impl !UnknownDataTypeMarker for PhantomData<f64> {}
impl !UnknownDataTypeMarker for PhantomData<Arc<str>> {}
impl !UnknownDataTypeMarker for PhantomData<String> {}

pub trait DataDeductor { 
    fn data_type(&self) -> DataType;
    fn data_category(&self) -> DataCategory;
 }

impl DataDeductor for bool {
    fn data_type(&self) -> DataType { DataType::Bool }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for u8 {
    fn data_type(&self) -> DataType { DataType::U8 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for u16 {
    fn data_type(&self) -> DataType { DataType::U16 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for u32 {
    fn data_type(&self) -> DataType { DataType::U32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for u64 {
    fn data_type(&self) -> DataType { DataType::U64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for usize {
    fn data_type(&self) -> DataType { DataType::U128 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for u128 {
    fn data_type(&self) -> DataType { DataType::USize }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i8 {
    fn data_type(&self) -> DataType { DataType::I8 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i16 {
    fn data_type(&self) -> DataType { DataType::I16 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i32 {
    fn data_type(&self) -> DataType { DataType::I32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i64 {
    fn data_type(&self) -> DataType { DataType::I64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i128 {
    fn data_type(&self) -> DataType { DataType::I128 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for isize {
    fn data_type(&self) -> DataType { DataType::ISize }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for f32 {
    fn data_type(&self) -> DataType { DataType::F32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for f64 {
    fn data_type(&self) -> DataType { DataType::F64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for Arc<str> {
    fn data_type(&self) -> DataType { DataType::ArcStr }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for String {
    fn data_type(&self) -> DataType { DataType::String }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for PhantomData<bool> {
    fn data_type(&self) -> DataType { DataType::Bool }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for PhantomData<u8> {
    fn data_type(&self) -> DataType { DataType::U8 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<u16> {
    fn data_type(&self) -> DataType { DataType::U16 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<u32> {
    fn data_type(&self) -> DataType { DataType::U32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<u64> {
    fn data_type(&self) -> DataType { DataType::U64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<usize> {
    fn data_type(&self) -> DataType { DataType::U128 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<u128> {
    fn data_type(&self) -> DataType { DataType::USize }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i8> {
    fn data_type(&self) -> DataType { DataType::I8 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i16> {
    fn data_type(&self) -> DataType { DataType::I16 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i32> {
    fn data_type(&self) -> DataType { DataType::I32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i64> {
    fn data_type(&self) -> DataType { DataType::I64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i128> {
    fn data_type(&self) -> DataType { DataType::I128 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<isize> {
    fn data_type(&self) -> DataType { DataType::ISize }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<f32> {
    fn data_type(&self) -> DataType { DataType::F32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<f64> {
    fn data_type(&self) -> DataType { DataType::F64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<Arc<str>> {
    fn data_type(&self) -> DataType { DataType::ArcStr }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for PhantomData<String> {
    fn data_type(&self) -> DataType { DataType::String }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn data_type_value_distance() {
        let x: DataTypeValue = 1.0f32.into();
        let y: DataTypeValue = 3.0f32.into();
        assert_eq!(x.distance(&y), 2.0f64);
        
        let x: DataTypeValue = 1.0f32.into();
        let y: DataTypeValue = 3i32.into();
        assert_eq!(x.distance(&y), 2.0f64);

        let x: DataTypeValue = 1.0f64.into();
        let y: DataTypeValue = 2usize.into();
        assert_eq!(x.distance(&y), 1.0f64);

        let x: DataTypeValue = "1.0f32".to_string().into();
        let y: DataTypeValue = "1.0f32".to_string().into();
        assert_eq!(x.distance(&y), 0.0f64);
        let y: DataTypeValue = "0.0f32".to_string().into();
        assert_eq!(x.distance(&y), 1.0f64);

        let x: DataTypeValue = "1.0f32".to_string().into();
        let y: DataTypeValue = Arc::<str>::from("1.0f32").into();
        assert_eq!(x.distance(&y), 0.0f64);
        let y: DataTypeValue = Arc::<str>::from("1.1f32").into();
        assert_eq!(x.distance(&y), 1.0f64);

        let x: DataTypeValue = 1.0f32.into();
        let y: DataTypeValue = 3.0f64.into();
        assert_eq!(x.distance(&y), 2.0f64);
        assert_eq!(y.distance(&x), 2.0f64);
        
    }
}