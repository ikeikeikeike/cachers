use std::string::ToString;
use strum_macros;

use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::PyResult;

use from_variants::FromVariants;
use rustc_hash::FxHashMap;

#[derive(Clone, Eq, PartialEq, Hash, strum_macros::ToString, FromVariants)]
pub enum Key {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    // Array(Vec<Value>),
    // Object(Map<String, Value>),
}

pub struct Cache {
    /// A pool of caches
    pub data: FxHashMap<Key, PyObject>,

    /// A pool of clients size
    pub datasize: FxHashMap<Key, usize>,

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

    pub fn __getitem__(&self, key: Key) -> PyResult<&PyObject> {
        self.data
            .get(&key)
            .ok_or(PyKeyError::new_err(key.to_string()))
    }

    pub fn __setitem__(&mut self, key: Key, value: PyObject) -> PyResult<()> {
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

    pub fn __delitem__(&mut self, key: Key) -> PyResult<PyObject> {
        let data = self.data.remove(&key);

        if let Some(datasize) = self.datasize.remove(&key) {
            self.currsize -= datasize
        }

        data.ok_or(PyKeyError::new_err(key.to_string()))
    }

    pub fn __contains__(&self, key: Key) -> bool {
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

            self.data.insert(key, value.to_object(py));
            Ok(())
        })
    }

    pub fn get<'a>(&'a self, key: Key, default: Option<&'a PyObject>) -> PyResult<&'a PyObject> {
        if let Some(value) = self.data.get(&key) {
            Ok(value)
        } else {
            default.ok_or(PyKeyError::new_err(key.to_string()))
        }
    }

    pub fn pop(&mut self, key: Key, default: Option<PyObject>) -> PyResult<PyObject> {
        self.__delitem__(key.clone())
            .or(default.ok_or(PyKeyError::new_err(key.to_string())))
    }

    pub fn popitem(&mut self) -> PyResult<(Key, PyObject)> {
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
