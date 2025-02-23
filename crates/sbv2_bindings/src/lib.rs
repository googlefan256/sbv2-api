use pyo3::prelude::*;
mod sbv2;

#[pymodule]
fn sbv2_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<sbv2::TTSModel>()?;
    Ok(())
}
