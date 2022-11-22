export starclassify, starclassify_df, starclassify_plot
export starreg, starreg_df, starreg_plot

using WitchnetBenchmark
using DataFrames
using MLJ
using CSV
using CategoricalArrays

function starclassify(measure::Symbol=:accuracy)
    data = RDatasets.dataset("mlmRev", "star")
    dropmissing!(data)
    
    models = classification_models()

    resultdf = evalmodels(data, :Sx, models, measure)

    resultplot = if measure == :accuracy 
        percent_barplot(
            resultdf, :model, measure, "palmer stars species classification $measure"
        )
    else nothing end

    resultdf, resultplot
end

starclassify_df(measure::Symbol=:accuracy) = starclassify(measure)[1]

starclassify_plot(measure::Symbol=:accuracy) = starclassify(measure)[2]

function starreg(target::Symbol=:Math, measure::Symbol=:rmse)
    data = RDatasets.dataset("mlmRev", "star")
    dropmissing!(data)

    mapcols!(col -> eltype(col) <: Real ? Float64.(col) : col, data)
    
    models = regression_models()

    resultdf = evalmodels(data, target, models, measure)

    resultplot = value_barplot(
        resultdf, :model, measure, "palmer stars $target regression $measure"
    )

    resultdf, resultplot
end

starreg_df(
    target::Symbol=:Math, measure::Symbol=:rmse
) = starreg(target, measure)[1]

starreg_plot(
    target::Symbol=:Math, measure::Symbol=:rmse
) = starreg(target, measure)[2]