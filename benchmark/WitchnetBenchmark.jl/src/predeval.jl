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
function pred(modelfactory, X, y, measure; ttratio=0.7, seed=58)
    outs = measure in [:rmse, :mae] ? 1 : length(unique(y))
    model = modelfactory(ncol(X), outs)
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
    ytest, ŷtest = pred(modelfactory, X, y, measure; ttratio=ttratio, seed=seed)
    result = getproperty(MLJ, measure)(ŷtest, ytest)
    @info string(measure, ": ", result)
    result
end

function evalmodels(
    data::DataFrame, target::Symbol, models::Dict, metric::Symbol;
    ttratio=0.7, seed=58
)::DataFrame
    MLJ.default_resource(CPUProcesses())

    y, X = MLJ.unpack(data, ==(target), colname -> true)
    encodermach = MLJ.machine(MLJ.ContinuousEncoder(), X) |> MLJ.fit!
    Xencoded = MLJ.transform(encodermach, X)
    modelnames = []
    results = []
    times = []
    memory = []

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
            asyncmagds_predict = getproperty(@__MODULE__, Symbol("asyncmagds_" * string(metric)))

            push!(modelnames, :MAGDS_t00001_e1_w1)
            time = @elapsed begin
                mem = @allocated result = asyncmagds_predict(
                    trainpath, testpath, string(target), "ConstantOneWeight", Float32(0.00001), Int32(1)
                )
            end
            push!(results, result); push!(memory, mem); push!(times, time)

            # push!(modelnames, :MAGDS_t98_e2_w1)
            # time = @elapsed begin
            #     mem = @allocated result = asyncmagds_predict(
            #         trainpath, testpath, string(target), "ConstantOneWeight", Float32(0.98), Int32(2)
            #     )
            # end
            # push!(results, result); push!(memory, mem); push!(times, time)

            # push!(modelnames, :MAGDS_t00001_e1_w05)
            # time = @elapsed begin
            #     mem = @allocated result = asyncmagds_predict(
            #         trainpath, testpath, string(target), "OneOverOutsUpperHalf", Float32(0.00001), Int32(1)
            #     )
            # end
            # push!(results, result); push!(memory, mem); push!(times, time)

            # push!(modelnames, :MAGDS_t98_e2_w05)
            # time = @elapsed begin
            #     mem = @allocated result = asyncmagds_predict(
            #         trainpath, testpath, string(target), "OneOverOutsUpperHalf", Float32(0.98), Int32(2)
            #     )
            # end
            # push!(results, result); push!(memory, mem); push!(times, time)

            rm(tmpdir; recursive=true)
        else
            push!(modelnames, name)
            time = @elapsed begin
                mem = @allocated result = predeval(
                    model, Xencoded, y, metric; ttratio=ttratio, seed=seed
                )
            end
            push!(results, result)
            push!(memory, mem)
            push!(times, time)
        end
    end

    resultdf = DataFrame(
        :model => modelnames, 
        metric => results, 
        :time => times,
        :memory => memory
    )
    sort!(resultdf, metric, rev=true)

    resultdf
end