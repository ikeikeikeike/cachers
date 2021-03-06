use std::string::ToString;
use strum_macros;

use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::PyResult;

use from_variants::FromVariants;
use itertools::Itertools;
use once_cell::sync::OnceCell;

use indexmap::IndexMap;
use rustc_hash::FxHasher; // FxHashMap
use std::hash::BuildHasherDefault;
// use std::collections::HashMap;
// use hashbrown::HashMap;

#[derive(Clone, Eq, PartialEq, Hash, strum_macros::ToString, FromVariants)]
pub enum Key {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    // Key, Key
    // IntTuple(i64, i64), TODO:
    // StringIntTuple(String, i64), TODO:
    // IntStringTuple(i64, String), TODO:
}

impl From<&PyAny> for Key {
    #[inline]
    fn from(key: &PyAny) -> Self {
        if let Ok(value) = key.extract::<bool>() {
            Key::Bool(value)
        } else if let Ok(value) = key.extract::<i64>() {
            Key::Int(value)
        } else if let Ok(value) = key.extract::<String>() {
            Key::Str(value)
        } else {
            Key::Null
        }
    }
}

impl<'source> FromPyObject<'source> for Key {
    #[inline]
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        if let Ok(value) = ob.extract::<bool>() {
            Ok(Key::Bool(value))
        } else if let Ok(value) = ob.extract::<i64>() {
            Ok(Key::Int(value))
        } else if let Ok(value) = ob.extract::<String>() {
            Ok(Key::Str(value))
        } else {
            Ok(Key::Null)
        }
    }
}

impl IntoPy<PyObject> for Key {
    #[inline]
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Key::Bool(value) => value.to_object(py),
            Key::Int(value) => value.to_object(py),
            Key::Str(value) => value.to_object(py),
            _ => py.None(),
        }
    }
}

// Pool value as Default value
pub static MARKER: OnceCell<PyObject> = OnceCell::new();
// Pool value as Default value
pub static NONE: OnceCell<PyObject> = OnceCell::new();

type BuildHasher = BuildHasherDefault<FxHasher>;
pub type Data = IndexMap<Key, PyObject, BuildHasher>; // TODO: generics(FxHashMap, IndexMap)

pub struct Cache {
    /// A pool of caches
    pub data: Data,

    /// A internal counter for assigning new cache indexes
    pub currsize: usize,

    /// The maximum size of the cache.
    pub maxsize: usize,
}

impl Cache {
    /// Create a new cache with a given ....
    #[inline]
    pub fn new(maxsize: usize) -> Self {
        Self {
            data: Data::with_capacity_and_hasher(maxsize, BuildHasher::default()),
            currsize: 0,
            maxsize,
        }
    }

    #[inline]
    pub fn __repr__(&self) -> String {
        format!("Cache(maxsize={}, currsize={})", self.maxsize, self.currsize)
    }

    #[inline]
    pub fn __contains__(&self, key: Key) -> bool {
        self.data.contains_key(&key)
    }

    #[inline]
    pub fn __len__(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn __getitem__(&self, key: Key) -> PyResult<&PyObject> {
        self.data.get(&key).ok_or(PyKeyError::new_err(key))
    }

    #[inline]
    pub fn __setitem__(&mut self, key: Key, value: PyObject) -> PyResult<()> {
        if !self.data.contains_key(&key) {
            // if self.currsize >= self.maxsize {
            //     self.data.drain(..maxsize + 1 - self.currsize);
            // }
            while self.currsize >= self.maxsize {
                let _ = self.popitem();
            }
        }

        // TODO: insert every time returns None
        //
        // self.data.insert(key.clone(), value).map_or_else(
        //     || {
        //         Err(PyKeyError::new_err(key))
        //     },
        //     |_| {
        //         self.currsize += size;
        //         Ok(())
        //     },
        // )
        self.data.insert(key.clone(), value);
        self.currsize = self.data.len();

        Ok(())
    }

    #[inline]
    pub fn __delitem__(&mut self, key: Key) -> PyResult<()> {
        // XXX: use swap remove
        self.data.shift_remove(&key).map_or_else(
            || Err(PyKeyError::new_err(key)),
            |_| {
                self.currsize -= 1;
                Ok(())
            },
        )
    }

    #[inline]
    pub fn pop(&mut self, key: Key, default: Option<PyObject>) -> PyResult<PyObject> {
        // XXX: use swap remove
        // TODO: remove_entry
        self.data.shift_remove(&key).map_or_else(
            || {
                if MARKER.get() == default.as_ref() {
                    return Err(PyKeyError::new_err(key));
                }

                Ok(default.map_or_else(|| unsafe { NONE.get_unchecked() }.clone(), |elt| elt))
            },
            |elt| {
                self.currsize -= 1;
                Ok(elt)
            },
        )
    } // TODO: return key, value

    #[inline]
    pub fn popitem(&mut self) -> PyResult<(Key, PyObject)> {
        let maybe_item = self.data.iter_mut().next();

        if maybe_item.is_none() {
            return Err(PyKeyError::new_err("stop iteration"));
        }

        let item = maybe_item.unwrap();

        let key = item.0.clone(); // TODO: no clone
        let value = self.pop(key.clone(), MARKER.get().map(|elt| elt.clone()))?; // TODO: no clone

        Ok((key.clone(), value))
    }

    #[inline]
    pub fn update(&mut self, py: Python, values: &PyAny) -> PyResult<()> {
        values.iter()?.try_for_each(|elt| -> PyResult<()> {
            let tuple = elt?;
            let key = Key::from(tuple.get_item(0)?);
            let value = tuple.get_item(1)?.to_object(py);

            self.__setitem__(key, value)
        })
    }

    #[inline]
    pub fn get<'a>(&'a self, _py: Python, key: Key, default: Option<&'a PyObject>) -> PyResult<&'a PyObject> {
        if let Some(value) = self.data.get(&key) {
            Ok(value)
        } else {
            Ok(default.unwrap_or_else(|| unsafe { NONE.get_unchecked() }))
        }
    }

    #[inline]
    pub fn setdefault<'a>(&'a mut self, py: Python, key: Key, default: Option<&'a PyObject>) -> PyResult<&'a PyObject> {
        if self.data.contains_key(&key) {
            return self.__getitem__(key);
        }

        let uvalue = default.map_or_else(|| unsafe { NONE.get_unchecked() }.clone(), |elm| elm.to_object(py));
        let rvalue = default.ok_or(PyKeyError::new_err(key.to_string()));

        let _ = self.__setitem__(key, uvalue);
        rvalue
    }
}
