module Utils

export Utils

using CSV
using DataFrames
using Gadfly
using PythonCall
using CondaPkg
using Pkg

CondaPkg.add("cairosvg")
Pkg.build("PyCall")

function writecsv(df::DataFrame, dataset::String, task::String, target::Symbol)
    path = normpath(joinpath(@__DIR__, "..", "benchmark", dataset))
    mkpath(path)
    filename = string(dataset, "_", task, "_",  lowercase(string(target)),".csv")
    CSV.write(joinpath(path, filename), df)
end

function writeimg(plot, dataset::String, task::String, target::Symbol)
    path = normpath(joinpath(@__DIR__, "..", "benchmark", dataset))
    mkpath(path)
    filename = string(dataset, "_", task, "_",  lowercase(string(target)))
    draw(SVG(joinpath(path, filename * ".svg")), plot)
    svg2png(joinpath(path, filename * ".svg"), joinpath(path, filename * ".png"))
end

function svg2png(filein::String, fileout::String)
    cairosvg = pyimport("cairosvg")
    cairosvg.svg2png(url=filein, write_to=fileout, dpi=600)
end

end