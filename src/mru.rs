use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::PyResult;

use itertools::Itertools;

use crate::cache::{Cache, Data, Datasize, Key, MARKER};

#[pyclass(dict, subclass)]
pub struct MRUCache {
    base: Cache,
}

#[pymethods]
impl MRUCache {
    /// Create a new MRUCache with a given ....
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
    fn get(&self, py: Python, key: Key, default: Option<PyObject>) -> PyResult<PyObject> {
        self.base.get(py, Key::from(key), default.as_ref()).map(|t| t.clone()) // TODO: No Clone
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

    fn popitem(&mut self) -> PyResult<(Key, PyObject)> {
        self.base.popitem()
    }

    #[getter]
    fn maxsize(&self) -> usize {
        self.base.maxsize
    }

    #[getter]
    fn currsize(&self) -> usize {
        self.base.currsize
    }

    // #[getter]
    // fn data(&self) -> Data {
    //     self.base.data.clone() // TODO: No Clone
    // }
    //
    // #[getter]
    // fn datasize(&self) -> Datasize {
    //     self.base.datasize.clone() // TODO: No Clone
    // }
}

// Non python imples
// impl MRUCache {}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for MRUCache {
    fn __repr__(&self) -> String {
        format!(
            "MRUCache(maxsize={}, currsize={})",
            self.base.maxsize, self.base.currsize
        )
    }
}

#[pyproto]
impl pyo3::class::PyMappingProtocol for MRUCache {
    fn __getitem__(&self, key: &PyAny) -> PyResult<&PyObject> {
        self.base.__getitem__(Key::from(key))
    }

    fn __setitem__(&mut self, key: &PyAny, value: PyObject) -> PyResult<()> {
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
impl pyo3::class::PySequenceProtocol for MRUCache {
    fn __contains__(&self, key: &PyAny) -> bool {
        self.base.__contains__(Key::from(key))
    }
}

// #[pyproto]
// impl pyo3::class::PyIterProtocol for MRUCache {
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
