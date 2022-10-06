use crate::{
    data::DataTypeValue, 
    distances::Distance
};

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
    ) -> SupervisedPerformance {
        SupervisedPerformance::Classification(SupervisedData{ 
            references, predictions, probabilities 
        })
    }

    pub fn regression(
        references: Vec<DataTypeValue>, 
        predictions: Vec<DataTypeValue>,
        probabilities: Vec<f32>
    ) -> SupervisedPerformance {
        SupervisedPerformance::Regression(SupervisedData{ 
            references, predictions, probabilities
        })
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

    pub fn accuracy(&self) -> Option<f64> { 
        match self {
            Self::Classification(data) => {
                let data_len = data.references.len();
                assert_eq!(data_len, data.predictions.len());

                let mut total_error: f64 = 0.0;
                for i in 0..data_len {
                    total_error += data.references[i].distance(&data.predictions[i]);
                }

                Some(total_error / data_len as f64)
            }
            Self::Regression(_) => { None }
        }
    }

    pub fn rmse(&self) -> Option<f64> {
        match self {
            Self::Classification(_) => { None }
            Self::Regression(data) => {
                let data_len = data.references.len();
                assert_eq!(data_len, data.predictions.len());

                let mut total_error: f64 = 0.0;
                for i in 0..data_len {
                    total_error += (data.references[i].distance(&data.predictions[i])).powf(2.0);
                }

                Some((total_error / data_len as f64).sqrt())
            }
        }
    }

    pub fn mae(&self) -> Option<f64> {
        match self {
            Self::Classification(_) => { None }
            Self::Regression(data) => {
                let data_len = data.references.len();
                assert_eq!(data_len, data.predictions.len());

                let mut total_error: f64 = 0.0;
                for i in 0..data_len {
                    total_error += data.references[i].distance(&data.predictions[i]);
                }

                Some(total_error / data_len as f64)
            }
        }
    }
}

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

        let performace = SupervisedPerformance::regression(y_f64_ref, y_f64_pred, probas);

        assert_eq!(performace.mae().unwrap(), 1.0);

        let rmse_result = performace.rmse().unwrap();
        assert!(rmse_result > 1.224 && rmse_result < 1.225);

        assert!(performace.accuracy().is_none());

        assert_eq!(performace.references().len(), 3);
        assert_eq!(performace.predictions().len(), 3);
        assert_eq!(performace.probabilities().len(), 3);
    }

    #[test]
    fn classification() {
        let y_rcstr_ref = vec![
            DataTypeValue::RcStr("1.0".into()),
            DataTypeValue::RcStr("2.0".into()), 
            DataTypeValue::RcStr("3.0".into())
        ];
        let y_rcstr_pred = vec![
            DataTypeValue::RcStr("1.0".into()), 
            DataTypeValue::RcStr("2.5".into()), 
            DataTypeValue::RcStr("5.0".into())
        ];
        let probas = vec![0.5, 1.0, 0.8];

        let performace = SupervisedPerformance::classification(y_rcstr_ref, y_rcstr_pred, probas);

        let accuracy_result = performace.accuracy().unwrap();
        assert!(accuracy_result > 0.66 && accuracy_result < 0.67);

        assert!(performace.mae().is_none());
        assert!(performace.rmse().is_none());

        assert_eq!(performace.references().len(), 3);
        assert_eq!(performace.predictions().len(), 3);
        assert_eq!(performace.probabilities().len(), 3);
    }
}