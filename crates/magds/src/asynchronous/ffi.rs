use std::{
    panic,
    ffi::CStr,
    fs::File,
    sync::Arc
};

use polars::prelude::*;

use witchnet_common::{
    performance::SupervisedPerformance,
    connection::collective::defining::*
};

use super::{
    parser,
    algorithm::prediction
};

/// weighting_strategy available values:
///     - "ConstantZeroWeight"
///     - "ConstantOneWeight" (default)
///     - "OneOverOuts"
///     - "OneOverOutsUpperHalf"
///     - "OneOverOutsUpperQuarter"
unsafe fn extern_supervised_performance(
    name_ptr: *const i8,
    train_file_ptr: *const i8,
    test_file_ptr: *const i8,
    target_ptr: *const i8,
    weighting_strategy_ptr: *const i8,
    fuzzy: bool,
    weighted: bool,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32,
    winners_limit: usize, 
    weight_ratio: f32
) -> SupervisedPerformance {
    let name = CStr::from_ptr(name_ptr).to_str().unwrap().to_owned();
    let train_file = CStr::from_ptr(train_file_ptr).to_str().unwrap().to_owned();
    let test_file = CStr::from_ptr(test_file_ptr).to_str().unwrap().to_owned();
    let target = CStr::from_ptr(target_ptr).to_str().unwrap().to_owned();
    let weighting_strategy_cstr = CStr::from_ptr(weighting_strategy_ptr).to_str().unwrap().to_owned();

    let weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync> = 
    if &weighting_strategy_cstr == "ConstantOneWeight" {
        Arc::new(ConstantOneWeightAsync)
    } else if &weighting_strategy_cstr == "OneOverOutsUpperHalf" {
        Arc::new(OneOverOutsUpperHalfAsync)
    } else if &weighting_strategy_cstr == "OneOverOutsUpperQuarter" {
        Arc::new(OneOverOutsUpperQuarterAsync)
    } else if &weighting_strategy_cstr == "OneOverOuts" {
        Arc::new(OneOverOutsAsync)
    } else if &weighting_strategy_cstr == "ConstantZeroWeight" {
        Arc::new(ConstantZeroWeightAsync)
    } else {
        Arc::new(ConstantOneWeightAsync)
    };

    let mut magds_train = parser::magds_from_csv_custom(
        &name,
        &train_file,
        &vec![],
        weighting_strategy,
        interelement_activation_threshold,
        interelement_activation_exponent
    ).unwrap();
    let test: DataFrame = CsvReader::new(File::open(test_file).unwrap())
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap();

    prediction::prediction_score_df_custom(
        &mut magds_train,
        &test,
        (target.as_str()).into(),
        fuzzy,
        weighted,
        winners_limit,
        weight_ratio
    ).unwrap()
}

#[no_mangle]
unsafe extern "C" fn async_magds_classification_accuracy(
    name_ptr: *const i8,
    train_file_ptr: *const i8,
    test_file_ptr: *const i8,
    target_ptr: *const i8,
    weighting_strategy_ptr: *const i8,
    fuzzy: bool,
    weighted: bool,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32,
    winners_limit: usize, 
    weight_ratio: f32
) -> f64 {
    match panic::catch_unwind(|| {
        extern_supervised_performance(
            name_ptr,
            train_file_ptr,
            test_file_ptr,
            target_ptr,
            weighting_strategy_ptr,
            fuzzy,
            weighted,
            interelement_activation_threshold,
            interelement_activation_exponent,
            winners_limit,
            weight_ratio
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
    target_ptr: *const i8,
    weighting_strategy_ptr: *const i8,
    fuzzy: bool,
    weighted: bool,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32,
    winners_limit: usize, 
    weight_ratio: f32
) -> f64 {
    match panic::catch_unwind(|| {
        extern_supervised_performance(
            name_ptr,
            train_file_ptr,
            test_file_ptr,
            target_ptr,
            weighting_strategy_ptr,
            fuzzy,
            weighted,
            interelement_activation_threshold,
            interelement_activation_exponent,
            winners_limit,
            weight_ratio
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
    target_ptr: *const i8,
    weighting_strategy_ptr: *const i8,
    fuzzy: bool,
    weighted: bool,
    interelement_activation_threshold: f32,
    interelement_activation_exponent: i32,
    winners_limit: usize, 
    weight_ratio: f32
) -> f64 {
    match panic::catch_unwind(|| {
        extern_supervised_performance(
            name_ptr,
            train_file_ptr,
            test_file_ptr,
            target_ptr,
            weighting_strategy_ptr,
            fuzzy,
            weighted,
            interelement_activation_threshold,
            interelement_activation_exponent,
            winners_limit,
            weight_ratio
        ).mae().unwrap()
    }) {
        Ok(r) => r,
        Err(e) => { println!("{:?}", e); -1.0 }
    }
}