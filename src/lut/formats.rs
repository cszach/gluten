use super::{Lut, LutSaveError, LutType};
use std::fs::File;
use std::io::prelude::*;

use pyo3::prelude::*;

/// Contains supported LUT file formats.
#[pyclass]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum LutFormat {
    /// Cube LUT (1D & 3D).
    CubeLut,
}

impl Default for LutFormat {
    fn default() -> Self {
        LutFormat::CubeLut
    }
}

/// LUTs that can be saved as a file format.
pub trait LutSave {
    fn save(
        lut: &Lut,
        path: &str,
        precision: usize,
        write_title: bool,
        write_domain_data: bool,
    ) -> Result<(), LutSaveError>;
}

/// The Cube LUT file format.
#[pyclass]
pub struct CubeLutFormat;

impl LutSave for CubeLutFormat {
    fn save(
        lut: &Lut,
        path: &str,
        precision: usize,
        write_title: bool,
        write_domain_data: bool,
    ) -> Result<(), LutSaveError> {
        let mut buffer = File::create(path)?;

        if write_title {
            if let Some(title) = &lut.title {
                buffer.write(format!(r#"TITLE "{}"{}"#, title, "\n").as_bytes())?;
            }
        }

        let lut_size_prop_name = match lut.lut_type {
            LutType::Lut1d => "LUT_1D_SIZE",
            LutType::Lut3d => "LUT_3D_SIZE",
        };

        buffer.write(format!("{} {}\n", lut_size_prop_name, lut.size).as_bytes())?;

        if write_domain_data {
            buffer.write(b"DOMAIN_MIN 0.0 0.0 0.0\nDOMAIN_MAX 1.0 1.0 1.0\n")?;
        }

        for rgb in lut.data.chunks(3) {
            let rgb = format!(
                "{:.p$} {:.p$} {:.p$}\n",
                rgb[0],
                rgb[1],
                rgb[2],
                p = precision
            );

            buffer.write(rgb.as_bytes())?;
        }

        Ok(())
    }
}
