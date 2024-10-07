use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

#[pymodule]
fn my_python_lib(_: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    // module.add_submodule(&fs_submodule(py)?)?;

    module.add_function(wrap_pyfunction!(version, module)?)?;

    Ok(())
}

#[gen_stub_pyfunction]
#[pyfunction]
fn version() -> PyResult<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}
