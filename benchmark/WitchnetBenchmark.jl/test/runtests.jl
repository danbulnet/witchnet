using Test
using RDatasets
using MLJ

using WitchnetBenchmark

@testset "iris" begin
    iris = dataset("datasets", "iris")
    y, X = unpack(iris, ==(:Species), colname -> true)

    @test predeval(@load(DecisionTreeClassifier, pkg=BetaML, verbosity=false), X, y, :accuracy) > 0.8
    @test predeval(@load(RandomForestClassifier, pkg=ScikitLearn, verbosity=false), X, y, :accuracy) > 0.8
end