mod base_log_field;
mod string_log_field;

use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rs_frc_log_lens(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<string_log_field::StringLogField>()?;

    Ok(())
}
