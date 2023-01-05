export 
    fast_classification_models, classification_models,
    fast_regression_models, regression_models,
    magds_model

using MLJ
using MLJFlux
using Flux

fast_classification_models() = Dict(
    :MAGDS => nothing,
	:DecisionTreeClassifier_BetaML => 
		(ins, outs) -> @load(DecisionTreeClassifier, pkg=BetaML, verbosity=false)(),
	:RandomForestClassifier_ScikitLearn => 
		(ins, outs) -> @load(RandomForestClassifier, pkg=ScikitLearn, verbosity=false)(),
	:XGBoostClassifier_XGBoost => 
		(ins, outs) -> @load(XGBoostClassifier, pkg=XGBoost, verbosity=false)(),
	:AdaBoostClassifier_ScikitLearn => 
        (ins, outs) -> @load(AdaBoostClassifier, pkg=ScikitLearn, verbosity=false)()
)

fast_regression_models() = Dict(
    :MAGDS => nothing,
	:DecisionTreeRegressor_BetaML => 
		(ins, outs) -> @load(DecisionTreeRegressor, pkg=BetaML, verbosity=false)(),
	:RandomForestRegressor_ScikitLearn => 
		(ins, outs) -> @load(RandomForestRegressor, pkg=ScikitLearn, verbosity=false)(),
	:XGBoostRegressor_XGBoost => 
		(ins, outs) -> @load(XGBoostRegressor, pkg=XGBoost, verbosity=false)(),
    :AdaBoostRegressor_ScikitLearn => 
        (ins, outs) -> @load(AdaBoostRegressor, pkg=ScikitLearn, verbosity=false)()
)

function classification_models()
    models = fast_classification_models()
    models[:NeuralNetworkClassifier_MLJFlux] = (ins, outs) -> begin
        builder = MLJFlux.@builder begin
            Chain(
                Dense(ins => 64, relu),
                Dense(64 => 32, relu),
                Dense(32 => outs),
                softmax
            )
        end
        @load(NeuralNetworkClassifier, pkg=MLJFlux, verbosity=true)(
            builder=builder, rng=123, epochs=50, acceleration=CUDALibs()
        )
    end
    models
end

function regression_models()
    models = fast_classification_models()
    models[:NeuralNetworkRegressor_MLJFlux] = (ins, outs) -> begin
        builder = MLJFlux.@builder begin
            Chain(
                Dense(ins, 64, relu),
                Dense(64, 32, relu),
                Dense(32, outs)
            )
        end
        @load(NeuralNetworkRegressor, pkg=MLJFlux, verbosity=false)(
            builder=builder, rng=58, epochs=20
        )
    end
    models
end

magds_model() = Dict(
    :MAGDS => nothing
)

magds_gridsearch_models() = Dict(
    :MAGDS_gridsearch => nothing
)
