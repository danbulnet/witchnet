export Iris

module Iris

using WitchnetBenchmark
using RDatasets
using DataFrames
using MLJ

"classification task on the iris dataset"
function classify(;target::Symbol=:Species, measure::Symbol=:accuracy)::DataFrame
    data = dataset()
    models = classification_models()
    evalmodels(data, target, models, measure)
end

"regression task on the iris dataset"
function estimate(;target::Symbol=:SepalLength, measure::Symbol=:rmse)::DataFrame
    data = dataset()
    models = regression_models()
    evalmodels(data, target, models, measure)
end

"load ready-to-use iris data"
dataset()::DataFrame = RDatasets.dataset("datasets", "iris")

end