use crate::{
    data::DataTypeValue, 
    distances::Distance
};

pub enum SupervisedPerformance {
    Classification(SupervisedData),
    Regression(SupervisedData)
}

pub struct SupervisedData {
    references: Vec<DataTypeValue>,
    predictions: Vec<DataTypeValue>
}

impl SupervisedPerformance {
    pub fn classification(
        references: Vec<DataTypeValue>, predictions: Vec<DataTypeValue>
    ) -> SupervisedPerformance {
        SupervisedPerformance::Classification(SupervisedData{ references, predictions })
    }

    pub fn regression(
        references: Vec<DataTypeValue>, predictions: Vec<DataTypeValue>
    ) -> SupervisedPerformance {
        SupervisedPerformance::Regression(SupervisedData{ references, predictions })
    }

    pub fn references(&self) -> Option<&[DataTypeValue]> {
        match self {
            Self::Classification(data) => { Some(&data.references) }
            Self::Regression(data) => { Some(&data.references) }
        }
    }
    
    pub fn predictions(&self) -> Option<&[DataTypeValue]> {
        match self {
            Self::Classification(data) => { Some(&data.predictions) }
            Self::Regression(data) => { Some(&data.predictions) }
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

        let performace = SupervisedPerformance::regression(y_f64_ref, y_f64_pred);

        assert_eq!(performace.mae().unwrap(), 1.0);

        let rmse_result = performace.rmse().unwrap();
        assert!(rmse_result > 1.224 && rmse_result < 1.225);

        assert!(performace.accuracy().is_none());

        assert_eq!(performace.references().unwrap().len(), 3);
        assert_eq!(performace.predictions().unwrap().len(), 3);
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

        let performace = SupervisedPerformance::classification(y_rcstr_ref, y_rcstr_pred);

        let accuracy_result = performace.accuracy().unwrap();
        assert!(accuracy_result > 0.66 && accuracy_result < 0.67);

        assert!(performace.mae().is_none());
        assert!(performace.rmse().is_none());

        assert_eq!(performace.references().unwrap().len(), 3);
        assert_eq!(performace.predictions().unwrap().len(), 3);
    }
}