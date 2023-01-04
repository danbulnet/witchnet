export RedWine

module RedWine

using WitchnetBenchmark
using WitchnetBenchmark.Utils
using RDatasets
using DataFrames
using MLJ
using CSV
using Gadfly

"classification task on the red wine quality dataset"
function classify(;
    target::Symbol=:quality,
    measure::Symbol=:accuracy,
    models=fast_classification_models()
)::DataFrame
    data = dataset()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "red_wine", "classify", target)
    
    title = string("red wine quality ", lowercase(string(target)), " classification ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "red_wine", "classify", target)

    result
end

"regression task on the red wine quality dataset"
function estimate(;
    target::Symbol=:sulphates,
    measure::Symbol=:nrmse,
    models=fast_regression_models()
)::DataFrame
    data = dataset()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "red_wine", "estimate", target)
    
    title = string("red wine quality", lowercase(string(target)), " estimation ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "red_wine", "estimate", target)
    
    result
end

"load ready-to-use red wine quality data"
function dataset()::DataFrame 
    df = Utils.uciurl2df("wine-quality/winequality-red.csv")
    df[!, :quality] = categorical("grade " .* string.(df[!, :quality]))
    df
end

end