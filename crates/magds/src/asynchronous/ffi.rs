use std::{
    panic,
    ffi::CStr,
    fs::File
};

use polars::prelude::*;

use witchnet_common::performance::SupervisedPerformance;

use super::{
    parser,
    algorithm::prediction
};

unsafe fn extern_supervised_performance(
    name_ptr: *const i8,
    train_file_ptr: *const i8,
    test_file_ptr: *const i8,
    target_ptr: *const i8
) -> SupervisedPerformance {
    let name = CStr::from_ptr(name_ptr).to_str().unwrap().to_owned();
    let train_file = CStr::from_ptr(train_file_ptr).to_str().unwrap().to_owned();
    let test_file = CStr::from_ptr(test_file_ptr).to_str().unwrap().to_owned();
    let target = CStr::from_ptr(target_ptr).to_str().unwrap().to_owned();

    let mut magds_train = parser::magds_from_csv(&name, &train_file, &vec![]).unwrap();
    let test: DataFrame = CsvReader::new(File::open(test_file).unwrap())
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap();

    prediction::prediction_score_df(
        &mut magds_train, &test, (target.as_str()).into(), true, false
    ).unwrap()
}

#[no_mangle]
unsafe extern "C" fn async_magds_classification_accuracy(
    name_ptr: *const i8,
    train_file_ptr: *const i8,
    test_file_ptr: *const i8,
    target_ptr: *const i8
) -> f64 {
    match panic::catch_unwind(|| {
        extern_supervised_performance(
            name_ptr, train_file_ptr, test_file_ptr, target_ptr
        ).accuracy().unwrap()
    }) {
        Ok(r) => r,
        Err(e) => { println!("{:?}", e); -1.0 }
    }
}

#[no_mangle]
unsafe extern "C" fn async_magds_regression_rmse(
    name_ptr: *const i8,
    train_file_ptr: *const i8,
    test_file_ptr: *const i8,
    target_ptr: *const i8
) -> f64 {
    match panic::catch_unwind(|| {
        extern_supervised_performance(
            name_ptr, train_file_ptr, test_file_ptr, target_ptr
        ).rmse().unwrap()
    }) {
        Ok(r) => r,
        Err(e) => { println!("{:?}", e); -1.0 }
    }
}

#[no_mangle]
unsafe extern "C" fn async_magds_regression_mae(
    name_ptr: *const i8,
    train_file_ptr: *const i8,
    test_file_ptr: *const i8,
    target_ptr: *const i8
) -> f64 {
    match panic::catch_unwind(|| {
        extern_supervised_performance(
            name_ptr, train_file_ptr, test_file_ptr, target_ptr
        ).mae().unwrap()
    }) {
        Ok(r) => r,
        Err(e) => { println!("{:?}", e); -1.0 }
    }
}