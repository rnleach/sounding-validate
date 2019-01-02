use metfor::{Celsius, HectoPascal, Meters, WindSpdDir, Knots};
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
    Sounding::new()
        .with_station_info(StationInfo::new_with_values(
            1,
            (45.0, -115.0),
            Meters(1023.0),
        ))
        .with_valid_time(None)
        .with_lead_time(0)
        .with_pressure_profile(vec![
            Optioned::from(HectoPascal(840.0)),
            Optioned::from(HectoPascal(800.0)),
            Optioned::from(HectoPascal(700.0)),
            Optioned::from(HectoPascal(500.0)),
            Optioned::from(HectoPascal(300.0)),
            Optioned::from(HectoPascal(250.0)),
            Optioned::from(HectoPascal(200.0)),
            Optioned::from(HectoPascal(100.0)),
        ])
        .with_temperature_profile(vec![
            Optioned::from(Celsius(20.0)),
            Optioned::from(Celsius(15.0)),
            Optioned::from(Celsius(2.0)),
            Optioned::from(Celsius(-10.0)),
            Optioned::from(Celsius(-20.0)),
            Optioned::from(Celsius(-30.0)),
            Optioned::from(Celsius(-50.0)),
            Optioned::from(Celsius(-45.0)),
        ])
        .with_wet_bulb_profile(vec![
            Optioned::from(Celsius(20.0)),
            Optioned::from(Celsius(14.0)),
            Optioned::from(Celsius(1.0)),
            Optioned::from(Celsius(-11.0)),
            Optioned::from(Celsius(-25.0)),
            Optioned::from(Celsius(-39.0)),
            Optioned::from(Celsius(-58.0)),
            Optioned::from(Celsius(-60.0)),
        ])
        .with_dew_point_profile(vec![
            Optioned::from(Celsius(20.0)),
            Optioned::from(Celsius(13.0)),
            Optioned::from(Celsius(0.0)),
            Optioned::from(Celsius(-12.0)),
            Optioned::from(Celsius(-27.0)),
            Optioned::from(Celsius(-45.0)),
            Optioned::from(Celsius(-62.0)),
            Optioned::from(Celsius(-80.0)),
        ])
        .with_wind_profile(vec![
            Optioned::from(WindSpdDir {
                direction: 0.0,
                speed: Knots(5.0),
            }),
            Optioned::from(WindSpdDir {
                direction: 40.0,
                speed: Knots(10.0),
            }),
            Optioned::from(WindSpdDir {
                direction: 80.0,
                speed: Knots(15.0),
            }),
            Optioned::from(WindSpdDir {
                direction: 120.0,
                speed: Knots(12.0),
            }),
            Optioned::from(WindSpdDir {
                direction: 160.0,
                speed: Knots(27.0),
            }),
            Optioned::from(WindSpdDir {
                direction: 200.0,
                speed: Knots(45.0),
            }),
            Optioned::from(WindSpdDir {
                direction: 240.0,
                speed: Knots(62.0),
            }),
            Optioned::from(WindSpdDir {
                direction: 280.0,
                speed: Knots(80.0),
            }),
        ])
        .with_height_profile(vec![
            Optioned::from(Meters(1050.0)),
            Optioned::from(Meters(2000.0)),
            Optioned::from(Meters(3000.0)),
            Optioned::from(Meters(4000.0)),
            Optioned::from(Meters(5000.0)),
            Optioned::from(Meters(6500.0)),
            Optioned::from(Meters(7000.0)),
            Optioned::from(Meters(8000.0)),
        ])
        .with_cloud_fraction_profile(vec![
            Optioned::from(100.0),
            Optioned::from(85.0),
            Optioned::from(70.0),
            Optioned::from(50.0),
            Optioned::from(30.0),
            Optioned::from(25.0),
            Optioned::from(20.0),
            Optioned::from(10.0),
        ])
        .with_mslp(HectoPascal(1014.0))
        .with_station_pressure(HectoPascal(847.0))
        .with_sfc_wind(WindSpdDir {
            speed: Knots(0.0),
            direction: 0.0,
        })
}

#[test]
fn test_pressure_not_decreasing_with_height() {
    let snd = create_invalid_test_sounding_pressure_not_decreasing_with_height();
    let result = validate(&snd);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .into_inner()
        .contains(&ValidationError::PressureNotDecreasingWithHeight));
}

fn create_invalid_test_sounding_pressure_not_decreasing_with_height() -> Sounding {
    create_valid_test_sounding().with_station_pressure(HectoPascal(830.0))
}

#[test]
fn test_no_pressure_profile() {
    let snd = create_invalid_test_sounding_no_pressure_profile();
    let result = validate(&snd);

    let err = result.unwrap_err();
    println!("{}", err);

    assert!(err
        .into_inner()
        .contains(&ValidationError::NoPressureProfile));
}

fn create_invalid_test_sounding_no_pressure_profile() -> Sounding {
    create_valid_test_sounding().with_pressure_profile(vec![])
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
        assert_eq!(desc, "Cloud fraction");
        assert_eq!(actual_length, 7);
        assert_eq!(desired_length, 9);
    } else {
        panic!("Error is of wrong type!");
    }
}

fn create_invalid_test_sounding_vector_legth() -> Sounding {
    let cloud_profile = vec![
        Optioned::from(20.0),
        Optioned::from(15.0),
        Optioned::from(2.0),
        Optioned::from(10.0),
        Optioned::from(20.0),
        Optioned::from(30.0),
    ];

    create_valid_test_sounding().with_cloud_fraction_profile(cloud_profile)
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
        Optioned::from(Celsius(20.0)),
        Optioned::from(Celsius(13.0)),
        Optioned::from(Celsius(0.0)),
        Optioned::from(Celsius(-12.0)),
        Optioned::from(Celsius(-20.0)),
        Optioned::from(Celsius(-30.0)),
        Optioned::from(Celsius(-50.0)),
        Optioned::from(Celsius(-45.0)),
    ];

    create_valid_test_sounding().with_temperature_profile(t_profile)
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
        Optioned::from(Celsius(20.0)),
        Optioned::from(Celsius(10.0)),
        Optioned::from(Celsius(-1.0)),
        Optioned::from(Celsius(-13.0)),
        Optioned::from(Celsius(-20.0)),
        Optioned::from(Celsius(-30.0)),
        Optioned::from(Celsius(-50.0)),
        Optioned::from(Celsius(-45.0)),
    ];

    create_valid_test_sounding().with_temperature_profile(t_profile)
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
        Optioned::from(Celsius(20.0)),
        Optioned::from(Celsius(12.0)),
        Optioned::from(Celsius(-1.0)),
        Optioned::from(Celsius(-13.0)),
        Optioned::from(Celsius(-25.0)),
        Optioned::from(Celsius(-39.0)),
        Optioned::from(Celsius(-58.0)),
        Optioned::from(Celsius(-60.0)),
    ];

    create_valid_test_sounding().with_wet_bulb_profile(wb_profile)
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
        Optioned::from(WindSpdDir {
            direction: 0.0,
            speed: Knots(-5.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 40.0,
            speed: Knots(-10.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 80.0,
            speed: Knots(15.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 120.0,
            speed: Knots(12.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 160.0,
            speed: Knots(27.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 200.0,
            speed: Knots(45.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 240.0,
            speed: Knots(62.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 280.0,
            speed: Knots(80.0),
        }),
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
        .with_wind_profile(wind_speed)
        .with_cloud_fraction_profile(cc)
        .with_mslp(HectoPascal(-1014.0))
        .with_station_pressure(HectoPascal(-847.0))
        .with_sfc_wind(WindSpdDir {
            speed: Knots(-10.0),
            direction: 100.0,
        })
}

#[test]
fn test_invalid_wind_direction() {
    let snd = create_invalid_test_wind_direction();
    let result = validate(&snd);
    let err = result.unwrap_err();
    println!("{}", err);

    let errs = err.into_inner();
    assert_eq!(errs.len(), 4);

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
        Optioned::from(WindSpdDir {
            direction: 0.0,
            speed: Knots(5.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 40.0,
            speed: Knots(10.0),
        }),
        Optioned::from(WindSpdDir {
            direction: -80.0,
            speed: Knots(15.0),
        }),
        Optioned::from(WindSpdDir {
            direction: -120.0,
            speed: Knots(12.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 160.0,
            speed: Knots(27.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 200.0,
            speed: Knots(45.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 240.0,
            speed: Knots(62.0),
        }),
        Optioned::from(WindSpdDir {
            direction: 280.0,
            speed: Knots(80.0),
        }),
    ];

    create_valid_test_sounding()
        .with_wind_profile(wind_dir)
        .with_sfc_wind(WindSpdDir {
            direction: -90.0,
            speed: Knots(5.0),
        })
}
