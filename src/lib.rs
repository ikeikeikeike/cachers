use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// mod free;
// use crate::free::*;

#[pymodule]
fn cachers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    // m.add_wrapped(wrap_pyfunction!(all))?;
    // m.add_wrapped(wrap_pyfunction!(any))?;
    // m.add_wrapped(wrap_pyfunction!(max))?;
    // m.add_wrapped(wrap_pyfunction!(join))?;
    // m.add_wrapped(wrap_pyfunction!(sorted))?;

    Ok(())
}
