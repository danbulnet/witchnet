export Iris

module Iris

using WitchnetBenchmark
using WitchnetBenchmark.Utils
using RDatasets
using DataFrames
using MLJ
using CSV
using Gadfly

"classification task on the iris dataset"
function classify(;
    target::Symbol=:Species, 
    measure::Symbol=:accuracy, 
    models=fast_classification_models()
)::DataFrame
    data = dataset()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "iris", "classify", target)
    
    title = string("iris ", lowercase(string(target)), " classification ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "iris", "classify", target)

    result
end

"regression task on the iris dataset"
function estimate(;
    target::Symbol=:SepalLength,
    measure::Symbol=:nrmse,
    models=fast_regression_models()
)::DataFrame
    data = dataset()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "iris", "estimate", target)
    
    title = string("iris ", lowercase(string(target)), " estimation ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "iris", "estimate", target)
    
    result
end

"load ready-to-use iris data"
dataset()::DataFrame = RDatasets.dataset("datasets", "iris")

end