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

function predictall(models=fast_classification_models())
    Iris.classify(models=models)
    Iris.estimate(models=models)
    Penguin.classify(models=models)
    Penguin.classify(models=models)
    Star.estimate(models=models)
    Star.estimate(models=models)
    WhiteWine.estimate(models=models)
    WhiteWine.estimate(models=models)
    RedWine.estimate(models=models)
    RedWine.estimate(models=models)
end