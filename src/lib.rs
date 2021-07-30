//! mzdata provides basic access to raw and processed mass spectrometry data formats in
//! Rust.
//!
//! The library currently supports reading:
//!   1. MGF files using [`MGFReader`] in [`crate::io::mgf`]
//!   2. mzML files using [`MzMLReader`] in [`crate::io::mzml`]
//!
//! It also includes a set of representation layers for spectra in [`crate::spectrum`]
//!
//! # Example
//! ```
//! use std::fs;
//! use mzdata::io::prelude::*;
//! use mzdata::io::mzml::MzMLReader;
//!
//! let mut ms1_count = 0;
//! let mut msn_count = 0;
//! let mut reader = MzMLReader::open_path("./test/data/small.mzML").unwrap();
//! for scan in reader {
//!     if scan.ms_level() == 1 {
//!         ms1_count += 1;
//!     } else {
//!         msn_count += 1;
//!     }
//! }
//! println!("MS1 Count: {}\nMSn Count: {}", ms1_count, msn_count);
//! assert_eq!(ms1_count, 14);
//! assert_eq!(msn_count, 34);
//! ```
//!
//! It also provides a sorted data structure for representing peak lists, [`PeakSet`]
//! and a trait implementing the majority of the logic, [`PeakCollection`].
pub mod io;
pub mod mass_error;
pub mod meta;
#[macro_use]
pub mod params;
pub mod peaks;
pub mod spectrum;
mod utils;

pub use crate::peaks::coordinate::{CoordinateDimension, CoordinateLike, Mass, MZ};

pub use crate::mass_error::MassErrorType;
pub use crate::peaks::peak::{CentroidPeak, DeconvolutedPeak};
pub use crate::peaks::{PeakCollection, PeakSet};

pub use crate::io::mgf::MGFReader;
pub use crate::io::mzml::MzMLReader;

pub use crate::params::{Param, ParamList, ParamDescribed};

pub use crate::spectrum::{CentroidSpectrum, RawSpectrum, SpectrumBehavior};
