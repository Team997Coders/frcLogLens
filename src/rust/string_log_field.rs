use pyo3::pyclass;

use crate::base_log_field::BaseLogField;

use std::collections::BTreeMap;

#[pyclass]
pub struct StringLogField {}

impl BaseLogField<String> for StringLogField {
    fn get_all_entries(&self) -> BTreeMap<String, f64> {
        todo!()
    }

    fn get_unique_values(&self) -> Vec<String> {
        todo!()
    }

    fn get_unique_timestamps(&self) -> Vec<f64> {
        todo!()
    }

    fn count_unique_values(&self) -> BTreeMap<String, i32> {
        todo!()
    }

    fn time_count_unique_values(&self) -> BTreeMap<String, f64> {
        todo!()
    }
}