use pyo3::prelude::*;

#[pymodule]
fn my_python_lib(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    // module.add_submodule(&fs_submodule(py)?)?;

    module.add_function(wrap_pyfunction!(version, module)?)?;

    Ok(())
}

#[pyfunction]
fn version() -> PyResult<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}
