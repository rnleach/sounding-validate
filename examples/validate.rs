extern crate sounding_base;
extern crate sounding_validate;

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
                Option::from(840.0),
                Option::from(800.0),
                Option::from(700.0),
                Option::from(500.0),
                Option::from(300.0),
                Option::from(250.0),
                Option::from(200.0),
                Option::from(100.0),
            ],
        )
        .set_profile(
            Profile::Temperature,
            vec![
                Option::from(20.0),
                Option::from(15.0),
                Option::from(2.0),
                Option::from(-10.0),
                Option::from(-20.0),
                Option::from(-30.0),
                Option::from(-50.0),
                Option::from(-45.0),
            ],
        )
        .set_profile(
            Profile::WetBulb,
            vec![
                Option::from(20.0),
                Option::from(14.0),
                Option::from(1.0),
                Option::from(-11.0),
                Option::from(-25.0),
                Option::from(-39.0),
                Option::from(-58.0),
                Option::from(-60.0),
            ],
        )
        .set_profile(
            Profile::DewPoint,
            vec![
                Option::from(20.0),
                Option::from(13.0),
                Option::from(0.0),
                Option::from(-12.0),
                Option::from(-27.0),
                Option::from(-45.0),
                Option::from(-62.0),
                Option::from(-80.0),
            ],
        )
        .set_profile(
            Profile::WindDirection,
            vec![
                Option::from(0.0),
                Option::from(40.0),
                Option::from(80.0),
                Option::from(120.0),
                Option::from(160.0),
                Option::from(200.0),
                Option::from(240.0),
                Option::from(280.0),
            ],
        )
        .set_profile(
            Profile::WindSpeed,
            vec![
                Option::from(5.0),
                Option::from(10.0),
                Option::from(15.0),
                Option::from(12.0),
                Option::from(27.0),
                Option::from(45.0),
                Option::from(62.0),
                Option::from(80.0),
            ],
        )
        .set_profile(
            Profile::GeopotentialHeight,
            vec![
                Option::from(1050.0),
                Option::from(2000.0),
                Option::from(3000.0),
                Option::from(4000.0),
                Option::from(5000.0),
                Option::from(6500.0),
                Option::from(7000.0),
                Option::from(8000.0),
            ],
        )
        .set_profile(
            Profile::CloudFraction,
            vec![
                Option::from(100.0),
                Option::from(85.0),
                Option::from(70.0),
                Option::from(50.0),
                Option::from(30.0),
                Option::from(25.0),
                Option::from(20.0),
                Option::from(10.0),
            ],
        )
        .set_surface_value(Surface::MSLP, 1014.0)
        .set_surface_value(Surface::StationPressure, 847.0)
        .set_surface_value(Surface::WindSpeed, 0.0)
        .set_surface_value(Surface::WindDirection, 0.0)
}

fn create_invalid_test_sounding() -> Sounding {
    create_valid_test_sounding().set_surface_value(sounding_base::Surface::StationPressure, 830.0)
}
