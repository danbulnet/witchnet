export warmup, predictall, classifyall, estimateall, summarizeall

import Logging

include("Iris.jl")
include("Penguin.jl")
include("Star.jl")
include("WhiteWine.jl")
include("RedWine.jl")
include("BostonHousing.jl")

ALL_DATASETS = [
    Iris,
    Penguin,
    Star,
    WhiteWine,
    RedWine,
    BostonHousing
]

function warmup(dataset::Module=Iris)
    for _ in 1:5
        try
            Base.@invokelatest dataset.classify()
        catch
            continue
        end
    end
end

function predictall(
    classifymodels=fast_classification_models(),
    estimatemodels=fast_regression_models()
)::Dict{Symbol, Dict{Symbol, DataFrame}}
    Dict(
        :classification => classifyall(classifymodels),
        :regression => estimateall(estimatemodels)
    )
end

function classifyall(
    classifymodels=fast_classification_models(),
    datasets=ALL_DATASETS
)::Dict{Symbol, DataFrame}
    results = Dict{Symbol, DataFrame}()
    for dataset in datasets
        key = Symbol(lowercase(string(dataset)))
        
        Logging.disable_logging(Logging.Debug)
        @info "$key classification"
        Logging.disable_logging(Logging.Warn)

        redirect_stdout(devnull) do
            results[key] = dataset.classify(models=classifymodels)
        end
    end
    results
end

function estimateall(
    estimatemodels=fast_classification_models(),
    datasets=ALL_DATASETS
)::Dict{Symbol, DataFrame}    
    results = Dict{Symbol, DataFrame}()
    for dataset in datasets
        key = Symbol(lowercase(string(dataset)))

        Logging.disable_logging(Logging.Debug)
        @info "$key regression"
        Logging.disable_logging(Logging.Warn)

        redirect_stdout(devnull) do
            results[key] = dataset.estimate(models=estimatemodels)
        end
    end
    results
end

function summarizeall(results=predictall())::Dict{Symbol, DataFrame}
    classification = results[:classification]
    regression = results[:regression]

    classificationlen = length(values(classification))
    regressionlen = length(values(regression))

    accuracy = sum(getproperty.(values(classification), :accuracy)) / classificationlen
    time = sum(getproperty.(values(classification), :time)) / classificationlen
    memory = sum(getproperty.(values(classification), :memory)) / classificationlen
    classificationmean = DataFrame(
        :model => first(values(classification)).model,
        :accuracy => accuracy,
        :time => time,
        :memory => memory
    )

    nrmse = sum(getproperty.(values(regression), :nrmse)) / regressionlen
    time = sum(getproperty.(values(regression), :time)) / regressionlen
    memory = sum(getproperty.(values(regression), :memory)) / regressionlen
    regressionmean = DataFrame(
        :model => first(values(regression)).model,
        :nrmse => nrmse,
        :time => time,
        :memory => memory
    )

    Utils.writecsv(classificationmean, "summary", "classification", :mean)    
    title = string("classification accuracy mean of ", classificationlen, " datasets")
    plot = percent_barplot(classificationmean, :model, :accuracy, title)
    Utils.writeimg(plot, "summary", "accuracy", :mean)

    Utils.writecsv(regressionmean, "summary", "regression", :mean)
    title = string("regression nrmse mean of ", regressionlen, " datasets")
    plot = percent_barplot(regressionmean, :model, :nrmse, title)
    Utils.writeimg(plot, "summary", "nrmse", :mean)

    Dict(
        :classificationmean => classificationmean,
        :regressionmean => regressionmean
    )
end