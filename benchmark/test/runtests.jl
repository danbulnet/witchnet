using Test
using RDatasets
using MLJ

using WitchnetBenchmark

@testset "iris" begin
    iris = dataset("datasets", "iris")
    y, X = unpack(iris, ==(:Species), colname -> true)

    @test predacc(@load(DecisionTreeClassifier, pkg=DecisionTree, verbosity=false), X, y) > 0.9
    @test predacc(@load(RandomForestClassifier, pkg = ScikitLearn, verbosity=false), X, y) > 0.9
end