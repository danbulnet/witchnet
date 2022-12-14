export pred, predeval, evalmodels

using DataFrames
using CSV

import Random
import MLJ
import CategoricalArrays.CategoricalValue
import StatsBase

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
    result = if measure == :nrmse
        nrmse(ŷtest, ytest)
    else
        getproperty(MLJ, measure)(ŷtest, ytest)
    end
    @info string(measure, ": ", result)
    result
end

nrmse(ŷtest, ytest) = MLJ.rmse(ŷtest, ytest) / StatsBase.iqr(ytest)

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
        if name == :MAGDS || name == :MAGDS_gridsearch
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

            if name == :MAGDS
                push!(modelnames, :MAGDS)
                time = @elapsed begin
                    mem = @allocated result = magds_predict(
                        trainpath, 
                        testpath, 
                        string(target), 
                        "ConstantOneWeight",
                        true,
                        true,
                        Float32(0.00001), 
                        Int32(1),
                        UInt(1000),
                        Float32(1.5)
                    )
                end
                push!(results, result); push!(memory, mem); push!(times, time)

                push!(modelnames, :MAGDS_grid)
                time = @elapsed begin
                    mem = @allocated result = magds_predict(
                        trainpath, 
                        testpath, 
                        string(target), 
                        "OneOverOutsUpperHalf",
                        true,
                        false,
                        Float32(0.1), 
                        Int32(5),
                        UInt(1000),
                        Float32(3.0)
                    )
                end
                push!(results, result); push!(memory, mem); push!(times, time)

                push!(modelnames, :MAGDS_sh_f1_w1_t0e5_e2_l1k_r1p1)
                time = @elapsed begin
                    mem = @allocated result = magds_predict(
                        trainpath, 
                        testpath, 
                        string(target), 
                        "OneOverOutsUpperHalf",
                        true,
                        true,
                        Float32(0.00001), 
                        Int32(2),
                        UInt(1000),
                        Float32(1.1)
                    )
                end
                push!(results, result); push!(memory, mem); push!(times, time)

                push!(modelnames, :MAGDS_sh_f0_w1_t0e5_e2_l1k_r1p1)
                time = @elapsed begin
                    mem = @allocated result = magds_predict(
                        trainpath, 
                        testpath, 
                        string(target), 
                        "OneOverOutsUpperHalf",
                        false,
                        true,
                        Float32(0.00001), 
                        Int32(2),
                        UInt(1000),
                        Float32(1.1)
                    )
                end
                push!(results, result); push!(memory, mem); push!(times, time)

                push!(modelnames, :MAGDS_sh_f1_w0_t0e5_e2_l1k_r1p1)
                time = @elapsed begin
                    mem = @allocated result = magds_predict(
                        trainpath, 
                        testpath, 
                        string(target), 
                        "OneOverOutsUpperHalf",
                        true,
                        false,
                        Float32(0.00001), 
                        Int32(2),
                        UInt(1000),
                        Float32(1.1)
                    )
                end
                push!(results, result); push!(memory, mem); push!(times, time)

                push!(modelnames, :MAGDS_sh_f0_w0_t0e5_e2_l1k_r1p1)
                time = @elapsed begin
                    mem = @allocated result = magds_predict(
                        trainpath, 
                        testpath, 
                        string(target), 
                        "OneOverOutsUpperHalf",
                        false,
                        false,
                        Float32(0.00001), 
                        Int32(2),
                        UInt(1000),
                        Float32(1.1)
                    )
                end
                push!(results, result); push!(memory, mem); push!(times, time)

                push!(modelnames, :MAGDS_sh_f1_w1_t0e5_e1_l1k_r1p1)
                time = @elapsed begin
                    mem = @allocated result = magds_predict(
                        trainpath, 
                        testpath, 
                        string(target), 
                        "OneOverOutsUpperHalf",
                        true,
                        true,
                        Float32(0.00001), 
                        Int32(1),
                        UInt(1000),
                        Float32(1.1)
                    )
                end
                push!(results, result); push!(memory, mem); push!(times, time)
            elseif name == :MAGDS_gridsearch
                i = 1
                for weightingstrategy in ["ConstantOneWeight", "OneOverOuts", "OneOverOutsUpperHalf", "OneOverOutsUpperQuarter"]
                    for fuzzy in [true, false]
                        for weighted in [true, false]
                            for ieth in [0.00001, 0.1, 0.9, 0.99]
                            # for ieth in [0.00001, 0.1, 0.5, 0.8, 0.9, 0.95, 0.98, 0.99]
                                for iee in 1:5
                                    for winnerslimit in [1, 2, 5, 20, 100, 500]
                                    # for winnerslimit in [1, 2, 5, 10, 25, 50, 100, 500, 1000]
                                        for weightratio in [1.0, 1.1, 1.25, 1.5, 2.0, 3.0, 5.0]
                                        # for weightratio in 1.0:0.1:5.0
                                            @info "iteration ", i
                                            i += 1
                                            modelname = Symbol(string(
                                                "weightingstrategy[", weightingstrategy, "]_",
                                                "fuzzy[", fuzzy, "]_",
                                                "weighted[", weighted, "]_",
                                                "ieth[", ieth, "]_",
                                                "iee[", iee, "]_",
                                                "winnerslimit[", winnerslimit, "]_",
                                                "weightratio[", weightratio, "]_",
                                            ))
                                            push!(modelnames, modelname)
                                            time = @elapsed begin
                                                mem = @allocated result = magds_predict(
                                                    trainpath, 
                                                    testpath, 
                                                    string(target), 
                                                    weightingstrategy,
                                                    fuzzy,
                                                    weighted,
                                                    Float32(ieth), 
                                                    Int32(iee),
                                                    UInt(winnerslimit),
                                                    Float32(weightratio)
                                                )
                                            end
                                            push!(results, result); 
                                            push!(memory, mem); push!(times, time)
                                        end
                                    end
                                end
                            end
                        end
                    end
                end
            end

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