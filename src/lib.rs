use pyo3::prelude::*;

mod util;
pub use util::ListVec;

mod sentence;
pub use sentence::{PySentence, PySentenceIterator, PyToken};

mod decoder;
use decoder::{PyDecoder, PyLabel};

/// This is a Python module for wrapping the sticker sequence labeler.
#[pymodule]
fn sticker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySentence>()?;
    m.add_class::<PySentenceIterator>()?;
    m.add_class::<PyToken>()?;
    m.add_class::<PyDecoder>()?;
    m.add_class::<PyLabel>()?;
    Ok(())
}
