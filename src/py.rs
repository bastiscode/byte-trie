use crate::{AdaptiveRadixTrie, PatriciaTrie, PrefixSearch};
use pyo3::prelude::*;

#[pyclass]
#[pyo3(name = "PatriciaTrie")]
struct PyPatriciaTrie {
    inner: PatriciaTrie<PyObject>,
}

#[pymethods]
impl PyPatriciaTrie {
    #[new]
    fn new() -> Self {
        Self {
            inner: PatriciaTrie::default(),
        }
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn insert(&mut self, key: &[u8], value: PyObject) -> Option<PyObject> {
        self.inner.insert(key, value)
    }

    fn delete(&mut self, key: &[u8]) -> Option<PyObject> {
        self.inner.delete(key)
    }

    fn get(&self, key: &[u8]) -> Option<&PyObject> {
        self.inner.get(key)
    }

    fn contains(&self, key: &[u8]) -> bool {
        self.inner.contains(key)
    }

    fn contains_prefix(&self, prefix: &[u8]) -> bool {
        self.inner.contains_prefix(prefix)
    }

    fn prefix_matches(&self, key: &[u8]) -> Vec<(usize, &PyObject)> {
        self.inner.prefix_matches(key)
    }

    fn continuations(&self, prefix: &[u8]) -> Vec<(Vec<u8>, &PyObject)> {
        self.inner.continuations(prefix).collect()
    }
}

#[pyclass]
#[pyo3(name = "AdaptiveRadixTrie")]
struct PyAdaptiveRadixTrie {
    inner: AdaptiveRadixTrie<PyObject>,
}

#[pymethods]
impl PyAdaptiveRadixTrie {
    #[new]
    fn new() -> Self {
        Self {
            inner: AdaptiveRadixTrie::default(),
        }
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn insert(&mut self, key: &[u8], value: PyObject) -> Option<PyObject> {
        self.inner.insert(key, value)
    }

    fn delete(&mut self, key: &[u8]) -> Option<PyObject> {
        self.inner.delete(key)
    }

    fn get(&self, key: &[u8]) -> Option<&PyObject> {
        self.inner.get(key)
    }

    fn contains(&self, key: &[u8]) -> bool {
        self.inner.contains(key)
    }

    fn contains_prefix(&self, prefix: &[u8]) -> bool {
        self.inner.contains_prefix(prefix)
    }

    fn prefix_matches(&self, key: &[u8]) -> Vec<(usize, &PyObject)> {
        self.inner.prefix_matches(key)
    }

    fn continuations(&self, prefix: &[u8]) -> Vec<(Vec<u8>, &PyObject)> {
        self.inner.continuations(prefix).collect()
    }
}

#[pymodule]
fn _internal(_: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPatriciaTrie>()?;
    m.add_class::<PyAdaptiveRadixTrie>()?;
    Ok(())
}
