use crate::{
    data::DataTypeValue, 
    distances::Distance
};

pub enum Performance {
    Classification(SupervisedData),
    Regression(SupervisedData),
    Clustering(ClusteringData)
}

pub struct SupervisedData {
    pub references: Vec<DataTypeValue>,
    pub predictions: Vec<DataTypeValue>
}

pub struct ClusteringData {
    pub clusters: Vec<DataTypeValue>
}

impl Performance {
    pub fn classification(
        references: Vec<DataTypeValue>, predictions: Vec<DataTypeValue>
    ) -> Performance {
        Performance::Classification(SupervisedData{ references, predictions })
    }

    pub fn regression(
        references: Vec<DataTypeValue>, predictions: Vec<DataTypeValue>
    ) -> Performance {
        Performance::Regression(SupervisedData{ references, predictions })
    }

    pub fn clustering(
        clusters: Vec<DataTypeValue>
    ) -> Performance {
        Performance::Clustering(ClusteringData { clusters })
    }

    pub fn references(&self) -> Option<&[DataTypeValue]> {
        match self {
            Self::Classification(data) => { Some(&data.references) }
            Self::Regression(data) => { Some(&data.references) }
            Self::Clustering(_) => { None }
        }
    }
    
    pub fn predictions(&self) -> Option<&[DataTypeValue]> {
        match self {
            Self::Classification(data) => { Some(&data.predictions) }
            Self::Regression(data) => { Some(&data.predictions) }
            Self::Clustering(data) => { Some(&data.clusters) }
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
            Self::Clustering(_) => { None }
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
            Self::Clustering(_) => { None }
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
            Self::Clustering(_) => { None }
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

        let performace = Performance::regression(y_f64_ref, y_f64_pred);

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

        let performace = Performance::classification(y_rcstr_ref, y_rcstr_pred);

        let accuracy_result = performace.accuracy().unwrap();
        assert!(accuracy_result > 0.66 && accuracy_result < 0.67);

        assert!(performace.mae().is_none());
        assert!(performace.rmse().is_none());

        assert_eq!(performace.references().unwrap().len(), 3);
        assert_eq!(performace.predictions().unwrap().len(), 3);
    }

    #[test]
    fn clustering() {
        let y_usize_pred = vec![
            DataTypeValue::USize(1), 
            DataTypeValue::USize(2), 
            DataTypeValue::USize(5)
        ];

        let performace = Performance::clustering(y_usize_pred);

        assert!(performace.accuracy().is_none());
        assert!(performace.mae().is_none());
        assert!(performace.rmse().is_none());
        assert!(performace.references().is_none());

        assert_eq!(performace.predictions().unwrap().len(), 3);
    }
}