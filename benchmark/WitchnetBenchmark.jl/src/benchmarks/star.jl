export Star

module Star

using WitchnetBenchmark
using WitchnetBenchmark.Utils
using RDatasets
using DataFrames
using CSV
using Gadfly

"classification task on the star dataset"
function classify(;target::Symbol=:SchType, measure::Symbol=:accuracy)::DataFrame
    data = dataset()
    models = classification_models()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "star", "classify", target)
    
    title = string("star ", lowercase(string(target)), " classification ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "star", "classify", target)

    result
end

"regression task on the star dataset"
function estimate(;target::Symbol=:Read, measure::Symbol=:rmse)::DataFrame
    data = dataset()
    models = regression_models()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "star", "estimate", target)
    
    title = string("star ", lowercase(string(target)), " estimation ", measure)
    plot = value_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "star", "estimate", target)
    
    result
end

"load ready-to-use star data"
function dataset() 
    data = RDatasets.dataset("mlmRev", "star")
    dropmissing!(data)
    data[!, Not(:Id)]
end

end