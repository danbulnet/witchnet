export Penguin

module Penguin

using WitchnetBenchmark
using WitchnetBenchmark.Utils
using RDatasets
using DataFrames
using MLJ
using CategoricalArrays
using CSV
using Gadfly

"classification task on the palmer penguins dataset"
function classify(;
    target::Symbol=:species, 
    measure::Symbol=:accuracy, 
    models=fast_classification_models()
)::DataFrame
    data = dataset()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "penguin", "classify", target)
    
    title = string("penguin ", lowercase(string(target)), " classification ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "penguin", "classify", target)

    result
end

"regression task on the palmer penguins dataset"
function estimate(;
    target::Symbol=:body_mass_g, 
    measure::Symbol=:nrmse, 
    models=fast_regression_models()
)::DataFrame
    data = dataset()
    result = evalmodels(data, target, models, measure)
    
    Utils.writecsv(result, "penguin", "estimate", target)
    
    title = string("penguin ", lowercase(string(target)), " estimation ", measure)
    plot = percent_barplot(result, :model, measure, title)
    Utils.writeimg(plot, "penguin", "estimate", target)
    
    result
end

"load ready-to-use palmer penguins data"
function dataset()
    data = CSV.File(
        normpath(joinpath(dirname(@__FILE__), "../../../datasets/single/penguins.csv"))
    ) |> DataFrame
    mapcols!(col -> eltype(col) <: AbstractString ? categorical(col) : col, data)
    mapcols!(col -> eltype(col) <: Real ? Float64.(col) : col, data)
end

end