export pred, predeval, evalmodels

import Random
import MLJ
using DataFrames

"""
    example: 
        modelfactory = @load RandomForestClassifier pkg = ScikitLearn
        predictmlj(modelfactory, X, y)
"""
function pred(modelfactory, X, y; ttratio=0.7, seed=58)
    model = modelfactory()
    mach = MLJ.machine(model, X, y)

    train, test = MLJ.partition(
        eachindex(y), ttratio;
        shuffle=true, rng=Random.MersenneTwister(seed)
    )
    MLJ.fit!(mach; rows=train, verbosity=0)
    y[test], MLJ.predict_mode(mach, X[test, :])
end

function predeval(modelfactory, X, y, measure::Symbol; ttratio=0.7, seed=58)
    ytest, ŷtest = pred(modelfactory, X, y; ttratio=ttratio, seed=seed)
    result = getproperty(MLJ, measure)(ŷtest, ytest)
    @info string(measure, ": ", result)
    result
end

function evalmodels(X, y, models::Dict, metric::Symbol)::DataFrame
    results = []
    for model in values(models)
        push!(results, predeval(model, X, y, metric))
    end

    resultdf = DataFrame(:model => collect(keys(models)), metric => results)
    sort!(resultdf, metric, rev=true)

    resultdf
end