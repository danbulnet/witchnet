export irisclassify, irisclassify_df, irisclassify_plot
export irisreg, irisreg_df, irisreg_plot

using WitchnetBenchmark
using RDatasets
using DataFrames
using MLJ

function irisclassify(measure::Symbol=:accuracy)
    data = RDatasets.dataset("datasets", "iris")
    
    models = classification_models()

    resultdf = evalmodels(data, :Species, models, measure)

    resultplot = if measure == :accuracy 
        percent_barplot(
            resultdf, :model, measure, "iris species classification $measure"
        )
    else nothing end

    resultdf, resultplot
end

irisclassify_df(measure::Symbol=:accuracy) = irisclassify(measure)[1]

irisclassify_plot(measure::Symbol=:accuracy) = irisclassify(measure)[2]

function irisreg(target::Symbol=:SepalLength, measure::Symbol=:rmse)
    data = RDatasets.dataset("datasets", "iris")
    @info describe(data)
    
    models = regression_models()

    resultdf = evalmodels(data, target, models, measure)

    resultplot = resultplot = value_barplot(
        resultdf, :model, measure, "iris $target regression $measure"
    )

    resultdf, resultplot
end

irisreg_df(
    target::Symbol=:SepalLength, measure::Symbol=:rmse
) = irisreg(target, measure)[1]

irisreg_plot(
    target::Symbol=:SepalLength, measure::Symbol=:rmse
) = irisreg(target, measure)[2]