use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyString};
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

    pub fn __setitem__(&mut self, key: String, value: PyObject) -> PyResult<()> {
        let maxsize = self.maxsize;
        let size = 1;
        let mut diffsize = 1;

        if !self.data.contains_key(&key) {
            while self.currsize + size > maxsize {
                let _ = self.popitem();
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

    pub fn __delitem__(&mut self, key: String) -> PyResult<PyObject> {
        let data = self.data.remove(&key);

        if let Some(datasize) = self.datasize.remove(&key) {
            self.currsize -= datasize
        }

        data.ok_or(PyKeyError::new_err(key))
    }

    pub fn __contains__(&self, key: String) -> bool {
        self.data.contains_key(&key)
    }

    pub fn __len__(&self) -> usize {
        self.data.len()
    }

    pub fn update(&mut self, py: Python, values: &PyAny) -> PyResult<()> {
        values.iter()?.try_for_each(|elt| -> PyResult<()> {
            let tuple = elt?;
            let key = tuple.get_item(0)?;
            let value = tuple.get_item(1)?;

            self.data.insert(key.to_string(), value.to_object(py));
            Ok(())
        })
    }

    pub fn get<'a>(&'a self, key: String, default: Option<&'a PyObject>) -> PyResult<&'a PyObject> {
        if let Some(value) = self.data.get(&key) {
            Ok(value)
        } else {
            default.ok_or(PyKeyError::new_err(key))
        }
    }

    pub fn pop(&mut self, key: String, default: Option<PyObject>) -> PyResult<PyObject> {
        self.__delitem__(key.clone())
            .or(default.ok_or(PyKeyError::new_err(key)))
    }

    pub fn popitem(&mut self) -> PyResult<(String, PyObject)> {
        let maybe_item = self.data.iter_mut().next();

        if maybe_item.is_none() {
            return Err(PyKeyError::new_err("stop iteration"));
        }

        let item = maybe_item.unwrap();

        let key = item.0.clone(); // TODO: no clone
        let value = self.__delitem__(key.clone())?; // TODO: no clone

        Ok((key.clone(), value))
    }

    // def setdefault(&self, key, default=None):
    //     if key in self:
    //         value = self[key]
    //     else:
    //         self[key] = value = default
    //     return value
}
