export magds_accuracy, magds_rmse, magds_mae
export asyncmagds_accuracy, asyncmagds_rmse, asyncmagds_mae

# const libpath::String = "benchmark/WitchnetBenchmark.jl/lib/magds.dll"
const libpath::String = "target/release/magds.dll"

"""
example:
    ````
    trainfile = "crates/magds/data/iris_original_train.csv"
    testfile = "crates/magds/data/iris_original_test.csv"
    magds_accuracy("iris", trainfile, testfile, "variety")
    ```
"""
function magds_accuracy(
    train_file::String, 
    test_file::String, 
    target::String,
    weighting_strategy::String,
    fuzzy::Bool,
    weighted::Bool,
    interelement_activation_threshold::Float32,
    interelement_activation_exponent::Int32,
    winners_limit::UInt, 
    weight_ratio::Float32
)::Float64
    result = ccall(
        (:magds_classification_accuracy, libpath), 
        Float64, 
        (Cstring, Cstring, Cstring, Cstring, Cstring, Bool, Bool, Float32, Int32, UInt, Float32), 
        "magds_accuracy",
        train_file,
        test_file,
        target,
        weighting_strategy,
        fuzzy,
        weighted,
        interelement_activation_threshold,
        interelement_activation_exponent,
        winners_limit,
        weight_ratio
    )
    @info string("magds accuracy: ", result)
    result
end

"""
example:
    ````
    trainfile = "crates/magds/data/iris_original_train.csv"
    testfile = "crates/magds/data/iris_original_test.csv"
    magds_rmse("iris", trainfile, testfile, "sepal.length")
    ```
"""
function magds_rmse(
    train_file::String, 
    test_file::String, 
    target::String,
    weighting_strategy::String,
    fuzzy::Bool,
    weighted::Bool,
    interelement_activation_threshold::Float32,
    interelement_activation_exponent::Int32,
    winners_limit::UInt, 
    weight_ratio::Float32
)::Float64
    result = ccall(
        (:magds_regression_rmse, libpath), 
        Float64, 
        (Cstring, Cstring, Cstring, Cstring, Cstring, Bool, Bool, Float32, Int32, UInt, Float32),
        "magds_rmse",
        train_file,
        test_file,
        target,
        weighting_strategy,
        fuzzy,
        weighted,
        interelement_activation_threshold,
        interelement_activation_exponent,
        winners_limit,
        weight_ratio
    )
    @info string("magds rmse: ", result)
    result
end

"""
example:
    ````
    trainfile = "crates/magds/data/iris_original_train.csv"
    testfile = "crates/magds/data/iris_original_test.csv"
    magds_mae("iris", trainfile, testfile, "sepal.length")
    ```
"""
function magds_mae(
    train_file::String, 
    test_file::String, 
    target::String,
    weighting_strategy::String,
    fuzzy::Bool,
    weighted::Bool,
    interelement_activation_threshold::Float32,
    interelement_activation_exponent::Int32,
    winners_limit::UInt, 
    weight_ratio::Float32
)::Float64
    result = ccall(
        (:magds_regression_mae, libpath), 
        Float64, 
        (Cstring, Cstring, Cstring, Cstring, Cstring, Bool, Bool, Float32, Int32, UInt, Float32),
        "magds_mae",
        train_file,
        test_file,
        target,
        weighting_strategy,
        fuzzy,
        weighted,
        interelement_activation_threshold,
        interelement_activation_exponent,
        winners_limit,
        weight_ratio
    )
    @info string("magds mae: ", result)
    result
end

"""
example:
    ````
    trainfile = "crates/magds/data/iris_original_train.csv"
    testfile = "crates/magds/data/iris_original_test.csv"
    asyncmagds_accuracy("iris", trainfile, testfile, "variety")
    ```
"""
function asyncmagds_accuracy(
    train_file::String, 
    test_file::String, 
    target::String,
    weighting_strategy::String,
    fuzzy::Bool,
    weighted::Bool,
    interelement_activation_threshold::Float32,
    interelement_activation_exponent::Int32,
    winners_limit::UInt, 
    weight_ratio::Float32
)::Float64
    result = ccall(
        (:async_magds_classification_accuracy, libpath), 
        Float64, 
        (Cstring, Cstring, Cstring, Cstring, Cstring, Bool, Bool, Float32, Int32, UInt, Float32),
        "asyncmagds_accuracy",
        train_file,
        test_file,
        target,
        weighting_strategy,
        fuzzy,
        weighted,
        interelement_activation_threshold,
        interelement_activation_exponent,
        winners_limit,
        weight_ratio
    )
    @info string("async magds accuracy: ", result)
    result
end

"""
example:
    ````
    trainfile = "crates/magds/data/iris_original_train.csv"
    testfile = "crates/magds/data/iris_original_test.csv"
    asyncmagds_rmse("iris", trainfile, testfile, "sepal.length")
    ```
"""
function asyncmagds_rmse(
    train_file::String, 
    test_file::String, 
    target::String,
    weighting_strategy::String,
    fuzzy::Bool,
    weighted::Bool,
    interelement_activation_threshold::Float32,
    interelement_activation_exponent::Int32,
    winners_limit::UInt, 
    weight_ratio::Float32
)::Float64
    result = ccall(
        (:async_magds_regression_rmse, libpath), 
        Float64, 
        (Cstring, Cstring, Cstring, Cstring, Cstring, Bool, Bool, Float32, Int32, UInt, Float32),
        "asyncmagds_rmse",
        train_file,
        test_file,
        target,
        weighting_strategy,
        fuzzy,
        weighted,
        interelement_activation_threshold,
        interelement_activation_exponent,
        winners_limit,
        weight_ratio
    )
    @info string("async magds rmse: ", result)
    result
end

"""
example:
    ````
    trainfile = "crates/magds/data/iris_original_train.csv"
    testfile = "crates/magds/data/iris_original_test.csv"
    asyncmagds_mae("iris", trainfile, testfile, "sepal.length")
    ```
"""
function asyncmagds_mae(
    train_file::String, 
    test_file::String, 
    target::String,
    weighting_strategy::String,
    fuzzy::Bool,
    weighted::Bool,
    interelement_activation_threshold::Float32,
    interelement_activation_exponent::Int32,
    winners_limit::UInt, 
    weight_ratio::Float32
)::Float64
    result = ccall(
        (:async_magds_regression_mae, libpath), 
        Float64, 
        (Cstring, Cstring, Cstring, Cstring, Cstring, Bool, Bool, Float32, Int32, UInt, Float32),
        "asyncmagds_mae",
        train_file,
        test_file,
        target,
        weighting_strategy,
        fuzzy,
        weighted,
        interelement_activation_threshold,
        interelement_activation_exponent,
        winners_limit,
        weight_ratio
    )
    @info string("async magds mae: ", result)
    result
end