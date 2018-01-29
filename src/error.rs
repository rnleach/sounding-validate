//! Errors for the soudning-base crate.
use std::fmt;

use failure::Fail;

/// Validation errors.
#[derive(Debug, Fail)]
pub enum ValidationError {
    /// Sounding did not have a profile for pressure.
    #[fail(display="No profile for pressure.")]
    NoPressureProfile,
    /// One of the profiles had a vector length that did not match the length of the pressure 
    /// profile vector. This is required because pressure is the vertical coordinate. The string is
    /// the name of the profile, the first `usize` is the length of that profile, and the second
    /// `usize` is the length it should have been.
    #[fail(display="Vector length for profile of {} ({}) does not match length of pressure profile: {}.", _0, _1, _2)]
    InvalidVectorLength(&'static str, usize, usize),
    /// Pressure not decreasing with height. This also checks that geopotential increases 
    /// "with height". It assumes the vectors are sorted with values from the lowest level to the
    /// highest level above ground.
    #[fail(display="Pressure not decreasing with height.")]
    PressureNotDecreasingWithHeight,
    /// Checks the required relationship between temperature and wet bulb.
    #[fail(display="Dry bulb temperature less than wet bulb temperature. ({} < {})", _0, _1)]
    TemperatureLessThanWetBulb(f64,f64),
    /// Checks the required relationship between temperature and dew point.
    #[fail(display="Dry bulb temperature less than dew point temperature. ({} < {})", _0, _1)]
    TemperatureLessThanDewPoint(f64,f64),
    /// Checks the required relationship between wet bulb and dew point.
    #[fail(display="Wet bulb temperature less than dew point temperature. ({} < {})", _0, _1)]
    WetBulbLessThanDewPoint(f64,f64),
    /// Invalid negative value, such as speed which must be positive.
    #[fail(display="{} less than 0.0 ({})", _0, _1)]
    InvalidNegativeValue(&'static str, f64),
    /// Invalid positive value, such as CIN which is always negative.
    #[fail(display="{} greater than 0.0 ({})", _0, _1)]
    InvalidPositiveValue(&'static str, f64),
}

/// Collection of validation errors.
#[derive(Debug)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Create a new collection of errors.
    pub fn new() -> Self {
        ValidationErrors{errors: vec![]}
    }

    /// Get the interior list of errors.
    pub fn as_vec(self) -> Vec<ValidationError>
    {
        self.errors
    }

    /// Add an error to this list
    pub fn push_error(&mut self, result: Result<(), ValidationError>) {
        match result {
            Ok(()) => {},
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
        writeln!(f,"\nValidation Errors")?;
        for error in &self.errors {
            writeln!(f, "     {}", error)?;
        }

        writeln!(f,"")
    }
}

impl Fail for ValidationErrors {

    fn cause(&self) -> Option<&Fail> {
        if self.errors.is_empty() {
            None
        } else {
            Some(&self.errors[0])
        }
    }

}
