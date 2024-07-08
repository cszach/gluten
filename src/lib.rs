//! A crate for building, processing, and applying LUTs (lookup tables) used in
//! color correction and color grading.
//!
//! A work in progress, gLUTen currently allows you to:
//! - [x] Generate LUTs.
//!
//! ## Python interface
//!
//! gLUTen comes with a Python interface so you can integrate it with scripts
//! for DCCs (Maya, Blender, etc.). Here is an example:
//! ```python
//! import gluten
//!
//! image = gluten.Image.load_png("edited.png")
//! lut = gluten.Lut.build(image, title="Test")
//! lut.save("lut.cube", gluten.LutFormat.CubeLut, 6, True, False)
//! ```
//!
//! ## Supported file formats
//!
//! ### Images
//!
//! - PNG
//!
//! ### LUTs
//!
//! - Cube LUT

use pyo3::prelude::*;

/// Image struct and functions for loading and saving bitmap images.
pub mod image;
/// LUT struct and functions for building, processing, and applying LUTs.
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
