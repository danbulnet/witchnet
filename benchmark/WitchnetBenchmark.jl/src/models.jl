export classification_models, regression_models

using MLJ

classification_models() = Dict(
    :MAGDS => nothing,
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

regression_models() = Dict(
    :MAGDS => nothing,
	:DecisionTreeRegressor_BetaML => 
		@load(DecisionTreeRegressor, pkg=BetaML, verbosity=false),
	:RandomForestRegressor_ScikitLearn => 
		@load(RandomForestRegressor, pkg=ScikitLearn, verbosity=false),
	:XGBoostRegressor_XGBoost => 
		@load(XGBoostRegressor, pkg=XGBoost, verbosity=false),
	:NeuralNetworkRegressor_MLJFlux => 
		@load(NeuralNetworkRegressor, pkg=MLJFlux, verbosity=false),
	:AdaBoostRegressor_ScikitLearn => 
		@load(AdaBoostRegressor, pkg=ScikitLearn, verbosity=false),
)