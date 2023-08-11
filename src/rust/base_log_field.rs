use std::collections::BTreeMap;

pub trait LoggableType: std::cmp::Ord + Clone {}

impl<T: std::cmp::Ord + Clone> LoggableType for T {}

pub struct BaseLogEntry<T: LoggableType> {
    value: T,
    timestamp_seconds: f64,
}

pub trait BaseLogField<T: LoggableType> {
    /**
     * Returns the timestamp of the end of the log file. This does not need to coorespond to an entry.
     */
    fn get_final_timestamp(&self) -> f64;

    /**
     * Gets a Vector of BaseLogEntries, sorted in ascending order by their timestamps.
     */
    fn get_entries_sorted(&self) -> Vec<BaseLogEntry<T>>;

    /**
     * Returns a Vector of all unique values contained in the log field.
     */
    fn get_unique_values(&self) -> Vec<T> {
        let mut unique_vals: Vec<T> = Vec::new();

        for entry in self.get_entries_sorted().iter() {
            if !unique_vals.contains(&entry.value) {
                unique_vals.push(entry.value.clone());
            }
        }
        return unique_vals;
    }

    /**
     * Returns a vector of all unique timestamps contained in the log field.
     */
    fn get_unique_timestamps(&self) -> Vec<f64> {
        let mut unique_timestamps: Vec<f64> = Vec::new();

        let mut last_timestamp: Option<f64> = None;

        for entry in self.get_entries_sorted().iter() {
            // field map is already sorted so only need to check the previous

            let mut should_push: bool = false;

            match last_timestamp {
                Some(ts) => {
                    if ts != entry.timestamp_seconds {
                        should_push = true;
                    }
                }
                _ => should_push = true,
            }

            if should_push {
                unique_timestamps.push(entry.timestamp_seconds);
                last_timestamp = Some(entry.timestamp_seconds);
            }
        }

        return unique_timestamps;
    }

    /**
     * Returns a map, sorted in ascending order, of values to the number of non-consecutive occurances they have.
     */
    fn count_unique_values(&self) -> BTreeMap<T, i32> {
        let mut output: BTreeMap<T, i32> = BTreeMap::new();

        let mut previous_value: Option<T> = None;

        for entry in self.get_entries_sorted() {
            let should_increment: bool;

            if !output.contains_key(&entry.value) {
                output.insert(entry.value.clone(), 0);
            }

            if !previous_value.is_none() {
                should_increment = previous_value.clone().unwrap() != entry.value;
                // safe unwrap
            } else {
                should_increment = true;
            }

            if should_increment {
                let count: i32 = *output.get(&entry.value).unwrap(); // safe unwrap

                output.insert(entry.value.clone(), count + 1);

                previous_value = Some(entry.value.clone());
            }
        }

        return output;
    }

    /**
     * Returns a map, sorted in ascending order, of values to the number of occurances (including consecutive) they have.
     */
    fn count_unique_values_consecutive(&self) -> BTreeMap<T, i32> {
        let mut output: BTreeMap<T, i32> = BTreeMap::new();

        return output;
    }

    /**
     * Returns a map, sorted in ascending order, of values to the cumulative amount of time that they occupy (sum of first consecutive timestamp to first timestamp of another value over the lifetime of the field).
     */
    fn time_count_unique_values(&self) -> BTreeMap<T, f64>;

    /**
     * Returns a pointer to a copy of this object, but with consecutive duplicates and entries with timestamps after the final timestamp removed.
     */
    fn squash(&self) -> Box<BaseLogEntry<T>>;
}
