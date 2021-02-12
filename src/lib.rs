mod cache;
mod fifo;

use pyo3::prelude::*;

use crate::cache::MARKER;
use crate::fifo::FIFOCache;

#[pymodule]
fn cachers(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<FIFOCache>()?;
    // m.add_wrapped(wrap_pyfunction!(all))?;
    // m.add_wrapped(wrap_pyfunction!(any))?;
    // m.add_wrapped(wrap_pyfunction!(max))?;
    // m.add_wrapped(wrap_pyfunction!(join))?;
    // m.add_wrapped(wrap_pyfunction!(sorted))?;

    let _ = MARKER.set(py.eval("object()", None, None)?.to_object(py));

    Ok(())
}
