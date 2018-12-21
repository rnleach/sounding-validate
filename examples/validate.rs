use metfor::{Celsius, HectoPascal, Meters, WindSpdDir};
use optional::Optioned;
use sounding_base::{Sounding, StationInfo};
use sounding_validate::validate;

fn main() {
    let soundings = vec![create_valid_test_sounding(), create_invalid_test_sounding()];

    let results = soundings.iter().map(validate);

    for result in results {
        if let Err(ref err) = result {
            println!("{}", err);
        } else {
            println!("Validated!");
        }
    }
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
                speed_kt: 5.0,
            }),
            Optioned::from(WindSpdDir {
                direction: 40.0,
                speed_kt: 10.0,
            }),
            Optioned::from(WindSpdDir {
                direction: 80.0,
                speed_kt: 15.0,
            }),
            Optioned::from(WindSpdDir {
                direction: 120.0,
                speed_kt: 12.0,
            }),
            Optioned::from(WindSpdDir {
                direction: 160.0,
                speed_kt: 27.0,
            }),
            Optioned::from(WindSpdDir {
                direction: 200.0,
                speed_kt: 45.0,
            }),
            Optioned::from(WindSpdDir {
                direction: 240.0,
                speed_kt: 62.0,
            }),
            Optioned::from(WindSpdDir {
                direction: 280.0,
                speed_kt: 80.0,
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
            speed_kt: 0.0,
            direction: 0.0,
        })
}

fn create_invalid_test_sounding() -> Sounding {
    create_valid_test_sounding().with_station_pressure(HectoPascal(830.0))
}
