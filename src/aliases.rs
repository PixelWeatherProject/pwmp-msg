use rust_decimal::Decimal;

/// A fixed-precision decimal number that represents temperature in *Celsius*.
pub type Temperature = Decimal;

/// Relative humidity represented as percentage (*%*).
pub type Humidity = u8;

/// Air pressure represented in hecto-Pascal (*hPa*).
pub type AirPressure = u16;

/// A fixed-precision decimal number that represents voltage (*V*).
pub type BatteryVoltage = Decimal;

/// Signal strength represented in decibel-milliwatts (*dBm*).
pub type Rssi = i8;
