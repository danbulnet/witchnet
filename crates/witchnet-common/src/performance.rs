use statrs::statistics::{ Data, OrderStatistics };

use crate::{
    data::DataTypeValue, 
    distances::Distance
};

#[derive(Debug, Clone)]
pub struct DataProbability(pub DataTypeValue, pub f32);

pub enum SupervisedPerformance {
    Classification(SupervisedData),
    Regression(SupervisedData)
}

pub struct SupervisedData {
    references: Vec<DataTypeValue>,
    predictions: Vec<DataTypeValue>,
    probabilities: Vec<f32>
}

impl SupervisedPerformance {
    pub fn classification(
        references: Vec<DataTypeValue>, 
        predictions: Vec<DataTypeValue>, 
        probabilities: Vec<f32>
    ) -> anyhow::Result<SupervisedPerformance> {
        let ret = SupervisedPerformance::Classification(SupervisedData{ 
            references, predictions, probabilities 
        });
        match ret.is_ok() { Ok(_) => Ok(ret), Err(e) => Err(e) }
    }

    pub fn regression(
        references: Vec<DataTypeValue>, 
        predictions: Vec<DataTypeValue>,
        probabilities: Vec<f32>
    ) -> anyhow::Result<SupervisedPerformance> {
        let ret = SupervisedPerformance::Regression(SupervisedData{ 
            references, predictions, probabilities
        });
        match ret.is_ok() { Ok(_) => Ok(ret), Err(e) => Err(e) }
    }

    pub fn references(&self) -> &[DataTypeValue] {
        match self {
            Self::Classification(data) => { &data.references }
            Self::Regression(data) => { &data.references }
        }
    }
    
    pub fn predictions(&self) -> &[DataTypeValue] {
        match self {
            Self::Classification(data) => { &data.predictions }
            Self::Regression(data) => { &data.predictions }
        }
    }
    
    pub fn probabilities(&self) -> &[f32] {
        match self {
            Self::Classification(data) => { &data.probabilities }
            Self::Regression(data) => { &data.probabilities }
        }
    }

    pub fn accuracy(&self) -> anyhow::Result<f64> { 
        match self {
            Self::Classification(data) => {
                self.is_ok()?;

                let data_len = data.references.len();
                let mut total_error: f64 = 0.0;
                for i in 0..data_len {
                    total_error += data.references[i].distance(&data.predictions[i]);
                }

                Ok(1.0f64 - (total_error / data_len as f64))
            }
            Self::Regression(_) => { anyhow::bail!("accuracy is for classification only") }
        }
    }

    pub fn rmsp(&self) -> anyhow::Result<f64> {
        match self {
            Self::Classification(_) => { anyhow::bail!("rmse is for regression only") }
            Self::Regression(data) => {
                self.is_ok()?;

                let data_len = data.references.len();
                let mut total_error: f64 = 0.0;
                for i in 0..data_len {
                    total_error += (
                        data.references[i].distance(&data.predictions[i]) 
                        / data.references[i].to_f64().unwrap()
                    ).powf(2.0);
                }

                Ok((total_error / data_len as f64).sqrt())
            }
        }
    }

    pub fn nrmse(&self) -> anyhow::Result<f64> {
        match self {
            Self::Classification(_) => { anyhow::bail!("rmse is for regression only") }
            Self::Regression(data) => {
                self.is_ok()?;

                let data_len = data.references.len();
                let references_f64: Vec<f64> = (&data.references).into_iter()
                    .map(|x| x.to_f64().unwrap()).collect();
                let mut total_error: f64 = 0.0;
                for i in 0..data_len {
                    total_error += (data.references[i].distance(&data.predictions[i])).powf(2.0);
                }

                Ok(
                    (total_error / data_len as f64).sqrt() 
                    / Data::new(references_f64).interquartile_range()
                )
            }
        }
    }

    pub fn rmse(&self) -> anyhow::Result<f64> {
        match self {
            Self::Classification(_) => { anyhow::bail!("rmse is for regression only") }
            Self::Regression(data) => {
                self.is_ok()?;

                let data_len = data.references.len();
                let mut total_error: f64 = 0.0;
                for i in 0..data_len {
                    total_error += (data.references[i].distance(&data.predictions[i])).powf(2.0);
                }

                Ok((total_error / data_len as f64).sqrt())
            }
        }
    }

    pub fn mae(&self) -> anyhow::Result<f64> {
        match self {
            Self::Classification(_) => { anyhow::bail!("mae is for regression only") }
            Self::Regression(data) => {
                self.is_ok()?;

                let data_len = data.references.len();
                let mut total_error: f64 = 0.0;
                for i in 0..data_len {
                    total_error += data.references[i].distance(&data.predictions[i]);
                }

                Ok(total_error / data_len as f64)
            }
        }
    }

    pub fn mean_probability(&self) -> anyhow::Result<f32> {
        match self {
            Self::Classification(data) => { 
                self.is_ok()?;
                Ok(data.probabilities.iter().sum::<f32>() / data.probabilities.len() as f32)
            }
            Self::Regression(data) => { 
                self.is_ok()?;
                Ok(data.probabilities.iter().sum::<f32>() / data.probabilities.len() as f32)
            }
        }
    }

    pub fn is_ok(&self) -> anyhow::Result<()> {
        match self {
            Self::Classification(data) => { 
                if data.references.is_empty() { anyhow::bail!("references vec is empty") }
                let references_len = data.references.len();
                let predictions_len = data.predictions.len();
                let probabilities_len = data.probabilities.len();
                if references_len != predictions_len || references_len != probabilities_len {
                    anyhow::bail!(
                        "references ({}), predictions ({}), and
                        probabilities ({}) are not equal in length",
                        references_len, predictions_len, probabilities_len
                    )
                }
                Ok(())
            }
            Self::Regression(data) => { 
                if data.references.is_empty() { anyhow::bail!("references vec is empty") }
                let references_len = data.references.len();
                let predictions_len = data.predictions.len();
                let probabilities_len = data.probabilities.len();
                if references_len != predictions_len || references_len != probabilities_len {
                    anyhow::bail!(
                        "references ({}), predictions ({}), and
                        probabilities ({}) are not equal in length",
                        references_len, predictions_len, probabilities_len
                    )
                }
                Ok(())
            }
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::data::DataTypeValue;

    use super::*;

    #[test]
    fn regression() {
        let y_f64_ref = vec![
            DataTypeValue::F64(1.0), 
            DataTypeValue::F64(2.0), 
            DataTypeValue::F64(3.0)
        ];
        let y_f64_pred = vec![
            DataTypeValue::F64(1.5), 
            DataTypeValue::F64(2.5), 
            DataTypeValue::F64(5.0)
        ];
        let probas = vec![0.5, 1.0, 0.8];

        let performace = SupervisedPerformance::regression(
            y_f64_ref, y_f64_pred, probas
        ).unwrap();

        assert_eq!(performace.mae().unwrap(), 1.0);
        
        let rmse_result = performace.rmse().unwrap();
        assert!(rmse_result > 1.224 && rmse_result < 1.225);

        let nrmse_result = performace.nrmse().unwrap();
        assert!(nrmse_result > 0.0);

        let rmsp_result = performace.rmsp().unwrap();
        assert!(rmsp_result > 0.0);
        
        let mean_probability_result = performace.mean_probability().unwrap();
        assert!(mean_probability_result > 0.76 && mean_probability_result < 0.77);
        
        assert!(performace.accuracy().is_err());

        assert_eq!(performace.references().len(), 3);
        assert_eq!(performace.predictions().len(), 3);
        assert_eq!(performace.probabilities().len(), 3);
    }

    #[test]
    fn classification() {
        let y_arcstr_ref = vec![
            DataTypeValue::ArcStr("1.0".into()),
            DataTypeValue::ArcStr("2.0".into()), 
            DataTypeValue::ArcStr("3.0".into())
        ];
        let y_arcstr_pred = vec![
            DataTypeValue::ArcStr("1.0".into()), 
            DataTypeValue::ArcStr("2.5".into()), 
            DataTypeValue::ArcStr("5.0".into())
        ];
        let probas = vec![0.5, 1.0, 0.8];

        let performace = SupervisedPerformance::classification(
            y_arcstr_ref, y_arcstr_pred, probas
        ).unwrap();

        let accuracy_result = performace.accuracy().unwrap();
        println!("accuracy_result {accuracy_result}");
        assert!(accuracy_result > 0.33 && accuracy_result < 0.34);

        let mean_probability_result = performace.mean_probability().unwrap();
        assert!(mean_probability_result > 0.76 && mean_probability_result < 0.77);

        assert!(performace.mae().is_err());
        assert!(performace.rmse().is_err());

        assert_eq!(performace.references().len(), 3);
        assert_eq!(performace.predictions().len(), 3);
        assert_eq!(performace.probabilities().len(), 3);
    }

    
    #[test]
    fn wrong_input() {
        let performace = SupervisedPerformance::regression(vec![], vec![], vec![]);
        assert!(performace.is_err());

        let performace = SupervisedPerformance::classification(
            vec![DataTypeValue::F64(1.5), DataTypeValue::F64(1.5)], 
            vec![DataTypeValue::F64(1.5)], 
            vec![0.5]
        );
        assert!(performace.is_err());
    }
}