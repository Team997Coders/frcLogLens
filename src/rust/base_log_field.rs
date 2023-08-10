use std::collections::BTreeMap;

pub trait BaseLogField<T> {
    fn get_all_entries(&self) -> BTreeMap<T, f64>;

    fn get_unique_values(&self) -> Vec<T>;

    fn get_unique_timestamps(&self) -> Vec<f64>;

    fn count_unique_values(&self) -> BTreeMap<T, i32>;

    fn time_count_unique_values(&self) -> BTreeMap<T, f64>;
}