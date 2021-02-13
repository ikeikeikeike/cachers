use std::cell::RefCell;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::PyResult;

// use itertools::Itertools;

use crate::cache::{
    Cache,
    // NONE,
    // Data,
    Key,
    MARKER,
};

#[pyclass(dict, subclass)]
pub struct LRUCache {
    cache: RefCell<Cache>,
}

// Non python imples
// impl LRUCache {}

#[pymethods]
impl LRUCache {
    /// Create a new LRUCache with a given ....
    #[new]
    fn new(maxsize: usize) -> Self {
        Self {
            cache: RefCell::new(Cache::new(maxsize)),
        }
    }

    fn update(&mut self, py: Python, values: PyObject) -> PyResult<()> {
        self.cache
            .borrow_mut()
            .update(py, values.cast_as::<PyDict>(py)?.items())
    }

    #[args(default = "None")]
    fn get(&self, py: Python, key: Key, default: Option<PyObject>) -> PyResult<PyObject> {
        self.cache
            .borrow()
            .get(py, Key::from(key), default.as_ref())
            .map(|t| t.clone()) // TODO: No Clone
    }

    #[args(default = "MARKER.get().unwrap().clone()")]
    fn pop(&mut self, _py: Python, key: &PyAny, default: Option<PyObject>) -> PyResult<PyObject> {
        self.cache.borrow_mut().pop(Key::from(key), default)
    }

    #[args(default = "None")]
    fn setdefault(&mut self, py: Python, key: &PyAny, default: Option<PyObject>) -> PyResult<PyObject> {
        self.cache
            .borrow_mut()
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
        self.cache.borrow_mut().popitem()
    }

    #[getter]
    fn maxsize(&self) -> usize {
        self.cache.borrow().maxsize
    }

    #[getter]
    fn currsize(&self) -> usize {
        self.cache.borrow().currsize
    }

    // #[getter]
    // fn data(&self) -> Data {
    //     self.cache.borrow().data.clone() // TODO: No Clone
    // }
    //
    // #[getter]
    // fn datasize(&self) -> Datasize {
    //     self.cache.borrow().datasize.clone() // TODO: No Clone
    // }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for LRUCache {
    fn __repr__(&self) -> String {
        format!(
            "LRUCache(maxsize={}, currsize={})",
            self.cache.borrow().maxsize,
            self.cache.borrow().currsize
        )
    }
}

#[pyproto]
impl pyo3::class::PyMappingProtocol for LRUCache {
    fn __getitem__(&self, key: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(move |py| -> PyResult<PyObject> {
            // pop index
            let value = self
                .cache
                .borrow_mut()
                .pop(Key::from(key), MARKER.get().map(|elt| elt.clone()))?;
            // set last
            let _ = self.cache.borrow_mut().__setitem__(Key::from(key), value.to_object(py));

            Ok(value)
        })
    }

    fn __setitem__(&mut self, key: &PyAny, value: PyObject) -> PyResult<()> {
        self.cache.borrow_mut().__setitem__(Key::from(key).clone(), value)
    }

    fn __delitem__(&mut self, key: &PyAny) -> PyResult<()> {
        self.cache.borrow_mut().__delitem__(Key::from(key))
    }

    fn __len__(&self) -> usize {
        self.cache.borrow().__len__()
    }
}

#[pyproto]
impl pyo3::class::PySequenceProtocol for LRUCache {
    fn __contains__(&self, key: &PyAny) -> bool {
        self.cache.borrow().__contains__(Key::from(key))
    }
}

// #[pyproto]
// impl pyo3::class::PyIterProtocol for LRUCache {
//     fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
//         slf
//     }
//
//     fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Key> {
//         let maybe_item = slf.base.borrow().data.iter_mut().next();
//         if maybe_item.is_none() {
//             return Err(PyStopIteration::new_err("stop iteration"));
//         }
//
//         let item = maybe_item.unwrap();
//
//         item.0.clone()
//     }
// }
