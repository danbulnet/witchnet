use std::fs::File;

use polars::prelude::*;

use env_logger;

use magds::{
    simple::parser,
    algorithm::prediction
};

use witchnet_common::benchmark;

fn main() {
    env_logger::init();

    let train_file_path = format!(
        "{}/{}", 
        "magds/examples/carscom/data", 
        "carscom_full_1m_18_08_2022_prepared_train_small.csv"
    );
    let test_file_path = format!(
        "{}/{}", 
        "magds/examples/carscom/data", 
        "carscom_full_1m_18_08_2022_prepared_test_small.csv"
    );

    let train_file = File::open(&train_file_path).expect("could not open file");
    let train_df: DataFrame = CsvReader::new(train_file)
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap();
    println!("train set shape {:?}", train_df.shape());

    let test_file = File::open(&test_file_path).expect("could not open file");
    let test_df: DataFrame = CsvReader::new(test_file)
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap();
    println!("test set shape {:?}", test_df.shape());

    let mut magds_train = benchmark::timeit("magds training", move || {
        parser::magds_from_csv("carscom_train", &train_file_path).unwrap()
    });

    let performance = benchmark::timeit("magds prediction", move || {
        prediction::prediction_score_df(
            &mut magds_train, &test_df, "price".into(), false
        ).unwrap()
    });
    let rmse = performance.rmse().unwrap();
    let mae = performance.mae().unwrap();
    let proba = performance.mean_probability().unwrap();
    
    println!("rmse: {rmse} mae: {mae} proba: {proba}");
}