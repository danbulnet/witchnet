using Test
using RDatasets
using MLJ

using WitchnetBenchmark

# @testset "iris" begin
#     iris = dataset("datasets", "iris")
#     y, X = unpack(iris, ==(:Species), colname -> true)

#     @test predeval(@load(DecisionTreeClassifier, pkg=BetaML, verbosity=false), X, y, :accuracy) > 0.8
#     @test predeval(@load(RandomForestClassifier, pkg=ScikitLearn, verbosity=false), X, y, :accuracy) > 0.8
# end

@testset "magds" begin
    cd("../../..")
    trainfile = "crates/magds/data/iris_original_train.csv"
    testfile = "crates/magds/data/iris_original_test.csv"
    @test magds_accuracy(trainfile, testfile, "variety") > 0.8
    @test magds_rmse(trainfile, testfile, "sepal.length") > 0.0
    @test magds_mae(trainfile, testfile, "sepal.length") > 0.0
    @test asyncmagds_accuracy(trainfile, testfile, "variety") > 0.8
    @test asyncmagds_rmse(trainfile, testfile, "sepal.length") > 0.0
    @test asyncmagds_mae(trainfile, testfile, "sepal.length") > 0.0
end