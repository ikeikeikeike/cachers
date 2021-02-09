// use pyo3::exceptions::PyRuntimeError;
use pyo3::exceptions::PyKeyError;
use rustc_hash::FxHashMap;
// use pyo3::exceptions::{PyKeyError, ValueError};
use pyo3::types::{PyDict, PyString};
// use pyo3::class::iter::IterNextOutput;
use pyo3::prelude::*;
use pyo3::PyResult;

use crate::cache::Cache;

#[pyclass(dict)]
pub struct FIFOCache {
    base: Cache,
}

// Non python imples
// impl FIFOCache {}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for FIFOCache {
    fn __repr__(&self) -> String {
        format!(
            "FIFOCache(maxsize={}, currsize={})",
            self.base.maxsize, self.base.currsize
        )
    }
}

#[pyproto]
impl pyo3::class::PyMappingProtocol for FIFOCache {
    fn __getitem__(&self, key: String) -> PyResult<&PyObject> {
        self.base.__getitem__(key)
    }

    fn __setitem__(&mut self, key: String, value: PyObject) -> PyResult<()> {
        self.base.__setitem__(key, value, || println!("popitem"))
    }

    fn __delitem__(&mut self, key: String) {
        self.base.__delitem__(key)
    }

    fn __len__(&self) -> usize {
        self.base.__len__()
    }
}

#[pyproto]
impl pyo3::class::PySequenceProtocol for FIFOCache {
    fn __contains__(&self, key: String) -> bool {
        self.base.__contains__(key)
    }
}

// #[pyproto]
// impl pyo3::class::PyIterProtocol for FIFOCache {
//     fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
//         slf.base.data.keys().copied().collect();
//         slf
//     }
//
//     fn __next__(slf: PyRefMut<Self>) -> IterNextOutput<String, &'static str> {
//         let keys = slf.base.data.keys();
//
//         for key in keys {
//             println!("{}", key);
//             return IterNextOutput::Yield(String::from(key));
//         }
//
//         IterNextOutput::Return("ended items")
//     }
// }

#[pymethods]
impl FIFOCache {
    /// Create a new FIFOCache with a given ....
    #[new]
    fn new(maxsize: usize) -> Self {
        Self {
            base: Cache::new(maxsize),
        }
    }

    pub fn update(&mut self, py: Python, values: PyObject) -> PyResult<()> {
        self.base.update(py, values.cast_as::<PyDict>(py)?.items())
    }

    #[args(default = "None")]
    pub fn get(&self, py: Python, key: String, default: Option<PyObject>) -> PyResult<PyObject> {
        self.base
            .get(key, default.as_ref())
            .map(|t| t.clone()) // TODO: No Clone
            .or(Ok(py.None()))
    }

    // pub fn pop(&self, key: String) -> PyResult<&PyObject> {
    //     self.base.pop(key)
    // }

    #[getter]
    fn maxsize(&self) -> usize {
        self.base.maxsize
    }

    #[getter]
    fn currsize(&self) -> usize {
        self.base.currsize
    }

    // TODO
    // #[getter]
    // fn data(&self) -> FxHashMap<String, PyObject> {
    //     self.base.data
    // }

    // TODO
    // #[getter]
    // fn datasize(&self) -> FxHashMap<String, usize> {
    //     self.base.datasize
    // }
}
