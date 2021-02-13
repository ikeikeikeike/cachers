mod cache;
mod fifo;
mod lru;
mod mru;

use pyo3::prelude::*;

use crate::cache::{MARKER, NONE};
use crate::fifo::FIFOCache;
use crate::lru::LRUCache;
use crate::mru::MRUCache;

#[pymodule]
fn cachers(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<FIFOCache>()?;
    m.add_class::<MRUCache>()?;
    m.add_class::<LRUCache>()?;
    // m.add_wrapped(wrap_pyfunction!(sorted))?;

    let _ = MARKER.set(py.eval("object()", None, None)?.to_object(py));
    let _ = NONE.set(py.eval("None", None, None)?.to_object(py));

    Ok(())
}
