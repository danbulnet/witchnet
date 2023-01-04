export warmup, predictall, classifyall, estimateall, summarizeall

include("Iris.jl")
include("Penguin.jl")
include("Star.jl")
include("WhiteWine.jl")
include("RedWine.jl")

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

function classifyall(classifymodels=fast_classification_models())::Dict{Symbol, DataFrame}
    Dict(
        :iris => Iris.classify(models=classifymodels),
        :penguin => Penguin.classify(models=classifymodels),
        :star => Star.classify(models=classifymodels),
        :whitewine => WhiteWine.classify(models=classifymodels),
        :redwine => RedWine.classify(models=classifymodels)
    )
end

function estimateall(estimatemodels=fast_classification_models())::Dict{Symbol, DataFrame}
    Dict(        
        :iris => Iris.estimate(models=estimatemodels),
        :penguin => Penguin.estimate(models=estimatemodels),
        :star => Star.estimate(models=estimatemodels),
        :whitewine => WhiteWine.estimate(models=estimatemodels),
        :redwine => RedWine.estimate(models=estimatemodels)
    )
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

    rmse = sum(getproperty.(values(regression), :rmse)) / regressionlen
    time = sum(getproperty.(values(regression), :time)) / regressionlen
    memory = sum(getproperty.(values(regression), :memory)) / regressionlen
    regressionmean = DataFrame(
        :model => first(values(regression)).model,
        :rmse => rmse,
        :time => time,
        :memory => memory
    )

    Utils.writecsv(classificationmean, "summary", "classification", :mean)    
    title = string("classification accuracy mean of ", classificationlen, " datasets")
    plot = percent_barplot(classificationmean, :model, :accuracy, title)
    Utils.writeimg(plot, "summary", "accuracy", :mean)

    Utils.writecsv(regressionmean, "summary", "regression", :mean)
    title = string("regression rmse mean of ", regressionlen, " datasets")
    plot = value_barplot(regressionmean, :model, :rmse, title)
    Utils.writeimg(plot, "summary", "rmse", :mean)

    Dict(
        :classificationmean => classificationmean,
        :regressionmean => regressionmean
    )
end