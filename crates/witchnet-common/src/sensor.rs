use std::{
    sync::{ Arc, RwLock },
    rc::Rc,
    cell::RefCell,
    fmt::{ Debug, Display, Formatter, Result as FmtResult },
    cmp::Ordering,
    any::Any,
    marker::PhantomData,
    mem
};

use anyhow::Result;

use num_traits::ToPrimitive;

use dyn_clone::DynClone;

use crate::{
    data::{ DataCategory, DataType, DataTypeValue, DataDeductor, UnknownDataTypeMarker },
    neuron::{ Neuron, NeuronAsync },
    distances::Distance, 
    connection::collective::defining::{
        DefiningWeightingStrategy, 
        DefiningWeightingStrategyAsync
    }
};

pub trait AnyCast {
    fn any(&self) -> &dyn Any;
}

impl<T> AnyCast for T 
where T: SensorData + Display + PartialOrd + PartialEq + Any + 'static {
    fn any(&self) -> &dyn Any { self }
}

pub trait SensorData: AnyCast + Display + DynClone + 'static {
    fn equals(&self, rhs: &dyn SensorData) -> bool;
    fn partial_compare(&self, rhs: &dyn SensorData) -> Option<Ordering>;
    fn distance(&self, v: &dyn SensorData) -> f64;
}

dyn_clone::clone_trait_object!(SensorData);

macro_rules! impl_sensor_data_numeric {
    ( $($t:ty),* ) => {
        $( impl SensorData for $t {
            fn equals(&self, rhs: &dyn SensorData) -> bool {
                rhs.any().downcast_ref::<$t>().map(|rhs| rhs == self).unwrap_or(false)
            }
            
            fn partial_compare(&self, rhs: &dyn SensorData) -> Option<Ordering> {
                self.partial_cmp(rhs.any().downcast_ref::<$t>().unwrap())
            }

            fn distance(&self, rhs: &dyn SensorData) -> f64 {
                let rhs = *rhs.any().downcast_ref::<$t>().unwrap();
                unsafe { 
                    (
                        Self::to_f64(self).unwrap_unchecked() - 
                        Self::to_f64(&rhs).unwrap_unchecked()
                    ).abs()
                }
            }
        }) *
    }
}

macro_rules! impl_sensor_data_categoric {
    ( $($t:ty),* ) => {
        $( impl SensorData for $t {
            fn equals(&self, rhs: &dyn SensorData) -> bool {
                rhs.any().downcast_ref::<$t>().map(|rhs| rhs == self).unwrap_or(false)
            }
            
            fn partial_compare(&self, rhs: &dyn SensorData) -> Option<Ordering> {
                self.partial_cmp(rhs.any().downcast_ref::<$t>().unwrap())
            }

            fn distance(&self, rhs: &dyn SensorData) -> f64 {
                if *self == *rhs.any().downcast_ref::<$t>().unwrap() { 0.0 } else { 1.0 }
            }
        }) *
    }
}

impl_sensor_data_numeric! { 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

impl_sensor_data_categoric! {
    String, Arc<str>, bool
}

impl SensorData for DataTypeValue {
    fn equals(&self, rhs: &dyn SensorData) -> bool {
        rhs.any().downcast_ref::<DataTypeValue>().map(|rhs| rhs == self).unwrap_or(false)
    }
    
    fn partial_compare(&self, rhs: &dyn SensorData) -> Option<Ordering> {
        self.partial_cmp(rhs.any().downcast_ref::<DataTypeValue>().unwrap())
    }

    fn distance(&self, rhs: &dyn SensorData) -> f64 {
        let rhs = match rhs.any().downcast_ref::<DataTypeValue>() {
            Some(v) => v,
            None => return 1.0
        };
        Distance::distance(self, rhs)
    }
}

impl Eq for dyn SensorData {}

impl PartialEq for dyn SensorData + '_ { 
    fn eq(&self, rhs: &Self) -> bool { self.equals(rhs) }
 }

impl PartialOrd for dyn SensorData + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        self.partial_compare(other) 
    }
}

pub trait SensorDataMarker {}

impl<T: SensorData> SensorDataMarker for T {}

impl<T: UnknownDataTypeMarker + SensorDataMarker> DataDeductor for T {
    fn data_type(&self) -> DataType { DataType::Unknown }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl<T: UnknownDataTypeMarker + SensorDataMarker> DataDeductor for PhantomData<T> {
    fn data_type(&self) -> DataType { DataType::Unknown }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

pub trait Sensor<D: SensorData>: Any + Display {
    fn id(&self) -> u32;

    fn data_type(&self) -> DataType;

    fn data_category(&self) -> DataCategory;

    fn insert(&mut self, item: &D) -> Rc<RefCell<dyn Neuron>>;

    fn insert_custom(
        &mut self, 
        item: &D, 
        weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Rc<RefCell<dyn Neuron>>;
    
    fn search(&self, item: &D) -> Option<Rc<RefCell<dyn Neuron>>>;
    
    fn fuzzy_search(
        &mut self, item: &D, threshold: f32, perserve_inserted_neuron: bool
    ) -> Option<(Rc<RefCell<dyn Neuron>>, f32)>;

    fn remove(&mut self, item: &D) -> bool;

    fn activate(
        &mut self, 
        item: &D, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<f32>;
    
    fn deactivate(
        &mut self, item: &D, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<()>;

    fn deactivate_sensor(&mut self);

    fn neurons(&self) -> Vec<Rc<RefCell<dyn Neuron>>>;

    fn values(&self) -> Vec<D>;

    // fn iterator(&self) -> Vec<D>;
}

impl<D: SensorData> Debug for dyn Sensor<D> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self)
    }
}

pub trait SensorAsync<D: SensorData>: Any + Display + Sync + Send {
    fn id(&self) -> u32;

    fn data_type(&self) -> DataType;

    fn data_category(&self) -> DataCategory;

    fn insert(&mut self, item: &D) -> Arc<RwLock<dyn NeuronAsync>>;

    fn insert_custom(
        &mut self, 
        item: &D, 
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Arc<RwLock<dyn NeuronAsync>>;
    
    fn search(&self, item: &D) -> Option<Arc<RwLock<dyn NeuronAsync>>>;

    fn fuzzy_search(
        &mut self, item: &D, threshold: f32, perserve_inserted_neuron: bool
    ) -> Option<(Arc<RwLock<dyn NeuronAsync>>, f32)>;

    fn remove(&mut self, item: &D) -> bool;

    fn activate(
        &mut self, 
        item: &D, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<f32>;
    
    fn deactivate(
        &mut self, item: &D, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<()>;

    fn deactivate_sensor(&mut self);

    fn neurons(&self) -> Vec<Arc<RwLock<dyn NeuronAsync>>>;

    fn values(&self) -> Vec<D>;

    // fn iterator(&self) -> Vec<D>;
}

impl<D: SensorData> Debug for dyn SensorAsync<D> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self)
    }
}

pub trait SensorDynamicDowncast<D: SensorData> {
    fn sensor_dynamic_downcast(
        sensor: Rc<RefCell<dyn Sensor<dyn SensorData>>>
    ) -> Rc<RefCell<dyn Sensor<D>>>;
}

impl<D: SensorData> SensorDynamicDowncast<D> for dyn Sensor<D> {
    fn sensor_dynamic_downcast(
        sensor: Rc<RefCell<dyn Sensor<dyn SensorData>>>
    ) -> Rc<RefCell<dyn Sensor<D>>> {
        unsafe { 
            mem::transmute::<
                Rc<RefCell<dyn Sensor<dyn SensorData>>>,
                Rc<RefCell<dyn Sensor<D>>>, 
            >(sensor) 
        }
    }
}

pub trait SensorStaticDowncast<S: Sensor<D>, D: SensorData>  {
    fn sensor_static_downcast(
        sensor: Rc<RefCell<dyn Sensor<dyn SensorData>>>
    ) -> *mut S;
}

impl<S: Sensor<D>, D: SensorData> 
SensorStaticDowncast<S, D> for dyn Sensor<D> {
    fn sensor_static_downcast(
        sensor: Rc<RefCell<dyn Sensor<dyn SensorData>>>
    ) -> *mut S { &*sensor.borrow() as *const _ as *mut S }
}