export stdmodels

using MLJ

stdmodels() = Dict(
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