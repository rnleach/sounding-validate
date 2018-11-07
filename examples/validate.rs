extern crate optional;
extern crate sounding_base;
extern crate sounding_validate;

use optional::Optioned;
use sounding_base::{Sounding, StationInfo};
use sounding_validate::validate;

fn main() {
    let snd = create_valid_test_sounding();
    let result = validate(&snd);
    if let Err(ref err) = result {
        println!("{}", err);
    } else {
        println!("Validated!");
    }

    let snd = create_invalid_test_sounding();
    let result = validate(&snd);
    if let Err(ref err) = result {
        println!("{}", err);
    } else {
        println!("Validated!");
    }
}

fn create_valid_test_sounding() -> Sounding {
    use sounding_base::Profile;
    use sounding_base::Surface;

    Sounding::new()
        .set_station_info(StationInfo::new_with_values(1, (45.0, -115.0), 1023.0))
        .set_valid_time(None)
        .set_lead_time(0)
        .set_profile(
            Profile::Pressure,
            vec![
                Optioned::from(840.0),
                Optioned::from(800.0),
                Optioned::from(700.0),
                Optioned::from(500.0),
                Optioned::from(300.0),
                Optioned::from(250.0),
                Optioned::from(200.0),
                Optioned::from(100.0),
            ],
        ).set_profile(
            Profile::Temperature,
            vec![
                Optioned::from(20.0),
                Optioned::from(15.0),
                Optioned::from(2.0),
                Optioned::from(-10.0),
                Optioned::from(-20.0),
                Optioned::from(-30.0),
                Optioned::from(-50.0),
                Optioned::from(-45.0),
            ],
        ).set_profile(
            Profile::WetBulb,
            vec![
                Optioned::from(20.0),
                Optioned::from(14.0),
                Optioned::from(1.0),
                Optioned::from(-11.0),
                Optioned::from(-25.0),
                Optioned::from(-39.0),
                Optioned::from(-58.0),
                Optioned::from(-60.0),
            ],
        ).set_profile(
            Profile::DewPoint,
            vec![
                Optioned::from(20.0),
                Optioned::from(13.0),
                Optioned::from(0.0),
                Optioned::from(-12.0),
                Optioned::from(-27.0),
                Optioned::from(-45.0),
                Optioned::from(-62.0),
                Optioned::from(-80.0),
            ],
        ).set_profile(
            Profile::WindDirection,
            vec![
                Optioned::from(0.0),
                Optioned::from(40.0),
                Optioned::from(80.0),
                Optioned::from(120.0),
                Optioned::from(160.0),
                Optioned::from(200.0),
                Optioned::from(240.0),
                Optioned::from(280.0),
            ],
        ).set_profile(
            Profile::WindSpeed,
            vec![
                Optioned::from(5.0),
                Optioned::from(10.0),
                Optioned::from(15.0),
                Optioned::from(12.0),
                Optioned::from(27.0),
                Optioned::from(45.0),
                Optioned::from(62.0),
                Optioned::from(80.0),
            ],
        ).set_profile(
            Profile::GeopotentialHeight,
            vec![
                Optioned::from(1050.0),
                Optioned::from(2000.0),
                Optioned::from(3000.0),
                Optioned::from(4000.0),
                Optioned::from(5000.0),
                Optioned::from(6500.0),
                Optioned::from(7000.0),
                Optioned::from(8000.0),
            ],
        ).set_profile(
            Profile::CloudFraction,
            vec![
                Optioned::from(100.0),
                Optioned::from(85.0),
                Optioned::from(70.0),
                Optioned::from(50.0),
                Optioned::from(30.0),
                Optioned::from(25.0),
                Optioned::from(20.0),
                Optioned::from(10.0),
            ],
        ).set_surface_value(Surface::MSLP, 1014.0)
        .set_surface_value(Surface::StationPressure, 847.0)
        .set_surface_value(Surface::WindSpeed, 0.0)
        .set_surface_value(Surface::WindDirection, 0.0)
}

fn create_invalid_test_sounding() -> Sounding {
    create_valid_test_sounding().set_surface_value(sounding_base::Surface::StationPressure, 830.0)
}
