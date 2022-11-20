export pred, predeval, evalmodels

using DataFrames
using CSV

import Random
import MLJ
import CategoricalArrays.CategoricalValue

"""
    example: 
        modelfactory = @load RandomForestClassifier pkg = ScikitLearn
        predictmlj(modelfactory, X, y)
"""
function pred(modelfactory, X, y; ttratio=0.7, seed=58)
    model = modelfactory()
    mach = MLJ.machine(model, X, y)

    train, test = ttindices(y, ttratio; seed=seed)
    MLJ.fit!(mach; rows=train, verbosity=0)
    y[test], MLJ.predict_mode(mach, X[test, :])
end

function ttindices(y, ttratio=0.7; seed=58)
    train, test = MLJ.partition(
        eachindex(y), ttratio;
        shuffle=true, rng=Random.MersenneTwister(seed)
    )
    train, test
end

function predeval(modelfactory, X, y, measure::Symbol; ttratio=0.7, seed=58)
    ytest, ŷtest = pred(modelfactory, X, y; ttratio=ttratio, seed=seed)
    result = getproperty(MLJ, measure)(ŷtest, ytest)
    @info string(measure, ": ", result)
    result
end

function evalmodels(
    data::DataFrame, target::Symbol, models::Dict, metric::Symbol;
    ttratio=0.7, seed=58
)::DataFrame
    y, X = MLJ.unpack(data, ==(target), colname -> true)
    encodermach = MLJ.machine(MLJ.ContinuousEncoder(), X) |> MLJ.fit!
    Xencoded = MLJ.transform(encodermach, X)
    modelnames = []
    results = []

    for (name, model) in models
        if name == :MAGDS
            datav2 = mapcols(col -> eltype(col) <: CategoricalValue ? string.(col) : col , data)

            tmpdir = "evalmodels_temp"
            mkpath(tmpdir)
            
            train, test = ttindices(y, ttratio; seed=seed)

            trainpath = joinpath(tmpdir, "train.csv")
            testpath = joinpath(tmpdir, "test.csv")

            CSV.write(trainpath, datav2[train, :])
            CSV.write(testpath, datav2[test, :])

            magds_predict = getproperty(@__MODULE__, Symbol("magds_" * string(metric)))
            push!(modelnames, :sync_magds)
            push!(results, magds_predict(trainpath, testpath, string(target)))
            
            asyncmagds_predict = getproperty(@__MODULE__, Symbol("asyncmagds_" * string(metric)))
            push!(modelnames, :async_magds)
            push!(results, asyncmagds_predict(trainpath, testpath, string(target)))

            rm(tmpdir; recursive=true)
        else
            push!(modelnames, name)
            push!(results, predeval(model, Xencoded, y, metric; ttratio=ttratio, seed=seed))
        end
    end

    resultdf = DataFrame(:model => modelnames, metric => results)
    sort!(resultdf, metric, rev=true)

    resultdf
end