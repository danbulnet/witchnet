### A Pluto.jl notebook ###
# v0.19.15

using Markdown
using InteractiveUtils

# ╔═╡ 0884e910-67f5-11ed-3f4b-37aac5be50c7
import Pkg; Pkg.activate("..")

# ╔═╡ d72a57f8-b9bf-4a92-9103-d9b754ff9c2d
begin
	using RDatasets
	using MLJ
	using WitchnetBenchmark
	using Gadfly, Compose
	using DataFrames
	using ColorSchemes
end

# ╔═╡ fb4164c0-a3d6-4a7e-b0da-6ea736b722b2
begin
	iris = RDatasets.dataset("datasets", "iris")
	y, X = MLJ.unpack(iris, ==(:Species), colname -> true)
	nothing
end

# ╔═╡ 5bae5800-20b4-4431-8d60-5783fc620899
models = Dict(
	:DecisionTreeClassifier_BetaML => 
		@load(DecisionTreeClassifier, pkg=BetaML, verbosity=false),
	:RandomForestClassifier_ScikitLearn => 
		@load(RandomForestClassifier, pkg=ScikitLearn, verbosity=false),
	:XGBoostClassifier_XGBoost => 
		@load(XGBoostClassifier, pkg=XGBoost, verbosity=false),
	:NeuralNetworkClassifier_MLJFlux => 
		@load(NeuralNetworkClassifier, pkg=MLJFlux, verbosity=false),
	:AdaBoostClassifier_ScikitLearn => 
		@load(AdaBoostClassifier, pkg=ScikitLearn, verbosity=false),
)

# ╔═╡ d35ec144-e410-43d7-ba18-6cc466a3247f
# ╠═╡ show_logs = false
begin
	accuracies = []
	for (name, model) in models
	    push!(accuracies, predeval(model, X, y, :accuracy))
	end
end

# ╔═╡ dece9c81-3978-4eaf-80bb-f28632edb7b5
begin
	accdf = DataFrame(:model => collect(keys(models)), :accuracy => accuracies)
	sort!(accdf, :accuracy, rev=true)
end

# ╔═╡ b53c8c0f-acc6-4f74-9769-30e034250c98
begin
	set_default_plot_size(18cm, 10cm)
	p = ColorSchemes.seaborn_colorblind6
	accplot = plot(
		accdf, y=:model, x=:accuracy, 
		color=:model, 
		Coord.cartesian(yflip=true), Scale.y_discrete,
		Geom.bar(position=:dodge, orientation=:horizontal),
		Theme(bar_spacing=2.0mm, key_position=:none),
		Scale.color_discrete_manual(p...),
		Guide.title("iris species classification accuracy"),
		Guide.xlabel("classification accuracy"), Guide.ylabel("model"),
		Guide.annotation(
			compose(
				context(),
				text(
		            accdf.accuracy .- 0.03,
		            1:length(accdf.accuracy),
		            string.(round.(accdf.accuracy .* 100, digits=3), "%"),
		            [hright for x in accdf.accuracy],
		        ),
		        fontsize(2.5),
		        fill("black")
			)
		)
	)
	# hstack(accplot)
end

# ╔═╡ Cell order:
# ╠═0884e910-67f5-11ed-3f4b-37aac5be50c7
# ╠═d72a57f8-b9bf-4a92-9103-d9b754ff9c2d
# ╠═fb4164c0-a3d6-4a7e-b0da-6ea736b722b2
# ╠═5bae5800-20b4-4431-8d60-5783fc620899
# ╠═d35ec144-e410-43d7-ba18-6cc466a3247f
# ╠═dece9c81-3978-4eaf-80bb-f28632edb7b5
# ╠═b53c8c0f-acc6-4f74-9769-30e034250c98
