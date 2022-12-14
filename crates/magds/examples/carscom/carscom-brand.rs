use env_logger;

use magds::synchronous::{
    algorithm::prediction,
    parser
};

use witchnet_common::{
    benchmark,
    polars as polars_common
};

fn main() {
    env_logger::init();

    let train_file_path = format!(
        "{}/{}", 
        "crates/magds/examples/carscom/data", 
        // "carscom_full_1m_18_08_2022_prepared_train.csv"
        // "carscom_full_1m_18_08_2022_prepared_train_mid.csv"
        "carscom_full_1m_18_08_2022_prepared_train_small.csv"
    );
    let test_file_path = format!(
        "{}/{}", 
        "crates/magds/examples/carscom/data", 
        // "carscom_full_1m_18_08_2022_prepared_test.csv"
        // "carscom_full_1m_18_08_2022_prepared_test_mid.csv"
        "carscom_full_1m_18_08_2022_prepared_test_small.csv"
    );
// 
    // let skip_list = vec!["vin"];
    let skip_list = vec!["vin", "features"];

    let train_df = polars_common::csv_to_dataframe(&train_file_path, &skip_list).unwrap();
    println!("train set shape {:?}", train_df.shape());

    let test_df = polars_common::csv_to_dataframe(&test_file_path, &skip_list).unwrap();
    println!("test set shape {:?}", test_df.shape());

    let mut magds_train = benchmark::timeit("magds training", move || {
        parser::magds_from_df("carscom_train", &train_df)
    });

    let performance = benchmark::timeit("magds prediction", move || {
        prediction::prediction_score_df(
            &mut magds_train, &test_df, "brand".into()
        ).unwrap()
    });
    let accuracy = performance.accuracy().unwrap();
    let mean_probability = performance.mean_probability().unwrap();
    
    println!("accuracy: {accuracy} mean_probability: {mean_probability}");
}