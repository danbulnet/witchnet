using DataFrames
using Random

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
        shuffle=true, rng=MersenneTwister(seed)
    )
    MLJ.fit!(mach; rows=train, verbosity=0)
    y[test], MLJ.predict_mode(mach, X[test, :])
end

function predacc(modelfactory, X, y; ttratio=0.7, seed=58)
    ytest, ŷtest = pred(modelfactory, X, y; ttratio=ttratio, seed=seed)
    acc = MLJ.accuracy(ŷtest, ytest)
    @info "accuracy: ", acc
    acc
end