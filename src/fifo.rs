// use pyo3::exceptions::PyRuntimeError;
// use pyo3::class::iter::IterNextOutput;
// use pyo3::exceptions::PyKeyError;
// use pyo3::exceptions::PyStopIteration;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use pyo3::PyResult;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::cache::{Cache, Data, Key, MARKER};

#[pyclass(dict, subclass)]
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
    fn __getitem__(&self, key: &PyAny) -> PyResult<&PyObject> {
        self.base.__getitem__(Key::from(key))
    }

    fn __setitem__(&mut self, key: &PyAny, value: PyObject) -> PyResult<()> {
        // let popitem = || {
        //     let _ = self.base.pop(Key::from(key), None);
        // };
        //
        self.base.__setitem__(Key::from(key).clone(), value)
    }

    fn __delitem__(&mut self, key: &PyAny) -> PyResult<()> {
        self.base.__delitem__(Key::from(key)).and_then(|_| Ok(()))
    }

    fn __len__(&self) -> usize {
        self.base.__len__()
    }
}

#[pyproto]
impl pyo3::class::PySequenceProtocol for FIFOCache {
    fn __contains__(&self, key: &PyAny) -> bool {
        self.base.__contains__(Key::from(key))
    }
}

// #[pyproto]
// impl pyo3::class::PyIterProtocol for FIFOCache {
//     fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
//         slf
//     }
//
//     fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Key> {
//         let maybe_item = slf.base.data.iter_mut().next();
//         if maybe_item.is_none() {
//             return Err(PyStopIteration::new_err("stop iteration"));
//         }
//
//         let item = maybe_item.unwrap();
//
//         item.0.clone()
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

    fn update(&mut self, py: Python, values: PyObject) -> PyResult<()> {
        self.base.update(py, values.cast_as::<PyDict>(py)?.items())
    }

    #[args(default = "None")]
    fn get(&self, py: Python, key: &PyAny, default: Option<PyObject>) -> PyResult<PyObject> {
        self.base
            .get(Key::from(key), default.as_ref())
            .map(|t| t.clone()) // TODO: No Clone
            .or(Ok(py.None()))
    }

    #[args(default = "MARKER.get().unwrap().clone()")]
    fn pop(&mut self, py: Python, key: &PyAny, default: Option<PyObject>) -> PyResult<PyObject> {
        self.base.pop(py, Key::from(key), default)
    }

    #[args(default = "None")]
    fn setdefault(&mut self, py: Python, key: &PyAny, default: Option<PyObject>) -> PyResult<PyObject> {
        self.base
            .setdefault(py, Key::from(key), default.as_ref())
            .map(|t| t.clone()) // TODO: No Clone
    }

    #[getter]
    fn maxsize(&self) -> usize {
        self.base.maxsize
    }

    #[getter]
    fn currsize(&self) -> usize {
        self.base.currsize
    }

    #[getter]
    fn data(&self) -> Data {
        self.base.data.clone() // TODO: No Clone
    }

    #[getter]
    fn datasize(&self) -> FxHashMap<Key, usize> {
        self.base.datasize.clone() // TODO: No Clone
    }
}
