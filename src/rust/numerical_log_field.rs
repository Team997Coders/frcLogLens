use crate::base_log_field::BaseLogField;

pub trait NumericalLogField: BaseLogField<ordered_float::OrderedFloat<f64>> {
    fn absolute(&self) -> Box<dyn NumericalLogField>;

    fn accumulation(&self) -> Box<dyn NumericalLogField>;

    fn change_from_initial(&self) -> Box<dyn NumericalLogField>;

    fn change_from_previous(&self) -> Box<dyn NumericalLogField>;

    fn absolute_change_from_initial(&self) -> Box<dyn NumericalLogField>;

    fn absolute_change_from_previous(&self) -> Box<dyn NumericalLogField>;

    fn finite_integral(&self, trapezoidal: bool) -> Box<dyn NumericalLogField>;

    fn finite_differential(&self) -> Box<dyn NumericalLogField>;

    fn moving_average(&self, samples: i16) -> Box<dyn NumericalLogField>;

    fn sin(&self) -> Box<dyn NumericalLogField>;

    fn cos(&self) -> Box<dyn NumericalLogField>;

    fn scalar_add(&self, scalar: f64) -> Box<dyn NumericalLogField>;

    fn scalar_power(&self, scalar: f64) -> Box<dyn NumericalLogField>;

    fn scalar_multiply(&self, scalar: f64) -> Box<dyn NumericalLogField>;

    fn add(&self, other: Box<dyn NumericalLogField>) -> Box<dyn NumericalLogField>;
    
    fn difference(&self, other: Box<dyn NumericalLogField>) -> Box<dyn NumericalLogField>;

    fn multiply(&self, other: Box<dyn NumericalLogField>) -> Box<dyn NumericalLogField>;

    fn divide(&self, other: Box<dyn NumericalLogField>) -> Box<dyn NumericalLogField>;

    fn mean(&self) -> f64;

    fn covariance(&self, other: Box<dyn NumericalLogField>) -> f64;
}