pub mod formats;

use crate::image::Image;
use formats::{CubeLutFormat, LutSave};
use pyo3::{exceptions::PyTypeError, prelude::*};

pub use formats::LutFormat;

/// The type of the LUT i.e. 1D or 3D.
#[pyclass]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum LutType {
    /// 1D LUT.
    Lut1d,
    /// 3D LUT.
    Lut3d,
}

impl Default for LutType {
    fn default() -> Self {
        LutType::Lut3d
    }
}

/// LUT.
#[pyclass]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Lut {
    /// The type of the LUT (1D or 3D).
    #[pyo3(get)]
    pub lut_type: LutType,
    /// The title of the LUT (optional).
    #[pyo3(get)]
    pub title: Option<String>,
    /// The size of the LUT i.e. the number of entries (for a 1D LUT) or the
    /// number of entries per channel (for a 3D LUT).
    #[pyo3(get)]
    pub size: u16,
    /// The input range of the LUT.
    #[pyo3(get)]
    pub input_range: (f32, f32),
    /// The data specifying the entries of the LUT, starting with the entry that
    /// corresponds to the minimum input value and ends with the entry that
    /// corresponds to the maximum input value. Every 3 floating-point numbers
    /// constitute an entry.
    #[pyo3(get)]
    pub data: Vec<f32>,
}

/// Contains possible errors during the LUT build process.
#[pyclass]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum LutBuildError {
    /// The edited image used was invalid e.g. the sizes are invalid, the number
    /// of channels is invalid, or something else.
    InvalidEditedImage,
}

impl From<LutBuildError> for PyErr {
    fn from(rust_error: LutBuildError) -> Self {
        match rust_error {
            LutBuildError::InvalidEditedImage => {
                PyTypeError::new_err("Invalid edited image received for building a LUT")
            }
        }
    }
}

/// Contains possible errors during the LUT saving process.
#[pyclass]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum LutSaveError {
    /// An I/O error while trying to save the LUT e.g. the parent directory does
    /// not exist, or the user does not have write permissions in the path.
    IoError,
}

impl From<std::io::Error> for LutSaveError {
    fn from(_: std::io::Error) -> Self {
        LutSaveError::IoError
    }
}

impl From<LutSaveError> for PyErr {
    fn from(rust_error: LutSaveError) -> Self {
        match rust_error {
            LutSaveError::IoError => PyTypeError::new_err("IO error while saving LUT"),
        }
    }
}

/// The required width of the edited image used for LUT generation, typically
/// has to match the width of the original image.
pub const REQUIRED_WIDTH: u32 = 256;
/// The required height of the edited image used for LUT generation, typically
/// has to match the height of the original image.
pub const REQUIRED_HEIGHT: u32 = 128;

#[pymethods]
impl Lut {
    /// Build a LUT given an edited image.
    ///
    /// The edited image is expected to be an RGB/RGBA PNG with specifc width
    /// and height (currently 256 and 128), and contains the colors that
    /// correspond to the input colors in a LUT in a row-major order i.e. the
    /// top left pixel corresponds to the lowest input value of the LUT, and
    /// increases to the right and advances to the bottom with the red channel
    /// rotating the fastest, and the bottom right pixel corresponds to the
    /// highest input value.
    ///
    /// Arguments:
    /// * `edited_image` - The [`Image`] in the format described above.
    /// * `title` - The title for the LUT (optional).
    #[staticmethod]
    #[pyo3(signature = (edited_image, title=None))]
    pub fn build(edited_image: &Image, title: Option<String>) -> Result<Lut, LutBuildError> {
        if edited_image.width != REQUIRED_WIDTH || edited_image.height != REQUIRED_HEIGHT {
            return Err(LutBuildError::InvalidEditedImage);
        }

        let size = ((edited_image.width * edited_image.height) as f32).cbrt();
        let size = if size == ((size as u16) as f32) {
            Ok(size as u16)
        } else {
            Err(LutBuildError::InvalidEditedImage)
        }?;

        let mut data = Vec::new();
        let channel_capacity = 2_u16.pow(edited_image.bit_depth) as f32;

        for color in edited_image.data.chunks(edited_image.num_channels) {
            let r = color[0] as f32;
            let g = color[1] as f32;
            let b = color[2] as f32;

            data.push(r / channel_capacity);
            data.push(g / channel_capacity);
            data.push(b / channel_capacity);
        }

        Ok(Lut {
            lut_type: LutType::Lut3d,
            title,
            size,
            input_range: (0.0, 1.0),
            data,
        })
    }

    /// Save a LUT as a CUBE file.
    ///
    /// Arguments:
    /// * `path` - Where to save the LUT as a file, relative or absolute.
    /// * `precision` - The number of decimal places to use when writing RGB
    ///   values. In the Python interface, default is 6.
    /// * `write_title` - Whether or not to write the title in the file, if the
    ///   LUT has one. In the Python interface, default is `True`.
    /// * `write_domain_data` - Whether or not to write the domain data in the
    ///   file. In the Python interface, default is `False`.
    pub fn save(
        &self,
        path: &str,
        format: LutFormat,
        precision: usize,
        write_title: bool,
        write_domain_data: bool,
    ) -> Result<(), LutSaveError> {
        match format {
            LutFormat::CubeLut => {
                CubeLutFormat::save(&self, path, precision, write_title, write_domain_data)
            }
        }
    }
}
