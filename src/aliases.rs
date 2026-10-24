//! Type aliases.

/// Temperature expressed in units of decicelsius (often called a decidegree Celsius).
///
/// _0.1°dC is equivalent to 0.1°C_
pub type Temperature = i16;

/// Relative humidity represented as percentage (*%*).
pub type Humidity = u8;

/// Air pressure represented in hecto-Pascal (*hPa*).
pub type AirPressure = u16;

/// Voltage expressed in units of millivolts (_mV_).
///
/// _1mV is equivalent of 0.001V_
pub type BatteryVoltage = u16;

/// Signal strength represented in decibel-milliwatts (_dBm_).
pub type Rssi = i8;
