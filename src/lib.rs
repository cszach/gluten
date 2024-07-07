use pyo3::prelude::*;

pub mod image;
pub mod lut;

#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use lut::Lut;

/// A Python module for building LUTs, written in Rust.
#[pymodule]
fn lut_builder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Image>()?;
    m.add_class::<Lut>()?;

    Ok(())
}
