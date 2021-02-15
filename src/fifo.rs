use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::PyResult;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::cache::{
    Cache,
    // NONE,
    // Data,
    Key,
    MARKER,
};

#[pyclass(dict, subclass)]
pub struct FIFOCache {
    cache: Cache,
}

#[pymethods]
impl FIFOCache {
    /// Create a new FIFOCache with a given ....
    #[new]
    fn new(maxsize: usize) -> Self {
        Self {
            cache: Cache::new(maxsize),
        }
    }

    fn update(&mut self, py: Python, values: PyObject) -> PyResult<()> {
        self.cache.update(py, values.cast_as::<PyDict>(py)?.items())
    }

    #[args(default = "None")]
    fn get(&self, py: Python, key: Key, default: Option<PyObject>) -> PyResult<PyObject> {
        self.cache.get(py, Key::from(key), default.as_ref()).map(|t| t.clone()) // TODO: No Clone
    }

    #[args(default = "unsafe { MARKER.get_unchecked() }.clone()")]
    fn pop(&mut self, _py: Python, key: &PyAny, default: Option<PyObject>) -> PyResult<PyObject> {
        self.cache.pop(Key::from(key), default)
    }

    fn popitem(&mut self) -> PyResult<(Key, PyObject)> {
        self.cache.popitem()
    }

    #[args(default = "None")]
    fn setdefault(&mut self, py: Python, key: &PyAny, default: Option<PyObject>) -> PyResult<PyObject> {
        self.cache
            .setdefault(py, Key::from(key), default.as_ref())
            .map(|t| t.clone()) // TODO: No Clone
    }

    // TODO:
    // __missing__

    // TODO: Pickle
    // fn __reduce__<'py>(slf: &'py PyCell<Self>, py: Python<'py>) -> PyResult<(PyObject, &'py PyTuple, PyObject)> {
    //     println!("1");
    //     let cls = slf.to_object(py).getattr(py, "__class__")?;
    //     println!("2");
    //     let dict = slf.to_object(py).getattr(py, "__dict__")?;
    //     println!("3");
    //     Ok((cls, PyTuple::empty(py), dict))
    // }

    // TODO: Pickle
    // fn copy(&self) -> Self {
    //     println!("4");
    //     Self {
    //         base: Cache::new(self.maxsize()),
    //     }
    // }

    #[getter]
    fn maxsize(&self) -> usize {
        self.cache.maxsize
    }

    #[getter]
    fn currsize(&self) -> usize {
        self.cache.currsize
    }

    // #[getter]
    // fn data(&self) -> FxHashMap<Key, PyObject> {
    //     let mut fxmap = FxHashMap::default();
    //     for (key, value) in self.cache.data.clone() {
    //         // TODO: No Clone
    //         fxmap.insert(key, value);
    //     }
    //
    //     fxmap
    // }
    #[getter]
    fn data(&self) -> Vec<(Key, PyObject)> {
        self.cache.data.clone().into_iter().collect()
    }
}

// Non python imples
// impl FIFOCache {}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for FIFOCache {
    #[inline]
    fn __repr__(&self) -> String {
        format!(
            "FIFOCache(maxsize={}, currsize={})",
            self.cache.maxsize, self.cache.currsize
        )
    }
}

#[pyproto]
impl pyo3::class::PyMappingProtocol for FIFOCache {
    #[inline]
    fn __getitem__(&self, key: &PyAny) -> PyResult<&PyObject> {
        self.cache.__getitem__(Key::from(key))
    }

    #[inline]
    fn __setitem__(&mut self, key: &PyAny, value: PyObject) -> PyResult<()> {
        self.cache.__setitem__(Key::from(key).clone(), value)
    }

    #[inline]
    fn __delitem__(&mut self, key: &PyAny) -> PyResult<()> {
        self.cache.__delitem__(Key::from(key))
    }

    #[inline]
    fn __len__(&self) -> usize {
        self.cache.__len__()
    }
}

#[pyproto]
impl pyo3::class::PySequenceProtocol for FIFOCache {
    #[inline]
    fn __contains__(&self, key: &PyAny) -> bool {
        self.cache.__contains__(Key::from(key))
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
