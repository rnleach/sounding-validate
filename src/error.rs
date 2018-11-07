//! Errors for the sounding-validate crate.
use std::error::Error;
use std::fmt;

/// Validation errors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValidationError {
    /// Sounding did not have a profile for pressure.
    NoPressureProfile,
    /// One of the profiles had a vector length that did not match the length of the pressure
    /// profile vector. This is required because pressure is the vertical coordinate. The string is
    /// the name of the profile, the first `usize` is the length of that profile, and the second
    /// `usize` is the length it should have been.
    InvalidVectorLength(&'static str, usize, usize),
    /// Pressure not decreasing with height. This also checks that geopotential increases
    /// "with height". It assumes the vectors are sorted with values from the lowest level to the
    /// highest level above ground.
    PressureNotDecreasingWithHeight,
    /// Checks the required relationship between temperature and wet bulb.
    TemperatureLessThanWetBulb(f64, f64),
    /// Checks the required relationship between temperature and dew point.
    TemperatureLessThanDewPoint(f64, f64),
    /// Checks the required relationship between wet bulb and dew point.
    WetBulbLessThanDewPoint(f64, f64),
    /// Invalid negative value, such as speed which must be positive.
    InvalidNegativeValue(&'static str, f64),
    /// Invalid wind direction.
    InvalidWindDirection(f64),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValidationError::*;

        match self {
            NoPressureProfile => write!(f,"missing pressure profile"),
            InvalidVectorLength(_, _, _) => write!(f,"vectors do not match length"),
            PressureNotDecreasingWithHeight => write!(f,"pressure not decreasing with height"),
            TemperatureLessThanWetBulb(_, _) => write!(f,"temperature less than wet bulb"),
            TemperatureLessThanDewPoint(_, _) => write!(f,"temperature less than dew point"),
            WetBulbLessThanDewPoint(_, _) => write!(f,"wet bulb less than dew point"),
            InvalidNegativeValue(msg, val) => write!(f,"invalid negative value: {} : {}", msg, val),
            InvalidWindDirection(dir)=> write!(f,"invalid wind direction: {}", dir),
        }
    }
}

impl Error for ValidationError {}

/// Collection of validation errors.
#[derive(Debug, Default)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Create a new collection of errors.
    pub fn new() -> Self {
        ValidationErrors { errors: vec![] }
    }

    /// Get the interior list of errors.
    pub fn into_inner(self) -> Vec<ValidationError> {
        self.errors
    }

    /// Add an error to this list
    pub fn push_error(&mut self, result: Result<(), ValidationError>) {
        match result {
            Ok(()) => {}
            Err(err) => self.errors.push(err),
        }
    }

    /// Check if there are any errors, if not return `Ok`, otherwise return `self`.
    pub fn check_any(self) -> Result<(), ValidationErrors> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\nValidation Errors")?;
        for error in &self.errors {
            writeln!(f, "     {}", error)?;
        }

        writeln!(f)
    }
}

impl Error for ValidationErrors {}
