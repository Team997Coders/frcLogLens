use pyo3::pyclass;

use crate::base_log_field::{BaseLogEntry, BaseLogField};

use std::collections::BTreeMap;

#[pyclass]
pub struct StringLogField {}

impl BaseLogField<String> for StringLogField {
    fn get_final_timestamp(&self) -> f64 {
        todo!()
    }

    fn get_entries_sorted(&self) -> Vec<BaseLogEntry<String>> {
        todo!()
    }

    fn count_unique_values(&self) -> BTreeMap<String, i32> {
        todo!()
    }

    fn time_count_unique_values(&self) -> BTreeMap<String, f64> {
        todo!()
    }

    fn squash(&self) -> Box<BaseLogEntry<String>> {
        todo!()
    }
}
