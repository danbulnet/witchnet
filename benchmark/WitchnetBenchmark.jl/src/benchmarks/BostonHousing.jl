export BostonHousing

module BostonHousing

using WitchnetBenchmark
using WitchnetBenchmark.Utils
using RDatasets
using DataFrames
using MLJ
using CSV
using Gadfly

"classification task on the boston housing dataset"
function classify(;
    target::Symbol=:Rad, 
    measure::Symbol=:accuracy, 
    models=fast_classification_models()
)::DataFrame
    data = dataset()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "boston_housing", "classify", target)
    
    title = string("boston housing ", lowercase(string(target)), " classification ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "boston_housing", "classify", target)

    result
end

"regression task on the boston housing dataset"
function estimate(;
    target::Symbol=:MedV,
    measure::Symbol=:nrmse,
    models=fast_regression_models()
)::DataFrame
    data = dataset()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "boston_housing", "estimate", target)
    
    title = string("boston housing ", lowercase(string(target)), " estimation ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "boston_housing", "estimate", target)
    
    result
end

"load ready-to-use boston housing data"
function dataset()::DataFrame
    df = RDatasets.dataset("MASS", "Boston")
    df[!, :Rad] = categorical("rad " .* string.(df[!, :Rad]))
    df
end

end