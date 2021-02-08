use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
// use pyo3::types::{PyDict, PyString};
use pyo3::PyResult;
use rustc_hash::FxHashMap;

pub struct Cache {
    /// A pool of caches
    pub data: FxHashMap<String, PyObject>,

    /// A pool of clients size
    pub datasize: FxHashMap<String, usize>,

    /// A internal counter for assigning new cache indexes
    pub currsize: usize,

    /// The maximum size of the cache.
    pub maxsize: usize,
}

// Non python imples
// impl Cache {}

impl Cache {
    /// Create a new cache with a given ....
    pub fn new(maxsize: usize) -> Self {
        Self {
            data: FxHashMap::default(),
            datasize: FxHashMap::default(),
            currsize: 0,
            maxsize,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Cache(maxsize={}, currsize={})",
            self.maxsize, self.currsize
        )
    }

    pub fn __getitem__(&self, key: String) -> PyResult<&PyObject> {
        self.data.get(&key).ok_or(PyKeyError::new_err(key))
    }

    pub fn __setitem__(&mut self, key: String, value: PyObject, popitem: fn()) -> PyResult<()> {
        let maxsize = self.maxsize;
        let size = 1;
        let mut diffsize = 1;

        if !self.data.contains_key(&key) {
            while self.currsize + size > maxsize {
                popitem()
            }
        }

        if let Some(datasize) = self.datasize.get(&key) {
            diffsize = size - datasize
        }

        self.data.insert(key.clone(), value);
        self.datasize.insert(key, size);
        self.currsize += diffsize;
        Ok(())
    }

    pub fn __delitem__(&mut self, key: String) {
        self.data.remove(&key);
        if let Some(datasize) = self.datasize.remove(&key) {
            self.currsize -= datasize
        }
    }

    pub fn __contains__(&self, key: String) -> bool {
        self.data.contains_key(&key)
    }

    pub fn __len__(&self) -> usize {
        self.data.len()
    }

    pub fn get<'a>(&'a self, key: String, default: Option<&'a PyObject>) -> PyResult<&'a PyObject> {
        if let Some(value) = self.data.get(&key) {
            Ok(value)
        } else {
            default.ok_or(PyKeyError::new_err(key))
        }
    }

    pub fn pop(&self, key: String) -> PyResult<()> {
        // self.get(key)
        Ok(())
    }

    // def setdefault(&self, key, default=None):
    //     if key in self:
    //         value = self[key]
    //     else:
    //         self[key] = value = default
    //     return value
}
