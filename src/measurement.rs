//! Module for the `MeasurementColection` and helper types.

use crate::aliases::{AirPressure, BatteryVoltage, Humidity, Temperature};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A complete measurement data snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MeasurementCollection {
    /// Temperature
    pub temperature: Temperature,

    /// Humidity
    pub humidity: Humidity,

    /// Air pressure *(if supported by the node)*
    pub air_pressure: Option<AirPressure>,

    /// Node's battery voltage
    pub battery: BatteryVoltage,

    /// CPU die temperature
    pub cpu_temp: Temperature,
}

/// A historic measurement data snapshot, which contains a [`MeasurementCollection`] and a timestamp (unix epoch).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HistoricMeasurement {
    /// Measurement data
    collection: MeasurementCollection,

    /// Timestamp as seconds since UNIX epoch
    timestamp: u64,
}

impl HistoricMeasurement {
    /// Creates a new historic measurement snapshot with the specified collection and timestamp.
    ///
    /// # Example
    /// ```rust
    /// use pwmp_msg::measurement::*;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let collection = MeasurementCollection::default();
    /// let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    ///
    /// let timestamped_coll = HistoricMeasurement::new(collection.clone(), timestamp);
    ///
    /// assert_eq!(timestamped_coll.collection(), &collection);
    /// assert_eq!(timestamped_coll.timestamp(), timestamp);
    /// ```
    #[must_use]
    pub const fn new(collection: MeasurementCollection, timestamp: u64) -> Self {
        Self {
            collection,
            timestamp,
        }
    }

    /// Create a new historic measurement snapshot with the specified collection
    /// and the current timestamp (calculated using the current system time).
    ///
    /// # Example
    /// ```rust
    /// use pwmp_msg::measurement::*;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let collection = MeasurementCollection::default();
    /// let timestamped_coll = HistoricMeasurement::new_now(collection.clone());
    ///
    /// assert_eq!(timestamped_coll.collection(), &collection);
    /// ```
    #[must_use]
    pub fn new_now(collection: MeasurementCollection) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self::new(collection, timestamp)
    }

    /// Returns a reference to the inner collection.
    #[must_use]
    pub const fn collection(&self) -> &MeasurementCollection {
        &self.collection
    }

    /// Returns the stored timestamp as seconds since UNIX epoch.
    #[must_use]
    pub const fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl MeasurementCollection {
    /// Creates an empty collection.
    #[must_use]
    pub const fn const_new() -> Self {
        Self {
            temperature: 0,
            humidity: 0,
            air_pressure: None,
            battery: 0,
            cpu_temp: 0,
        }
    }
}

impl Default for MeasurementCollection {
    fn default() -> Self {
        Self::const_new()
    }
}
