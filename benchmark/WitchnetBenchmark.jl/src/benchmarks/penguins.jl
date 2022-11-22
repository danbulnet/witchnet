export penguinclassify, penguinclassify_df, penguinclassify_plot
export penguinreg, penguinreg_df, penguinreg_plot

using WitchnetBenchmark
using DataFrames
using MLJ
using CSV
using CategoricalArrays

function penguinclassify(measure::Symbol=:accuracy)
    data = CSV.File(
        normpath(joinpath(dirname(@__FILE__), "../../../datasets/single/penguins.csv"))
    ) |> DataFrame
    mapcols!(col -> eltype(col) <: AbstractString ? categorical(col) : col, data)
    
    models = classification_models()

    resultdf = evalmodels(data, :species, models, measure)

    resultplot = if measure == :accuracy 
        percent_barplot(
            resultdf, :model, measure, "palmer penguins species classification $measure"
        )
    else nothing end

    resultdf, resultplot
end

penguinclassify_df(measure::Symbol=:accuracy) = penguinclassify(measure)[1]

penguinclassify_plot(measure::Symbol=:accuracy) = penguinclassify(measure)[2]

function penguinreg(target::Symbol=:body_mass_g, measure::Symbol=:rmse)
    data = CSV.File(
        normpath(joinpath(dirname(@__FILE__), "../../../datasets/single/penguins.csv"))
    ) |> DataFrame
    mapcols!(col -> eltype(col) <: AbstractString ? categorical(col) : col, data)
    mapcols!(col -> eltype(col) <: Real ? Float64.(col) : col, data)
    
    models = regression_models()

    resultdf = evalmodels(data, target, models, measure)

    resultplot = value_barplot(
        resultdf, :model, measure, "palmer penguins $target regression $measure"
    )

    resultdf, resultplot
end

penguinreg_df(
    target::Symbol=:body_mass_g, measure::Symbol=:rmse
) = penguinreg(target, measure)[1]

penguinreg_plot(
    target::Symbol=:body_mass_g, measure::Symbol=:rmse
) = penguinreg(target, measure)[2]