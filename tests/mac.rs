use pwmp_msg::mac::{Mac, MacParseError};
use std::str::FromStr;

#[test]
fn valid_cases() {
    assert_eq!(
        Mac::from_str("01:02:03:04:05:06"),
        Ok(Mac::new(1, 2, 3, 4, 5, 6))
    );

    assert_eq!(
        Mac::from_str("01:AA:03:04:05:06"),
        Ok(Mac::new(1, 170, 3, 4, 5, 6))
    );

    assert_eq!(
        Mac::from_str("FF:FF:FF:FF:FF:FF"),
        Ok(Mac::new(255, 255, 255, 255, 255, 255))
    );

    assert_eq!(
        Mac::from_str("ff:ff:ff:ff:ff:ff"),
        Ok(Mac::new(255, 255, 255, 255, 255, 255))
    );

    assert_eq!(
        Mac::from_str(&Mac::default().to_string()),
        Ok(Mac::default())
    );
}

#[test]
fn invalid_cases() {
    assert_eq!(
        Mac::from_str("11:2:3:44:55:66:7"),
        Err(MacParseError) // requires double padded HEX octets
    );

    assert_eq!(Mac::from_str("+11:+22:+33:+44:5"), Err(MacParseError));

    assert_eq!(Mac::from_str("0::02:03:04:05:06"), Err(MacParseError));

    assert_eq!(Mac::from_str("+01:02:03:04:05:06"), Err(MacParseError));

    assert_eq!(Mac::from_str("01:02:03:04:05:06+"), Err(MacParseError));

    assert_eq!(Mac::from_str("01:02:03:x4:05:06"), Err(MacParseError));

    assert_eq!(Mac::from_str("01:+2:03:04:05:06"), Err(MacParseError));

    assert_eq!(Mac::from_str("XX:02:03:04:05:06"), Err(MacParseError));

    assert_eq!(Mac::from_str(""), Err(MacParseError));

    assert_eq!(Mac::from_str("01-02-03-04-05-06"), Err(MacParseError));

    assert_eq!(Mac::from_str("XX:02:03:04:05:06"), Err(MacParseError));
}
