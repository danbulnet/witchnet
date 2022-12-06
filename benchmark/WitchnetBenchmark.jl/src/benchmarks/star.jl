export Star

module Star

using WitchnetBenchmark
using RDatasets
using DataFrames
using MLJ

"classification task on the star dataset"
function classify(;target::Symbol=:Sx, measure::Symbol=:accuracy)::DataFrame
    data = dataset()
    models = classification_models()
    evalmodels(data, target, models, measure)
end

"regression task on the star dataset"
function estimate(;target::Symbol=:Math, measure::Symbol=:rmse)::DataFrame
    data = dataset()
    models = regression_models()
    evalmodels(data, target, models, measure)
end

"load ready-to-use star data"
function dataset() 
    data = RDatasets.dataset("mlmRev", "star")
    dropmissing!(data)
    data
end

end