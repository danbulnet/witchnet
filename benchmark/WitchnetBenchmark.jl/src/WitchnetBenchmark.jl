module WitchnetBenchmark

include("Utils.jl")
include("magds.jl")
include("predeval.jl")
include("plots.jl")
include("models.jl")
include("benchmarks/Benchmarks.jl")

end # module WitchnetBenchmark
# acceleration=CUDALibs()
# MLJ.default_resource(CPUProcesses())