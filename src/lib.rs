use pyo3::prelude::*;

pub mod image;
pub mod lut;

#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use lut::{Lut, LutBuildError, LutFormat, LutSaveError};

/// A Python module for building LUTs, written in Rust.
#[pymodule]
fn gluten(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Image>()?;
    m.add_class::<Lut>()?;
    m.add_class::<LutFormat>()?;

    Ok(())
}
