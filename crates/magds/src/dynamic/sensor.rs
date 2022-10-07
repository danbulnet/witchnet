use std::{
    fmt::{ Display, Formatter, Result as FmtResult },
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use enum_as_inner::EnumAsInner;

use witchnet_common::{
    neuron::{ Neuron, NeuronID },
    sensor::Sensor,
    data::{ DataType, DataTypeValue, DataCategory }
};

#[derive(EnumAsInner)]
pub enum SensorConatiner {
    Bool(Box<dyn Sensor<bool>>),
    U8(Box<dyn Sensor<u8>>),
    U16(Box<dyn Sensor<u16>>),
    U32(Box<dyn Sensor<u32>>),
    U64(Box<dyn Sensor<u64>>),
    U128(Box<dyn Sensor<u128>>),
    USize(Box<dyn Sensor<usize>>),
    I8(Box<dyn Sensor<i8>>),
    I16(Box<dyn Sensor<i16>>),
    I32(Box<dyn Sensor<i32>>),
    I64(Box<dyn Sensor<i64>>),
    I128(Box<dyn Sensor<i128>>),
    ISize(Box<dyn Sensor<isize>>),
    F32(Box<dyn Sensor<f32>>),
    F64(Box<dyn Sensor<f64>>),
    RcStr(Box<dyn Sensor<Rc<str>>>),
    String(Box<dyn Sensor<String>>)
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
            SensorConatiner::RcStr(v) => write!(f, "{v}"),
            SensorConatiner::String(v) => write!(f, "{v}"),
        }
    }
}

impl Sensor<DataTypeValue> for SensorConatiner {
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
            SensorConatiner::RcStr(v) => v.id(),
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
            SensorConatiner::RcStr(v) => v.data_type(),
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
            SensorConatiner::RcStr(v) => v.data_category(),
            SensorConatiner::String(v) => v.data_category()
        }
    }

    fn insert(&mut self, item: &DataTypeValue) -> Rc<RefCell<dyn Neuron>> {
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
            SensorConatiner::RcStr(v) => {
                v.insert(item.as_rc_str().unwrap())
            },
            SensorConatiner::String(v) => {
                v.insert(item.as_string().unwrap())
            }
        }
    }

    fn search(&self, item: &DataTypeValue) -> Option<Rc<RefCell<dyn Neuron>>> {
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
            SensorConatiner::RcStr(v) => {
                v.search(item.as_rc_str()?)
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
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String> {
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
            SensorConatiner::RcStr(v) => {
                v.activate(
                    item.as_rc_str().unwrap(), signal, propagate_horizontal, propagate_vertical
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
    ) -> Result<(), String> {
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
            SensorConatiner::RcStr(v) => {
                v.deactivate(
                    item.as_rc_str().unwrap(), propagate_horizontal, propagate_vertical
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
            SensorConatiner::RcStr(v) => v.deactivate_sensor(),
            SensorConatiner::String(v) => v.deactivate_sensor()
        }
    }
}

impl From<Box<dyn Sensor<bool>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<bool>>) -> SensorConatiner {
        SensorConatiner::Bool(sensor)
    }
}

impl From<Box<dyn Sensor<i8>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<i8>>) -> SensorConatiner {
        SensorConatiner::I8(sensor)
    }
}

impl From<Box<dyn Sensor<i16>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<i16>>) -> SensorConatiner {
        SensorConatiner::I16(sensor)
    }
}

impl From<Box<dyn Sensor<i32>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<i32>>) -> SensorConatiner {
        SensorConatiner::I32(sensor)
    }
}

impl From<Box<dyn Sensor<i64>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<i64>>) -> SensorConatiner {
        SensorConatiner::I64(sensor)
    }
}

impl From<Box<dyn Sensor<i128>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<i128>>) -> SensorConatiner {
        SensorConatiner::I128(sensor)
    }
}

impl From<Box<dyn Sensor<isize>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<isize>>) -> SensorConatiner {
        SensorConatiner::ISize(sensor)
    }
}

impl From<Box<dyn Sensor<u8>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<u8>>) -> SensorConatiner {
        SensorConatiner::U8(sensor)
    }
}

impl From<Box<dyn Sensor<u16>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<u16>>) -> SensorConatiner {
        SensorConatiner::U16(sensor)
    }
}

impl From<Box<dyn Sensor<u32>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<u32>>) -> SensorConatiner {
        SensorConatiner::U32(sensor)
    }
}

impl From<Box<dyn Sensor<u64>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<u64>>) -> SensorConatiner {
        SensorConatiner::U64(sensor)
    }
}

impl From<Box<dyn Sensor<u128>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<u128>>) -> SensorConatiner {
        SensorConatiner::U128(sensor)
    }
}

impl From<Box<dyn Sensor<usize>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<usize>>) -> SensorConatiner {
        SensorConatiner::USize(sensor)
    }
}

impl From<Box<dyn Sensor<f32>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<f32>>) -> SensorConatiner {
        SensorConatiner::F32(sensor)
    }
}

impl From<Box<dyn Sensor<f64>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<f64>>) -> SensorConatiner {
        SensorConatiner::F64(sensor)
    }
}

impl From<Box<dyn Sensor<Rc<str>>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<Rc<str>>>) -> SensorConatiner {
        SensorConatiner::RcStr(sensor)
    }
}

impl From<Box<dyn Sensor<String>>> for SensorConatiner {
    fn from(sensor: Box<dyn Sensor<String>>) -> SensorConatiner {
        SensorConatiner::String(sensor)
    }
}