extern crate optional;
extern crate sounding_base;
extern crate sounding_validate;

use optional::Optioned;
use sounding_base::{Sounding, StationInfo};
use sounding_validate::{validate, ValidationError};

#[test]
fn test_validate() {
    let snd = create_valid_test_sounding();
    let result = validate(&snd);
    if result.is_err() {
        println!("{:?}", result);
    }
    assert!(result.is_ok());
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
        )
        .set_profile(
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
        )
        .set_profile(
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
        )
        .set_profile(
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
        )
        .set_profile(
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
        )
        .set_profile(
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
        )
        .set_profile(
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
        )
        .set_profile(
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
        )
        .set_surface_value(Surface::MSLP, 1014.0)
        .set_surface_value(Surface::StationPressure, 847.0)
        .set_surface_value(Surface::WindSpeed, 0.0)
        .set_surface_value(Surface::WindDirection, 0.0)
}

#[test]
fn test_pressure_not_decreasing_with_height() {
    let snd = create_invalid_test_sounding_pressure_not_decreasing_with_height();
    let result = validate(&snd);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .into_inner()
            .contains(&ValidationError::PressureNotDecreasingWithHeight)
    );
}

fn create_invalid_test_sounding_pressure_not_decreasing_with_height() -> Sounding {
    create_valid_test_sounding().set_surface_value(sounding_base::Surface::StationPressure, 830.0)
}

#[test]
fn test_no_pressure_profile() {
    let snd = create_invalid_test_sounding_no_pressure_profile();
    let result = validate(&snd);

    let err = result.unwrap_err();
    println!("{}", err);

    assert!(err.into_inner().contains(&ValidationError::NoPressureProfile));
}

fn create_invalid_test_sounding_no_pressure_profile() -> Sounding {
    create_valid_test_sounding().set_profile(sounding_base::Profile::Pressure, vec![])
}

#[test]
fn test_no_invalid_vector_length() {
    let snd = create_invalid_test_sounding_vector_legth();
    let result = validate(&snd);
    let err = result.unwrap_err();
    println!("{}", err);

    if let ValidationError::InvalidVectorLength(desc, actual_length, desired_length) =
        err.into_inner()[0]
    {
        assert!(desc == "Temperature");
        assert!(actual_length == 8 && desired_length == 9);
    } else {
        panic!("Error is of wrong type!");
    }
}

fn create_invalid_test_sounding_vector_legth() -> Sounding {
    let t_profile = vec![
        Optioned::from(20.0),
        Optioned::from(15.0),
        Optioned::from(2.0),
        Optioned::from(-10.0),
        Optioned::from(-20.0),
        Optioned::from(-30.0),
        Optioned::from(-50.0),
    ];

    create_valid_test_sounding().set_profile(sounding_base::Profile::Temperature, t_profile)
}

#[test]
fn test_temperature_less_than_wetbulb() {
    let snd = create_invalid_test_temperature_less_than_wetbulb();
    let result = validate(&snd);
    let err = result.unwrap_err();
    println!("{}", err);

    let errs = err.into_inner();
    assert!(errs.len() == 3);

    for err in errs {
        if let ValidationError::TemperatureLessThanWetBulb(t, wb) = err {
            assert!(t < wb);
        } else {
            panic!("Error is of wrong type!");
        }
    }
}

fn create_invalid_test_temperature_less_than_wetbulb() -> Sounding {
    let t_profile = vec![
        Optioned::from(20.0),
        Optioned::from(13.0),
        Optioned::from(0.0),
        Optioned::from(-12.0),
        Optioned::from(-20.0),
        Optioned::from(-30.0),
        Optioned::from(-50.0),
        Optioned::from(-45.0),
    ];

    create_valid_test_sounding().set_profile(sounding_base::Profile::Temperature, t_profile)
}

#[test]
fn test_temperature_less_than_dewpoint() {
    let snd = create_invalid_test_temperature_less_than_dewpoint();
    let result = validate(&snd);
    let err = result.unwrap_err();
    println!("{}", err);

    let errs = err.into_inner();
    assert!(errs.len() == 6);

    for err in errs {
        if let ValidationError::TemperatureLessThanDewPoint(t, dp) = err {
            assert!(t < dp);
        } else if let ValidationError::TemperatureLessThanWetBulb(t, wb) = err {
            assert!(t < wb);
        } else {
            panic!("Error is of wrong type!");
        }
    }
}

fn create_invalid_test_temperature_less_than_dewpoint() -> Sounding {
    let t_profile = vec![
        Optioned::from(20.0),
        Optioned::from(10.0),
        Optioned::from(-1.0),
        Optioned::from(-13.0),
        Optioned::from(-20.0),
        Optioned::from(-30.0),
        Optioned::from(-50.0),
        Optioned::from(-45.0),
    ];

    create_valid_test_sounding().set_profile(sounding_base::Profile::Temperature, t_profile)
}

#[test]
fn test_wetbulb_less_than_dewpoint() {
    let snd = create_invalid_test_wetbulb_less_than_dewpoint();
    let result = validate(&snd);
    let err = result.unwrap_err();
    println!("{}", err);

    let errs = err.into_inner();
    assert!(errs.len() == 3);

    for err in errs {
        if let ValidationError::WetBulbLessThanDewPoint(wb, dp) = err {
            assert!(wb < dp);
        } else {
            panic!("Error is of wrong type!");
        }
    }
}

fn create_invalid_test_wetbulb_less_than_dewpoint() -> Sounding {
    let wb_profile = vec![
        Optioned::from(20.0),
        Optioned::from(12.0),
        Optioned::from(-1.0),
        Optioned::from(-13.0),
        Optioned::from(-25.0),
        Optioned::from(-39.0),
        Optioned::from(-58.0),
        Optioned::from(-60.0),
    ];

    create_valid_test_sounding().set_profile(sounding_base::Profile::WetBulb, wb_profile)
}

#[test]
fn test_invalid_negative_value() {
    let snd = create_invalid_test_invalid_negative_value();
    let result = validate(&snd);
    let err = result.unwrap_err();
    println!("{}", err);

    let errs = err.into_inner();
    assert!(errs.len() == 9);

    for err in errs {
        if let ValidationError::InvalidNegativeValue(_, val) = err {
            assert!(val < 0.0);
        } else {
            assert!(err == ValidationError::PressureNotDecreasingWithHeight);
        }
    }
}

fn create_invalid_test_invalid_negative_value() -> Sounding {
    let wind_speed = vec![
        Optioned::from(-5.0),
        Optioned::from(-10.0),
        Optioned::from(15.0),
        Optioned::from(12.0),
        Optioned::from(27.0),
        Optioned::from(45.0),
        Optioned::from(62.0),
        Optioned::from(80.0),
    ];
    let cc = vec![
        Optioned::from(100.0),
        Optioned::from(-85.0),
        Optioned::from(-70.0),
        Optioned::from(50.0),
        Optioned::from(30.0),
        Optioned::from(25.0),
        Optioned::from(20.0),
        Optioned::from(10.0),
    ];

    create_valid_test_sounding()
        .set_profile(sounding_base::Profile::WindSpeed, wind_speed)
        .set_profile(sounding_base::Profile::CloudFraction, cc)
        .set_surface_value(sounding_base::Surface::MSLP, -1014.0)
        .set_surface_value(sounding_base::Surface::StationPressure, -847.0)
        .set_surface_value(sounding_base::Surface::WindSpeed, -10.0)
}

#[test]
fn test_invalid_wind_direction() {
    let snd = create_invalid_test_wind_direction();
    let result = validate(&snd);
    let err = result.unwrap_err();
    println!("{}", err);

    let errs = err.into_inner();
    assert!(errs.len() == 6);

    for err in errs {
        if let ValidationError::InvalidWindDirection(val) = err {
            assert!(val < 0.0 || val > 360.0);
        } else {
            panic!("Error is of wrong type!");
        }
    }
}

fn create_invalid_test_wind_direction() -> Sounding {
    let wind_dir = vec![
        Optioned::from(0.0),
        Optioned::from(40.0),
        Optioned::from(-80.0),
        Optioned::from(-120.0),
        Optioned::from(460.0),
        Optioned::from(4200.0),
        Optioned::from(240.0),
        Optioned::from(280.0),
    ];

    create_valid_test_sounding()
        .set_profile(sounding_base::Profile::WindDirection, wind_dir)
        .set_surface_value(sounding_base::Surface::WindDirection, -90.0)
}
