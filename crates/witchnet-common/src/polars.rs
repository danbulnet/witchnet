use std::{
    fs::File,
    sync::Arc
};

use polars::prelude::*;

use crate::data::DataTypeValue;

pub enum DataVec {
    BoolVec(Vec<bool>),
    UInt8Vec(Vec<u8>),
    UInt16Vec(Vec<u16>),
    UInt32Vec(Vec<u32>),
    UInt64Vec(Vec<u64>),
    Int8Vec(Vec<i8>),
    Int16Vec(Vec<i16>),
    Int32Vec(Vec<i32>),
    Int64Vec(Vec<i64>),
    Float32Vec(Vec<f32>),
    Float64Vec(Vec<f64>),
    Utf8Vec(Vec<Arc<str>>),
    Unknown
}

impl DataVec {
    pub fn get(&self, index: usize) -> Option<DataTypeValue> {
        match self {
            DataVec::BoolVec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::UInt8Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::UInt16Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::UInt32Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::UInt64Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::Int8Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::Int16Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::Int32Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::Int64Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::Float32Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::Float64Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::Utf8Vec(v) => if let Some(x) = 
                v.get(index) {Some(x.clone().into()) } else { None },
            DataVec::Unknown => None
        }
    }
}

pub enum DataVecOption {
    BoolVec(Vec<Option<bool>>),
    UInt8Vec(Vec<Option<u8>>),
    UInt16Vec(Vec<Option<u16>>),
    UInt32Vec(Vec<Option<u32>>),
    UInt64Vec(Vec<Option<u64>>),
    Int8Vec(Vec<Option<i8>>),
    Int16Vec(Vec<Option<i16>>),
    Int32Vec(Vec<Option<i32>>),
    Int64Vec(Vec<Option<i64>>),
    Float32Vec(Vec<Option<f32>>),
    Float64Vec(Vec<Option<f64>>),
    Utf8Vec(Vec<Option<Arc<str>>>),
    Unknown
}

impl DataVecOption {
    pub fn get(&self, index: usize) -> Option<DataTypeValue> {
        match self {
            DataVecOption::BoolVec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::UInt8Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::UInt16Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::UInt32Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::UInt64Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::Int8Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::Int16Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::Int32Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::Int64Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::Float32Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::Float64Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::Utf8Vec(v) => if let Some(x) = v.get(index) { 
                if let Some(ix) = x.clone() { Some(ix.into()) } else { None } 
            } else { None },
            DataVecOption::Unknown => None
        }
    }
}

pub fn csv_to_dataframe(filename: &str, skip: &[&str]) -> PolarsResult<DataFrame> {
    let file = File::open(filename)?;
    let mut df = CsvReader::new(file).infer_schema(None).has_header(true).finish()?;
    for column_name in skip {
        let _ = df.drop_in_place(column_name)?;
        log::info!("skipping {column_name} since it is on skip list");
    }
    Ok(df)
}

pub fn series_to_datavec_skipna(series: &Series) -> PolarsResult<DataVec> {
    match series.dtype() {
        DataType::UInt8 => Ok(DataVec::UInt8Vec(
            series.u8()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::UInt16 => Ok(DataVec::UInt16Vec(
            series.u16()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::UInt32 => Ok(DataVec::UInt32Vec(
            series.u32()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::UInt64 => Ok(DataVec::UInt64Vec(
            series.u64()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Int8 => Ok(DataVec::Int8Vec(
            series.i8()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Int16 => Ok(DataVec::Int16Vec(
            series.i16()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Int32 => Ok(DataVec::Int32Vec(
            series.i32()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Int64 => Ok(DataVec::Int64Vec(
            series.i64()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Float32 => Ok(DataVec::Float32Vec(
            series.f32()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Float64 => Ok(DataVec::Float64Vec(
            series.f64()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Utf8 => Ok(DataVec::Utf8Vec(
            series.utf8()?.into_iter()
                .filter(|x| x.is_some()).map(|x| Arc::from(x.unwrap())).collect()
        )),
        _ => Ok(DataVec::Unknown)
    }
}

pub fn series_to_datavec(series: &Series) -> PolarsResult<DataVecOption> {
    match series.dtype() {
        DataType::UInt8 => Ok(DataVecOption::UInt8Vec(series.u8()?.into_iter().collect())),
        DataType::UInt16 => Ok(DataVecOption::UInt16Vec(series.u16()?.into_iter().collect())),
        DataType::UInt32 => Ok(DataVecOption::UInt32Vec(series.u32()?.into_iter().collect())),
        DataType::UInt64 => Ok(DataVecOption::UInt64Vec(series.u64()?.into_iter().collect())),
        DataType::Int8 => Ok(DataVecOption::Int8Vec(series.i8()?.into_iter().collect())),
        DataType::Int16 => Ok(DataVecOption::Int16Vec(series.i16()?.into_iter().collect())),
        DataType::Int32 => Ok(DataVecOption::Int32Vec(series.i32()?.into_iter().collect())),
        DataType::Int64 => Ok(DataVecOption::Int64Vec(series.i64()?.into_iter().collect())),
        DataType::Float32 => Ok(DataVecOption::Float32Vec(series.f32()?.into_iter().collect())),
        DataType::Float64 => Ok(DataVecOption::Float64Vec(series.f64()?.into_iter().collect())),
        DataType::Utf8 => Ok(DataVecOption::Utf8Vec(
            series.utf8()?.into_iter()
                .map(|x| match x { Some(y) => Some(Arc::from(y)), None => None })
                .collect()
        )),
        _ => Ok(DataVecOption::Unknown)
    }
}

mod tests {
    #[test]
    fn csv_to_dataframe() {
        let df = super::csv_to_dataframe("data/iris.csv", &vec!["sepal.width"]).unwrap();
        let valid_columns = vec![
            "sepal.length",
            "petal.length",
            "petal.width",
            "variety"
        ];
        for column_name in df.get_column_names() {
            println!("{column_name}");
            assert!(valid_columns.contains(&column_name));
        }
    }
}