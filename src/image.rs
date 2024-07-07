use png::{BitDepth, ColorType, DecodingError};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::fmt;
use std::fs::File;

/// Bitmap image.
#[pyclass]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Image {
    /// The width of the image.
    #[pyo3(get)]
    pub width: u32,
    /// The height of the image.
    #[pyo3(get)]
    pub height: u32,
    /// The number of color channels in the image.
    #[pyo3(get)]
    pub num_channels: usize,
    /// The bit-depth of the image i.e. how many bits each color channel is.
    #[pyo3(get)]
    pub bit_depth: u32,
    /// The data specifying the color sequence in 8-bit integers e.g. if there
    /// are 4 color channels, the values in the data are R, G, B, A, R, G, B, A,
    /// and so on.
    #[pyo3(get)]
    pub data: Vec<u8>,
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

/// Contains possible errors during the image loading process.
#[pyclass]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum ImageLoadingError {
    /// An I/O error while trying to open the file e.g. file does not exist.
    IoError,
    /// An error while trying to decode the file as an image e.g. the format is
    /// corrupted.
    DecodingError,
}

impl fmt::Display for ImageLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageLoadingError::IoError => "IO error",
                ImageLoadingError::DecodingError => "Decoding error",
            }
        )
    }
}

impl Default for ImageLoadingError {
    fn default() -> Self {
        ImageLoadingError::IoError
    }
}

impl From<std::io::Error> for ImageLoadingError {
    fn from(_: std::io::Error) -> Self {
        ImageLoadingError::IoError
    }
}

impl From<DecodingError> for ImageLoadingError {
    fn from(_: DecodingError) -> Self {
        ImageLoadingError::DecodingError
    }
}

impl From<ImageLoadingError> for PyErr {
    fn from(rust_error: ImageLoadingError) -> Self {
        match rust_error {
            ImageLoadingError::IoError => PyTypeError::new_err("IO error while loading image"),
            ImageLoadingError::DecodingError => {
                PyTypeError::new_err("Decoding error while loading image")
            }
        }
    }
}

#[pymethods]
impl Image {
    /// Load a PNG image at the given path. The path can be relative or
    /// absolute.
    #[staticmethod]
    #[pyo3(signature = (path))]
    pub fn load_png(path: &str) -> Result<Image, ImageLoadingError> {
        let file = File::open(path)?;
        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info()?;

        let info = reader.info();
        let width = info.width;
        let height = info.height;
        let num_channels = match info.color_type {
            ColorType::Indexed => 0,
            ColorType::Grayscale => 1,
            ColorType::GrayscaleAlpha => 2,
            ColorType::Rgb => 3,
            ColorType::Rgba => 4,
        };
        let bit_depth = match info.bit_depth {
            BitDepth::One => 1,
            BitDepth::Two => 2,
            BitDepth::Four => 4,
            BitDepth::Eight => 8,
            BitDepth::Sixteen => 16,
        };

        let mut buffer = vec![0; reader.output_buffer_size()];
        let frame = reader.next_frame(&mut buffer)?;
        let bytes = &buffer[..frame.buffer_size()];

        Ok(Image {
            width,
            height,
            num_channels,
            bit_depth,
            data: bytes.to_vec(),
        })
    }
}
