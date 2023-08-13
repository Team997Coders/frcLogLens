mod base_log_field;
mod numerical_log_field;

use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rs_frc_log_lens(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
