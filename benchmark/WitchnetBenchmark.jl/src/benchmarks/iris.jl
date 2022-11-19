export irisspecies, irisspecies_df, irisspecies_plot

using WitchnetBenchmark
using RDatasets
using DataFrames
using MLJ

function loadiris()
    iris = RDatasets.dataset("datasets", "iris")
    y, X = MLJ.unpack(iris, ==(:Species), colname -> true)
    y, X
end

function irisspecies(measure::Symbol=:accuracy)
    y, X = loadiris()
    
    models = stdmodels()

    resultdf = evalmodels(X, y, models, measure)

    resultplot = accplot(
        resultdf, :model, measure, "iris species classification $measure"
    )

    resultdf, resultplot
end

irisspecies_df(measure::Symbol=:accuracy) = irisspecies(measure)[1]

irisspecies_plot(measure::Symbol=:accuracy) = irisspecies(measure)[2]