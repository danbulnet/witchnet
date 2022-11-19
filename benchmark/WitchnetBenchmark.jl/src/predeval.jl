export pred, predeval

import Random
import MLJ

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