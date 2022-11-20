export irisclassify, irisclassify_df, irisclassify_plot
export irisregession, irisregession_df, irisregession_plot

using WitchnetBenchmark
using RDatasets
using DataFrames
using MLJ

function irisclassify(measure::Symbol=:accuracy)
    data = RDatasets.dataset("datasets", "iris")
    
    models = classification_models()

    resultdf = evalmodels(data, :Species, models, measure)

    resultplot = if measure == :accuracy 
        percentplot(
            resultdf, :model, measure, "iris species classification $measure"
        )
    else nothing end

    resultdf, resultplot
end

irisclassify_df(measure::Symbol=:accuracy) = irisclassify(measure)[1]

irisclassify_plot(measure::Symbol=:accuracy) = irisclassify(measure)[2]

function irisregession(target::Symbol=:SepalLength, measure::Symbol=:rmse)
    data = RDatasets.dataset("datasets", "iris")
    
    models = regression_models()

    resultdf = evalmodels(data, target, models, measure)

    resultplot = nothing

    resultdf, resultplot
end

irisregession_df(
    target::Symbol=:SepalLength, measure::Symbol=:rmse
) = irisregession(target, measure)[1]

irisregession_plot(
    target::Symbol=:SepalLength, measure::Symbol=:rmse
) = irisregession(target, measure)[2]