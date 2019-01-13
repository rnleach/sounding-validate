use crate::error::*;
use metfor::{Celsius, HectoPascal, Meters, WindSpdDir};
use sounding_base::Sounding;

use optional::{some, Optioned};

macro_rules! validate_f64_positive {
    ($var:expr, $var_name:expr, $err_list:ident) => {
        if let Some(val) = $var.into_option() {
            let val: f64 = metfor::Quantity::unpack(val);
            if val < 0.0 {
                $err_list.push_error(Err(ValidationError::InvalidNegativeValue($var_name, val)));
            }
        }
    };
}

macro_rules! validate_wind_direction {
    ($var:expr, $err_list:ident) => {
        if let Some(val) = $var.into() {
            if val < 0.0 || val > 360.0 {
                $err_list.push_error(Err(ValidationError::InvalidWindDirection(val)));
            }
        }
    };
}

/// Validates the sounding with some simple sanity checks. For instance, checks that pressure
/// decreases with height.
pub fn validate(snd: &Sounding) -> Result<(), ValidationErrors> {
    let mut err_return = ValidationErrors::new();

    let pressure = snd.pressure_profile();

    //
    // Sounding checks
    //

    // Pressure required as vertical coordinate.
    err_return.push_error(check_pressure_exists(pressure));

    let len = pressure.len();
    let temperature = snd.temperature_profile();
    let wet_bulb = snd.wet_bulb_profile();
    let dew_point = snd.dew_point_profile();
    let theta_e = snd.theta_e_profile();
    let wind = snd.wind_profile();
    let omega = snd.pvv_profile();
    let height = snd.height_profile();
    let cloud_fraction = snd.cloud_fraction_profile();

    err_return.push_error(validate_vector_len(temperature, len, "Temperature"));
    err_return.push_error(validate_vector_len(wet_bulb, len, "Wet bulb temperature"));
    err_return.push_error(validate_vector_len(dew_point, len, "Dew point"));
    err_return.push_error(validate_vector_len(theta_e, len, "Theta-e"));
    err_return.push_error(validate_vector_len(wind, len, "Wind"));
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
    check_temp_wet_bulb_dew_point(snd, &mut err_return);

    // Check that speed >= 0 and direction 0-360
    for wind_val in wind {
        if let Some(WindSpdDir {
            speed: spd,
            direction: dir,
        }) = wind_val.into_option()
        {
            validate_f64_positive!(some(spd), "Wind speed", err_return);
            validate_wind_direction!(dir, err_return);
        }
    }

    // Check that cloud fraction >= 0
    for cld in cloud_fraction {
        validate_f64_positive!(*cld, "Cloud fraction", err_return);
    }

    // Surface checks
    // Check that hi, mid, and low cloud are all positive or zero
    validate_f64_positive!(snd.low_cloud(), "Low cloud", err_return);
    validate_f64_positive!(snd.mid_cloud(), "Mid cloud", err_return);
    validate_f64_positive!(snd.high_cloud(), "Hi cloud", err_return);

    if let Some(WindSpdDir {
        speed: spd,
        direction: dir,
    }) = snd.sfc_wind().into_option()
    {
        validate_f64_positive!(some(spd), "Wind speed", err_return);
        validate_wind_direction!(dir, err_return);
    }

    validate_f64_positive!(snd.mslp(), "MSLP", err_return);

    validate_f64_positive!(snd.station_pressure(), "Station pressure", err_return);

    err_return.check_any()
}

fn check_pressure_exists(pressure: &[Optioned<HectoPascal>]) -> Result<(), ValidationError> {
    if pressure.is_empty() {
        Err(ValidationError::NoPressureProfile)
    } else {
        Ok(())
    }
}

fn validate_vector_len<T>(
    vec: &[T],
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
    // Check that pressure always decreases with height and that the station pressure is more
    // than the lowest pressure level in sounding.
    let pressure = snd
        .pressure_profile()
        .into_iter()
        .filter_map(|val| val.into_option())
        .map(|HectoPascal(val)| val);
    let mut pressure_one_level_down = snd
        .station_pressure()
        .map_t(|HectoPascal(val)| val)
        .unwrap_or(::std::f64::MAX);
    for pres in pressure {
        if pressure_one_level_down < pres {
            return Err(ValidationError::PressureNotDecreasingWithHeight);
        }
        pressure_one_level_down = pres;
    }

    // Check height always increases with height.
    let height = snd
        .height_profile()
        .into_iter()
        .filter_map(|val| val.into_option())
        .map(|Meters(val)| val);
    let mut height_one_level_down = snd
        .station_info()
        .elevation()
        .map(|Meters(val)| val)
        .unwrap_or(::std::f64::MIN);
    for hght in height {
        if height_one_level_down > hght {
            return Err(ValidationError::PressureNotDecreasingWithHeight);
        }
        height_one_level_down = hght;
    }

    Ok(())
}

fn check_temp_wet_bulb_dew_point(snd: &Sounding, ve: &mut ValidationErrors) {
    let temperature = snd.temperature_profile();
    let wet_bulb = snd.wet_bulb_profile();
    let dew_point = snd.dew_point_profile();

    // Check that dew point <= wet bulb <= t
    for (t, wb) in temperature.iter().zip(wet_bulb.iter()) {
        if let (Some(Celsius(t)), Some(Celsius(wb))) = (t.into_option(), wb.into_option()) {
            if t < wb {
                ve.push_error(Err(ValidationError::TemperatureLessThanWetBulb(t, wb)));
            }
        }
    }
    for (t, dp) in temperature.iter().zip(dew_point.iter()) {
        if let (Some(Celsius(t)), Some(Celsius(dp))) = (t.into_option(), dp.into_option()) {
            if t < dp {
                ve.push_error(Err(ValidationError::TemperatureLessThanDewPoint(t, dp)));
            }
        }
    }
    for (wb, dp) in wet_bulb.iter().zip(dew_point.iter()) {
        if let (Some(Celsius(wb)), Some(Celsius(dp))) = (wb.into_option(), dp.into_option()) {
            if wb < dp {
                ve.push_error(Err(ValidationError::WetBulbLessThanDewPoint(wb, dp)));
            }
        }
    }
}
