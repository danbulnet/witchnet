use std::{
    fmt::{ Display, Formatter, Result as FmtResult },
    sync::{ Arc, RwLock}
};

use anyhow::Result;

use enum_as_inner::EnumAsInner;

use witchnet_common::{
    neuron::NeuronAsync,
    sensor::SensorAsync,
    data::{ DataType, DataTypeValue, DataCategory }, 
    connection::collective::defining::DefiningWeightingStrategyAsync
};

#[derive(EnumAsInner, Debug)]
pub enum SensorConatiner {
    Bool(Box<dyn SensorAsync<bool>>),
    U8(Box<dyn SensorAsync<u8>>),
    U16(Box<dyn SensorAsync<u16>>),
    U32(Box<dyn SensorAsync<u32>>),
    U64(Box<dyn SensorAsync<u64>>),
    U128(Box<dyn SensorAsync<u128>>),
    USize(Box<dyn SensorAsync<usize>>),
    I8(Box<dyn SensorAsync<i8>>),
    I16(Box<dyn SensorAsync<i16>>),
    I32(Box<dyn SensorAsync<i32>>),
    I64(Box<dyn SensorAsync<i64>>),
    I128(Box<dyn SensorAsync<i128>>),
    ISize(Box<dyn SensorAsync<isize>>),
    F32(Box<dyn SensorAsync<f32>>),
    F64(Box<dyn SensorAsync<f64>>),
    ArcStr(Box<dyn SensorAsync<Arc<str>>>),
    String(Box<dyn SensorAsync<String>>)
}

impl Display for SensorConatiner {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SensorConatiner::Bool(v) => write!(f, "{v}"),
            SensorConatiner::U8(v) => write!(f, "{v}"),
            SensorConatiner::U16(v) => write!(f, "{v}"),
            SensorConatiner::U32(v) => write!(f, "{v}"),
            SensorConatiner::U64(v) => write!(f, "{v}"),
            SensorConatiner::U128(v) => write!(f, "{v}"),
            SensorConatiner::USize(v) => write!(f, "{v}"),
            SensorConatiner::I8(v) => write!(f, "{v}"),
            SensorConatiner::I16(v) => write!(f, "{v}"),
            SensorConatiner::I32(v) => write!(f, "{v}"),
            SensorConatiner::I64(v) => write!(f, "{v}"),
            SensorConatiner::I128(v) => write!(f, "{v}"),
            SensorConatiner::ISize(v) => write!(f, "{v}"),
            SensorConatiner::F32(v) => write!(f, "{v}"),
            SensorConatiner::F64(v) => write!(f, "{v}"),
            SensorConatiner::ArcStr(v) => write!(f, "{v}"),
            SensorConatiner::String(v) => write!(f, "{v}"),
        }
    }
}

impl SensorAsync<DataTypeValue> for SensorConatiner {
    fn id(&self) -> u32 {
        match self {
            SensorConatiner::Bool(v) => v.id(),
            SensorConatiner::U8(v) => v.id(),
            SensorConatiner::U16(v) => v.id(),
            SensorConatiner::U32(v) => v.id(),
            SensorConatiner::U64(v) => v.id(),
            SensorConatiner::U128(v) => v.id(),
            SensorConatiner::USize(v) => v.id(),
            SensorConatiner::I8(v) => v.id(),
            SensorConatiner::I16(v) => v.id(),
            SensorConatiner::I32(v) => v.id(),
            SensorConatiner::I64(v) => v.id(),
            SensorConatiner::I128(v) => v.id(),
            SensorConatiner::ISize(v) => v.id(),
            SensorConatiner::F32(v) => v.id(),
            SensorConatiner::F64(v) => v.id(),
            SensorConatiner::ArcStr(v) => v.id(),
            SensorConatiner::String(v) => v.id()
        }
    }

    fn data_type(&self) -> DataType {
        match self {
            SensorConatiner::Bool(v) => v.data_type(),
            SensorConatiner::U8(v) => v.data_type(),
            SensorConatiner::U16(v) => v.data_type(),
            SensorConatiner::U32(v) => v.data_type(),
            SensorConatiner::U64(v) => v.data_type(),
            SensorConatiner::U128(v) => v.data_type(),
            SensorConatiner::USize(v) => v.data_type(),
            SensorConatiner::I8(v) => v.data_type(),
            SensorConatiner::I16(v) => v.data_type(),
            SensorConatiner::I32(v) => v.data_type(),
            SensorConatiner::I64(v) => v.data_type(),
            SensorConatiner::I128(v) => v.data_type(),
            SensorConatiner::ISize(v) => v.data_type(),
            SensorConatiner::F32(v) => v.data_type(),
            SensorConatiner::F64(v) => v.data_type(),
            SensorConatiner::ArcStr(v) => v.data_type(),
            SensorConatiner::String(v) => v.data_type()
        }
    }

    fn data_category(&self) -> DataCategory {
        match self {
            SensorConatiner::Bool(v) => v.data_category(),
            SensorConatiner::U8(v) => v.data_category(),
            SensorConatiner::U16(v) => v.data_category(),
            SensorConatiner::U32(v) => v.data_category(),
            SensorConatiner::U64(v) => v.data_category(),
            SensorConatiner::U128(v) => v.data_category(),
            SensorConatiner::USize(v) => v.data_category(),
            SensorConatiner::I8(v) => v.data_category(),
            SensorConatiner::I16(v) => v.data_category(),
            SensorConatiner::I32(v) => v.data_category(),
            SensorConatiner::I64(v) => v.data_category(),
            SensorConatiner::I128(v) => v.data_category(),
            SensorConatiner::ISize(v) => v.data_category(),
            SensorConatiner::F32(v) => v.data_category(),
            SensorConatiner::F64(v) => v.data_category(),
            SensorConatiner::ArcStr(v) => v.data_category(),
            SensorConatiner::String(v) => v.data_category()
        }
    }

    fn insert(&mut self, item: &DataTypeValue) -> Arc<RwLock<dyn NeuronAsync>> {
        match self {
            SensorConatiner::Bool(v) => {
                v.insert(item.as_bool().unwrap())
            },
            SensorConatiner::U8(v) => {
                v.insert(item.as_u8().unwrap())
            },
            SensorConatiner::U16(v) => {
                v.insert(item.as_u16().unwrap())
            },
            SensorConatiner::U32(v) => {
                v.insert(item.as_u32().unwrap())
            },
            SensorConatiner::U64(v) => {
                v.insert(item.as_u64().unwrap())
            },
            SensorConatiner::U128(v) => {
                v.insert(item.as_u128().unwrap())
            },
            SensorConatiner::USize(v) => {
                v.insert(item.as_u_size().unwrap())
            },
            SensorConatiner::I8(v) => {
                v.insert(item.as_i8().unwrap())
            },
            SensorConatiner::I16(v) => {
                v.insert(item.as_i16().unwrap())
            },
            SensorConatiner::I32(v) => {
                v.insert(item.as_i32().unwrap())
            },
            SensorConatiner::I64(v) => {
                v.insert(item.as_i64().unwrap())
            },
            SensorConatiner::I128(v) => {
                v.insert(item.as_i128().unwrap())
            },
            SensorConatiner::ISize(v) => {
                v.insert(item.as_i_size().unwrap())
            },
            SensorConatiner::F32(v) => {
                v.insert(item.as_f32().unwrap())
            },
            SensorConatiner::F64(v) => {
                v.insert(item.as_f64().unwrap())
            },
            SensorConatiner::ArcStr(v) => {
                v.insert(item.as_arc_str().unwrap())
            },
            SensorConatiner::String(v) => {
                v.insert(item.as_string().unwrap())
            }
        }
    }
    
    fn insert_custom(
        &mut self, 
        item: &DataTypeValue, 
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Arc<RwLock<dyn NeuronAsync>> {
        match self {
            SensorConatiner::Bool(v) => {
                v.insert_custom(
                    item.as_bool().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::U8(v) => {
                v.insert_custom(
                    item.as_u8().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::U16(v) => {
                v.insert_custom(
                    item.as_u16().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::U32(v) => {
                v.insert_custom(
                    item.as_u32().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::U64(v) => {
                v.insert_custom(
                    item.as_u64().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::U128(v) => {
                v.insert_custom(
                    item.as_u128().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::USize(v) => {
                v.insert_custom(
                    item.as_u_size().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::I8(v) => {
                v.insert_custom(
                    item.as_i8().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::I16(v) => {
                v.insert_custom(
                    item.as_i16().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::I32(v) => {
                v.insert_custom(
                    item.as_i32().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::I64(v) => {
                v.insert_custom(
                    item.as_i64().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::I128(v) => {
                v.insert_custom(
                    item.as_i128().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::ISize(v) => {
                v.insert_custom(
                    item.as_i_size().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::F32(v) => {
                v.insert_custom(
                    item.as_f32().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::F64(v) => {
                v.insert_custom(
                    item.as_f64().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::ArcStr(v) => {
                v.insert_custom(
                    item.as_arc_str().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            },
            SensorConatiner::String(v) => {
                v.insert_custom(
                    item.as_string().unwrap(), 
                    weighting_strategy,
                    interelement_activation_threshold,
                    interelement_activation_exponent
                )
            }
        }
    }

    fn fuzzy_search(
        &mut self, item: &DataTypeValue, threshold: f32
    ) -> Option<(Arc<RwLock<dyn NeuronAsync>>, f32)> {
        match self {
            SensorConatiner::Bool(v) => {
                match v.fuzzy_search(item.as_bool()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::U8(v) => {
                match v.fuzzy_search(item.as_u8()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::U16(v) => {
                match v.fuzzy_search(item.as_u16()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::U32(v) => {
                match v.fuzzy_search(item.as_u32()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::U64(v) => {
                match v.fuzzy_search(item.as_u64()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::U128(v) => {
                match v.fuzzy_search(item.as_u128()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::USize(v) => {
                match v.fuzzy_search(item.as_u_size()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::I8(v) => {
                match v.fuzzy_search(item.as_i8()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::I16(v) => {
                match v.fuzzy_search(item.as_i16()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::I32(v) => {
                match v.fuzzy_search(item.as_i32()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::I64(v) => {
                match v.fuzzy_search(item.as_i64()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::I128(v) => {
                match v.fuzzy_search(item.as_i128()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::ISize(v) => {
                match v.fuzzy_search(item.as_i_size()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::F32(v) => {
                match v.fuzzy_search(item.as_f32()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::F64(v) => {
                match v.fuzzy_search(item.as_f64()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::ArcStr(v) => {
                match v.fuzzy_search(item.as_arc_str()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            },
            SensorConatiner::String(v) => {
                match v.fuzzy_search(item.as_string()?, threshold) {
                    Some(n) => Some((n.0 as Arc<RwLock<dyn NeuronAsync>>, n.1)),
                    None => None
                }
            }
        }
    }

    fn search(&self, item: &DataTypeValue) -> Option<Arc<RwLock<dyn NeuronAsync>>> {
        match self {
            SensorConatiner::Bool(v) => {
                v.search(item.as_bool()?)
            },
            SensorConatiner::U8(v) => {
                v.search(item.as_u8()?)
            },
            SensorConatiner::U16(v) => {
                v.search(item.as_u16()?)
            },
            SensorConatiner::U32(v) => {
                v.search(item.as_u32()?)
            },
            SensorConatiner::U64(v) => {
                v.search(item.as_u64()?)
            },
            SensorConatiner::U128(v) => {
                v.search(item.as_u128()?)
            },
            SensorConatiner::USize(v) => {
                v.search(item.as_u_size()?)
            },
            SensorConatiner::I8(v) => {
                v.search(item.as_i8()?)
            },
            SensorConatiner::I16(v) => {
                v.search(item.as_i16()?)
            },
            SensorConatiner::I32(v) => {
                v.search(item.as_i32()?)
            },
            SensorConatiner::I64(v) => {
                v.search(item.as_i64()?)
            },
            SensorConatiner::I128(v) => {
                v.search(item.as_i128()?)
            },
            SensorConatiner::ISize(v) => {
                v.search(item.as_i_size()?)
            },
            SensorConatiner::F32(v) => {
                v.search(item.as_f32()?)
            },
            SensorConatiner::F64(v) => {
                v.search(item.as_f64()?)
            },
            SensorConatiner::ArcStr(v) => {
                v.search(item.as_arc_str()?)
            },
            SensorConatiner::String(v) => {
                v.search(item.as_string()?)
            }
        }
    }

    fn activate(
        &mut self, 
        item: &DataTypeValue, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<f32> {
        match self {
            SensorConatiner::Bool(v) => {
                v.activate(
                    item.as_bool().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U8(v) => {
                v.activate(
                    item.as_u8().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U16(v) => {
                v.activate(
                    item.as_u16().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U32(v) => {
                v.activate(
                    item.as_u32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U64(v) => {
                v.activate(
                    item.as_u64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U128(v) => {
                v.activate(
                    item.as_u128().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::USize(v) => {
                v.activate(
                    item.as_u_size().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I8(v) => {
                v.activate(
                    item.as_i8().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I16(v) => {
                v.activate(
                    item.as_i16().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I32(v) => {
                v.activate(
                    item.as_i32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I64(v) => {
                v.activate(
                    item.as_i64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I128(v) => {
                v.activate(
                    item.as_i128().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::ISize(v) => {
                v.activate(
                    item.as_i_size().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F32(v) => {
                v.activate(
                    item.as_f32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F64(v) => {
                v.activate(
                    item.as_f64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::ArcStr(v) => {
                v.activate(
                    item.as_arc_str().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::String(v) => {
                v.activate(
                    item.as_string().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            }
        }
    }

    fn deactivate(
        &mut self, 
        item: &DataTypeValue, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<()> {
        match self {
            SensorConatiner::Bool(v) => {
                v.deactivate(
                    item.as_bool().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U8(v) => {
                v.deactivate(
                    item.as_u8().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U16(v) => {
                v.deactivate(
                    item.as_u16().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U32(v) => {
                v.deactivate(
                    item.as_u32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U64(v) => {
                v.deactivate(
                    item.as_u64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U128(v) => {
                v.deactivate(
                    item.as_u128().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::USize(v) => {
                v.deactivate(
                    item.as_u_size().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I8(v) => {
                v.deactivate(
                    item.as_i8().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I16(v) => {
                v.deactivate(
                    item.as_i16().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I32(v) => {
                v.deactivate(
                    item.as_i32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I64(v) => {
                v.deactivate(
                    item.as_i64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I128(v) => {
                v.deactivate(
                    item.as_i128().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::ISize(v) => {
                v.deactivate(
                    item.as_i_size().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F32(v) => {
                v.deactivate(
                    item.as_f32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F64(v) => {
                v.deactivate(
                    item.as_f64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::ArcStr(v) => {
                v.deactivate(
                    item.as_arc_str().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::String(v) => {
                v.deactivate(
                    item.as_string().unwrap(), propagate_horizontal, propagate_vertical
                )
            }
        }
    }

    fn deactivate_sensor(&mut self) {
        match self {
            SensorConatiner::Bool(v) => v.deactivate_sensor(),
            SensorConatiner::U8(v) => v.deactivate_sensor(),
            SensorConatiner::U16(v) => v.deactivate_sensor(),
            SensorConatiner::U32(v) => v.deactivate_sensor(),
            SensorConatiner::U64(v) => v.deactivate_sensor(),
            SensorConatiner::U128(v) => v.deactivate_sensor(),
            SensorConatiner::USize(v) => v.deactivate_sensor(),
            SensorConatiner::I8(v) => v.deactivate_sensor(),
            SensorConatiner::I16(v) => v.deactivate_sensor(),
            SensorConatiner::I32(v) => v.deactivate_sensor(),
            SensorConatiner::I64(v) => v.deactivate_sensor(),
            SensorConatiner::I128(v) => v.deactivate_sensor(),
            SensorConatiner::ISize(v) => v.deactivate_sensor(),
            SensorConatiner::F32(v) => v.deactivate_sensor(),
            SensorConatiner::F64(v) => v.deactivate_sensor(),
            SensorConatiner::ArcStr(v) => v.deactivate_sensor(),
            SensorConatiner::String(v) => v.deactivate_sensor()
        }
    }

    fn neurons(&self) -> Vec<Arc<RwLock<dyn NeuronAsync>>> {
        match self {
            SensorConatiner::Bool(v) => v.neurons(),
            SensorConatiner::U8(v) => v.neurons(),
            SensorConatiner::U16(v) => v.neurons(),
            SensorConatiner::U32(v) => v.neurons(),
            SensorConatiner::U64(v) => v.neurons(),
            SensorConatiner::U128(v) => v.neurons(),
            SensorConatiner::USize(v) => v.neurons(),
            SensorConatiner::I8(v) => v.neurons(),
            SensorConatiner::I16(v) => v.neurons(),
            SensorConatiner::I32(v) => v.neurons(),
            SensorConatiner::I64(v) => v.neurons(),
            SensorConatiner::I128(v) => v.neurons(),
            SensorConatiner::ISize(v) => v.neurons(),
            SensorConatiner::F32(v) => v.neurons(),
            SensorConatiner::F64(v) => v.neurons(),
            SensorConatiner::ArcStr(v) => v.neurons(),
            SensorConatiner::String(v) => v.neurons()
        }
    }

    fn values(&self) -> Vec<DataTypeValue> {
        match self {
            SensorConatiner::Bool(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::U8(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::U16(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::U32(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::U64(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::U128(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::USize(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::I8(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::I16(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::I32(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::I64(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::I128(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::ISize(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::F32(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::F64(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::ArcStr(v) => v.values().into_iter().map(|x| x.into()).collect(),
            SensorConatiner::String(v) => v.values().into_iter().map(|x| x.into()).collect()
        }
    }
}

impl From<Box<dyn SensorAsync<bool>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<bool>>) -> SensorConatiner {
        SensorConatiner::Bool(sensor)
    }
}

impl From<Box<dyn SensorAsync<i8>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<i8>>) -> SensorConatiner {
        SensorConatiner::I8(sensor)
    }
}

impl From<Box<dyn SensorAsync<i16>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<i16>>) -> SensorConatiner {
        SensorConatiner::I16(sensor)
    }
}

impl From<Box<dyn SensorAsync<i32>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<i32>>) -> SensorConatiner {
        SensorConatiner::I32(sensor)
    }
}

impl From<Box<dyn SensorAsync<i64>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<i64>>) -> SensorConatiner {
        SensorConatiner::I64(sensor)
    }
}

impl From<Box<dyn SensorAsync<i128>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<i128>>) -> SensorConatiner {
        SensorConatiner::I128(sensor)
    }
}

impl From<Box<dyn SensorAsync<isize>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<isize>>) -> SensorConatiner {
        SensorConatiner::ISize(sensor)
    }
}

impl From<Box<dyn SensorAsync<u8>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<u8>>) -> SensorConatiner {
        SensorConatiner::U8(sensor)
    }
}

impl From<Box<dyn SensorAsync<u16>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<u16>>) -> SensorConatiner {
        SensorConatiner::U16(sensor)
    }
}

impl From<Box<dyn SensorAsync<u32>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<u32>>) -> SensorConatiner {
        SensorConatiner::U32(sensor)
    }
}

impl From<Box<dyn SensorAsync<u64>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<u64>>) -> SensorConatiner {
        SensorConatiner::U64(sensor)
    }
}

impl From<Box<dyn SensorAsync<u128>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<u128>>) -> SensorConatiner {
        SensorConatiner::U128(sensor)
    }
}

impl From<Box<dyn SensorAsync<usize>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<usize>>) -> SensorConatiner {
        SensorConatiner::USize(sensor)
    }
}

impl From<Box<dyn SensorAsync<f32>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<f32>>) -> SensorConatiner {
        SensorConatiner::F32(sensor)
    }
}

impl From<Box<dyn SensorAsync<f64>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<f64>>) -> SensorConatiner {
        SensorConatiner::F64(sensor)
    }
}

impl From<Box<dyn SensorAsync<Arc<str>>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<Arc<str>>>) -> SensorConatiner {
        SensorConatiner::ArcStr(sensor)
    }
}

impl From<Box<dyn SensorAsync<String>>> for SensorConatiner {
    fn from(sensor: Box<dyn SensorAsync<String>>) -> SensorConatiner {
        SensorConatiner::String(sensor)
    }
}