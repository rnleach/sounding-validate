#![warn(missing_docs)]
#![recursion_limit = "1024"]
//! Library to validate an atmospheric sounding.

#[macro_use]
extern crate error_chain;

extern crate sounding_base;
use sounding_base::{Sounding, OptionVal};

pub mod error;
pub use error::*;

macro_rules! validate_f64_positive {
    ($var:expr, $err_msg:ident, $var_name:expr) => {
        if let Some(val) = $var.as_option() {
            if val < 0.0 {
                $err_msg.push_str(&format!("\n{} < 0.0: {}", $var_name, val));
            }
        }
    };
}

/// Validates the sounding with some simple sanity checks. For instance, checks that pressure
/// decreases with height.
pub fn validate(snd: &Sounding) -> Result<()> {

    use sounding_base::Profile::*;
    use sounding_base::Surface::*;
    use sounding_base::Index::*;

    let mut error_msg = String::from("");

    let pressure = snd.get_profile(Pressure);

    // Sounding checks
    check_pressure_exists(pressure, &mut error_msg); // Pressure required as vertical coordinate

    let len = pressure.len();
    let temperature = snd.get_profile(Temperature);
    let wet_bulb = snd.get_profile(WetBulb);
    let dew_point = snd.get_profile(DewPoint);
    let theta_e = snd.get_profile(ThetaE);
    let direction = snd.get_profile(WindDirection);
    let speed = snd.get_profile(WindSpeed);
    let omega = snd.get_profile(PressureVerticalVelocity);
    let height = snd.get_profile(GeopotentialHeight);
    let cloud_fraction = snd.get_profile(CloudFraction);

    validate_vector_len(temperature, len, &mut error_msg, "Temperature");
    validate_vector_len(wet_bulb, len, &mut error_msg, "Wet bulb temperature");
    validate_vector_len(dew_point, len, &mut error_msg, "Dew point");
    validate_vector_len(theta_e, len, &mut error_msg, "Theta-e");
    validate_vector_len(direction, len, &mut error_msg, "Wind direction");
    validate_vector_len(speed, len, &mut error_msg, "wind speed");
    validate_vector_len(
        omega,
        len,
        &mut error_msg,
        "Omega (pressure vertical velocity)",
    );
    validate_vector_len(height, len, &mut error_msg, "Height");
    validate_vector_len(cloud_fraction, len, &mut error_msg, "Cloud fraction");

    // Check that pressure always decreases with height and that the station pressure is more
    // than the lowest pressure level in sounding. AND..
    // Check height always increases with height.
    check_vertical_height_pressure(snd, &mut error_msg);

    // Check that dew point <= wet bulb <= t
    check_temp_wet_bulb_dew_point(snd, &mut error_msg);

    // Check that speed >= 0
    for spd in speed.iter().filter_map(|spd| spd.as_option()) {
        if spd < 0.0 {
            error_msg.push_str(&format!("\nWind speed < 0: {} < 0.0", spd));
        }
    }

    // Check that cloud fraction >= 0
    for cld in cloud_fraction.iter().filter_map(|cld| cld.as_option()) {
        if cld < 0.0 {
            error_msg.push_str(&format!("\nCloud fraction < 0: {} < 0.0", cld));
        }

    }

    // Index checks
    validate_f64_positive!(snd.get_index(CAPE), error_msg, "CAPE");
    validate_f64_positive!(snd.get_index(PWAT), error_msg, "PWAT");

    // Check that cin <= 0
    if let Some(val) = snd.get_index(CIN).as_option() {
        if val > 0.0 {
            error_msg.push_str(&format!("\nCINS > 0.0: {}", val));
        }
    }

    // Surface checks
    // Check that hi, mid, and low cloud are all positive or zero
    validate_f64_positive!(snd.get_surface_value(LowCloud), error_msg, "low cloud");
    validate_f64_positive!(snd.get_surface_value(MidCloud), error_msg, "mid cloud");
    validate_f64_positive!(snd.get_surface_value(HighCloud), error_msg, "hi cloud");

    if error_msg == "" {
        Ok(())
    } else {
        error_msg.push('\n');
        Err(Error::from(ErrorKind::ValidationError(error_msg)))
    }
}

fn check_pressure_exists(pressure: &[OptionVal<f64>], error_msg: &mut String) {
    if pressure.is_empty() {
        error_msg.push_str("\nPressure variable required, none given.");
    }
}

fn validate_vector_len(vec: &[OptionVal<f64>], len: usize, error_msg: &mut String, var_name: &str) {
    if !vec.is_empty() && vec.len() != len {
        error_msg.push_str(&format!(
            "\n{} array has different length than pressure array.",
            var_name
        ));
    }
}

fn check_vertical_height_pressure(snd: &Sounding, error_msg: &mut String) {
    use sounding_base::Profile::{GeopotentialHeight, Pressure};
    use sounding_base::Surface::StationPressure;

    // Check that pressure always decreases with height and that the station pressure is more
    // than the lowest pressure level in sounding.
    let pressure = snd.get_profile(Pressure);
    let mut pressure_one_level_down = snd.get_surface_value(StationPressure)
        .as_option()
        .unwrap_or(::std::f64::MAX);
    for pres in pressure.iter().filter_map(|pres| pres.as_option()) {
        if pressure_one_level_down < pres {
            error_msg.push_str(&format!(
                "\nPressure increasing with height: {} < {}",
                pressure_one_level_down,
                pres
            ));
        }
        pressure_one_level_down = pres;
    }

    // Check height always increases with height.
    // FIXME: Is height AGL or ASL?
    let height = snd.get_profile(GeopotentialHeight);
    let mut height_one_level_down = snd.get_location().2.as_option().unwrap_or(::std::f64::MIN);
    for hght in height.iter().filter_map(|hght| hght.as_option()) {
        if height_one_level_down > hght {
            error_msg.push_str(&format!(
                "\nHeight values decreasing with height: {} > {}",
                height_one_level_down,
                hght
            ));
        }
        height_one_level_down = hght;
    }
}

fn check_temp_wet_bulb_dew_point(snd: &Sounding, error_msg: &mut String) {
    use sounding_base::Profile::{Temperature, WetBulb, DewPoint};

    let temperature = snd.get_profile(Temperature);
    let wet_bulb = snd.get_profile(WetBulb);
    let dew_point = snd.get_profile(DewPoint);

    // Check that dew point <= wet bulb <= t
    for (t, wb) in temperature.iter().zip(wet_bulb.iter()) {
        if let (Some(t), Some(wb)) = (t.as_option(), wb.as_option()) {
            if t < wb {
                error_msg.push_str(&format!(
                    "\nTemperature < Wet bulb: {} < {}",
                    t,
                    wb,
                ));
            }
        }
    }
    for (t, dp) in temperature.iter().zip(dew_point.iter()) {
        if let (Some(t), Some(dp)) = (t.as_option(), dp.as_option()) {
            if t < dp {
                error_msg.push_str(&format!(
                    "\nTemperature < Dew Point: {} < {}",
                    t,
                    dp,
                ));
            }
        }
    }
    for (wb, dp) in wet_bulb.iter().zip(dew_point.iter()) {
        if let (Some(wb), Some(dp)) = (wb.as_option(), dp.as_option()) {
            if wb < dp {
                error_msg.push_str(&format!(
                    "\nWet bulb < Dew Point: {} < {}",
                    wb,
                    dp,
                ));
            }
        }
    }
}
