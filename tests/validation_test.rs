extern crate sounding_base;
extern crate sounding_validate;

use sounding_base::Sounding;
use sounding_validate::validate;

#[test]
fn test_validate() {
    let snd = create_valid_test_sounding();
    let result = validate(&snd);
    if result.is_err() {
        println!("{:?}", result);
    }
    assert!(result.is_ok());

    let snd = create_invalid_test_sounding();
    assert!(validate(&snd).is_err());
}

fn create_valid_test_sounding() -> Sounding {
    use sounding_base::Profile::*;
    use sounding_base::Index::*;
    use sounding_base::Surface::*;

    Sounding::new()
        .set_station_num(1)
        .set_valid_time(None)
        .set_lead_time(0)
        .set_location(45.0, -115.0, 1023.0)
        .set_index(Showalter, -2.0)
        .set_index(LI, -2.0)
        .set_index(SWeT, 35.0)
        .set_index(K, 45.0)
        .set_index(LCL, 850.0)
        .set_index(PWAT, 2.0)
        .set_index(TotalTotals, 55.0)
        .set_index(CAPE, 852.0)
        .set_index(LCLTemperature, 290.0)
        .set_index(CIN, -200.0)
        .set_index(EquilibrimLevel, 222.0)
        .set_index(LFC, 800.0)
        .set_index(BulkRichardsonNumber, 1.2)
        .set_profile(
            Pressure,
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
            Temperature,
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
            WetBulb,
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
            DewPoint,
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
            WindDirection,
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
            WindSpeed,
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
            GeopotentialHeight,
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
            CloudFraction,
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
        .set_surface_value(MSLP, 1014.0)
        .set_surface_value(StationPressure, 847.0)
        .set_surface_value(UWind, 0.0)
        .set_surface_value(VWind, 0.0)
}

fn create_invalid_test_sounding() -> Sounding {
    create_valid_test_sounding().set_index(sounding_base::Index::PWAT, -5.1)
}
