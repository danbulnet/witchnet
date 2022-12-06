export Penguins

module Penguins

using WitchnetBenchmark
using RDatasets
using DataFrames
using MLJ
using CategoricalArrays
using CSV

"classification task on the palmer penguins dataset"
function classify(;target::Symbol=:species, measure::Symbol=:accuracy)::DataFrame
    data = dataset()
    models = classification_models()
    evalmodels(data, target, models, measure)
end

"regression task on the palmer penguins dataset"
function estimate(;target::Symbol=:body_mass_g, measure::Symbol=:rmse)::DataFrame
    data = dataset()
    models = regression_models()
    evalmodels(data, target, models, measure)
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