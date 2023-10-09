use pyo3::{prelude::*, exceptions::PyValueError};
pub mod structs;
pub mod authors;
pub mod errors;
use structs::{BibTeXModel, BibTeXFormatter};

#[pyfunction]
fn convert_bibtex(text: String, format: String) -> PyResult<String> {
    let model = match BibTeXModel::from_string(&text) {
        Ok(model) => model,
        Err(err) => return Err(PyErr::new::<PyValueError, _>(err.msg())),
    };
    let formatter = match BibTeXFormatter::from_string(&format) {
        Ok(formatter) => formatter,
        Err(err) => return Err(PyErr::new::<PyValueError, _>(err.msg())),
    };
    Ok(model.format(&formatter))
}

#[pyfunction]
fn convert_bibtex_vectorized(text: String, format: String) -> PyResult<Vec<String>> {
    let formatter = match BibTeXFormatter::from_string(&format) {
        Ok(formatter) => formatter,
        Err(err) => return Err(PyErr::new::<PyValueError, _>(err.msg())),
    };
    let mut out = Vec::new();
    for substr in text.split("@") {
        if substr.trim().is_empty() {
            continue;
        }
        let substr = "@".to_owned() + substr;
        let model = match BibTeXModel::from_string(&substr) {
            Ok(model) => model,
            Err(err) => return Err(PyErr::new::<PyValueError, _>(err.msg())),
        };
        out.push(model.format(&formatter));
    }
    Ok(out)
}

/// A Python module implemented in Rust.
#[pymodule]
fn _bibtexer_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_bibtex, m)?)?;
    m.add_function(wrap_pyfunction!(convert_bibtex_vectorized, m)?)?;
    Ok(())
}
