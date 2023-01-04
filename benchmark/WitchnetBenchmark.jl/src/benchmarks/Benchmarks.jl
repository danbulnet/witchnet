export warmup, predictall

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
)
    Iris.classify(models=classifymodels)
    Iris.estimate(models=estimatemodels)
    Penguin.classify(models=classifymodels)
    Penguin.estimate(models=estimatemodels)
    Star.classify(models=classifymodels)
    Star.estimate(models=estimatemodels)
    WhiteWine.classify(models=classifymodels)
    WhiteWine.estimate(models=estimatemodels)
    RedWine.classify(models=classifymodels)
    RedWine.estimate(models=estimatemodels)
end