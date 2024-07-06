use pyo3::prelude::*;

pub mod image;
pub mod lut;

/// A Python module for building LUTs, written in Rust.
#[pymodule]
fn lut_builder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<image::Image>()?;
    m.add_class::<lut::Lut>()?;

    Ok(())
}
