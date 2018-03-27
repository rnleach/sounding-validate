#![warn(missing_docs)]
/*! Provides validation facilities for the 
[sounding-base](https://github.com/rnleach/sounding-base.git) crate.

See [examples](examples/validate.rs) for example of library use.

*/

//
// API
//
pub use error::{ValidationError, ValidationErrors};

//
// Internal use only
//

extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate sounding_base;
use sounding_base::Sounding;

mod error;

macro_rules! validate_f64_positive {
    ($var:expr, $var_name:expr, $err_list:ident) => {
        if let Some(val) = $var {
            if val < 0.0 {
                $err_list.push_error(Err(ValidationError::InvalidNegativeValue($var_name, val)));
            }
        }
    };
}

/// Validates the sounding with some simple sanity checks. For instance, checks that pressure
/// decreases with height.
pub fn validate(snd: &Sounding) -> Result<(), ValidationErrors> {
    use sounding_base::Profile;
    use sounding_base::Surface;

    let mut err_return = ValidationErrors::new();

    let pressure = snd.get_profile(Profile::Pressure);

    //
    // Sounding checks
    //

    // Pressure required as vertical coordinate.
    err_return.push_error(check_pressure_exists(pressure));

    let len = pressure.len();
    let temperature = snd.get_profile(Profile::Temperature);
    let wet_bulb = snd.get_profile(Profile::WetBulb);
    let dew_point = snd.get_profile(Profile::DewPoint);
    let theta_e = snd.get_profile(Profile::ThetaE);
    let direction = snd.get_profile(Profile::WindDirection);
    let speed = snd.get_profile(Profile::WindSpeed);
    let omega = snd.get_profile(Profile::PressureVerticalVelocity);
    let height = snd.get_profile(Profile::GeopotentialHeight);
    let cloud_fraction = snd.get_profile(Profile::CloudFraction);

    err_return.push_error(validate_vector_len(temperature, len, "Temperature"));
    err_return.push_error(validate_vector_len(wet_bulb, len, "Wet bulb temperature"));
    err_return.push_error(validate_vector_len(dew_point, len, "Dew point"));
    err_return.push_error(validate_vector_len(theta_e, len, "Theta-e"));
    err_return.push_error(validate_vector_len(direction, len, "Wind direction"));
    err_return.push_error(validate_vector_len(speed, len, "wind speed"));
    err_return.push_error(validate_vector_len(
        omega,
        len,
        "Omega (pressure vertical velocity)",
    ));
    err_return.push_error(validate_vector_len(height, len, "Height"));
    err_return.push_error(validate_vector_len(cloud_fraction, len, "Cloud fraction"));

    // Check that pressure always decreases with height and that the station pressure is more
    // than the lowest pressure level in sounding. AND..
    // Check height always increases with height.
    err_return.push_error(check_vertical_height_pressure(snd));

    // Check that dew point <= wet bulb <= t
    err_return.push_error(check_temp_wet_bulb_dew_point(snd));

    // Check that speed >= 0
    for spd in speed {
        validate_f64_positive!(*spd, "Wind speed", err_return);
    }

    // Check that cloud fraction >= 0
    for cld in cloud_fraction {
        validate_f64_positive!(*cld, "Cloud fraction", err_return);
    }

    // Surface checks
    // Check that hi, mid, and low cloud are all positive or zero
    validate_f64_positive!(
        snd.get_surface_value(Surface::LowCloud),
        "Low cloud",
        err_return
    );
    validate_f64_positive!(
        snd.get_surface_value(Surface::MidCloud),
        "Mid cloud",
        err_return
    );
    validate_f64_positive!(
        snd.get_surface_value(Surface::HighCloud),
        "Hi cloud",
        err_return
    );

    err_return.check_any()
}

fn check_pressure_exists(pressure: &[Option<f64>]) -> Result<(), ValidationError> {
    if pressure.is_empty() {
        Err(ValidationError::NoPressureProfile)
    } else {
        Ok(())
    }
}

fn validate_vector_len(
    vec: &[Option<f64>],
    len: usize,
    var_name: &'static str,
) -> Result<(), ValidationError> {
    if !vec.is_empty() && vec.len() != len {
        Err(ValidationError::InvalidVectorLength(
            var_name,
            vec.len(),
            len,
        ))
    } else {
        Ok(())
    }
}

fn check_vertical_height_pressure(snd: &Sounding) -> Result<(), ValidationError> {
    use sounding_base::Profile::{GeopotentialHeight, Pressure};
    use sounding_base::Surface::StationPressure;

    // Check that pressure always decreases with height and that the station pressure is more
    // than the lowest pressure level in sounding.
    let pressure = snd.get_profile(Pressure);
    let mut pressure_one_level_down = snd.get_surface_value(StationPressure)
        .unwrap_or(::std::f64::MAX);
    for pres in pressure.iter().filter_map(|pres| *pres) {
        if pressure_one_level_down < pres {
            return Err(ValidationError::PressureNotDecreasingWithHeight);
        }
        pressure_one_level_down = pres;
    }

    // Check height always increases with height.
    let height = snd.get_profile(GeopotentialHeight);
    let mut height_one_level_down = snd.get_station_info()
        .elevation()
        .unwrap_or(::std::f64::MIN);
    for hght in height.iter().filter_map(|hght| *hght) {
        if height_one_level_down > hght {
            return Err(ValidationError::PressureNotDecreasingWithHeight);
        }
        height_one_level_down = hght;
    }

    Ok(())
}

fn check_temp_wet_bulb_dew_point(snd: &Sounding) -> Result<(), ValidationError> {
    use sounding_base::Profile::{DewPoint, Temperature, WetBulb};

    let temperature = snd.get_profile(Temperature);
    let wet_bulb = snd.get_profile(WetBulb);
    let dew_point = snd.get_profile(DewPoint);

    // Check that dew point <= wet bulb <= t
    for (t, wb) in temperature.iter().zip(wet_bulb.iter()) {
        if let (Some(t), Some(wb)) = (*t, *wb) {
            if t < wb {
                return Err(ValidationError::TemperatureLessThanWetBulb(t, wb));
            }
        }
    }
    for (t, dp) in temperature.iter().zip(dew_point.iter()) {
        if let (Some(t), Some(dp)) = (*t, *dp) {
            if t < dp {
                return Err(ValidationError::TemperatureLessThanDewPoint(t, dp));
            }
        }
    }
    for (wb, dp) in wet_bulb.iter().zip(dew_point.iter()) {
        if let (Some(wb), Some(dp)) = (*wb, *dp) {
            if wb < dp {
                return Err(ValidationError::WetBulbLessThanDewPoint(wb, dp));
            }
        }
    }

    Ok(())
}
