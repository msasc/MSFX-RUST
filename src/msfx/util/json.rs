//! Provides the `JSON` struct and `Entry` enum for flexible JSON-like data storage and manipulation.
//!
//! The `JSON` struct can hold key-value pairs where values are of type `Entry`,
//! supporting various data types, including booleans, decimals, dates, times, and binary data.

use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{self, Value};
use std::collections::HashMap;

/// Represents a flexible data entry that can hold various data types.
///
/// `Entry` allows storage of booleans, decimals, dates, times, timestamps, strings,
/// binary data, and nested objects.#[derive(Debug, Clone, PartialEq)]
pub enum Entry {
    Boolean(bool),
    Decimal(Decimal),
    Date(NaiveDate),
    Time(NaiveTime),
    Timestamp(DateTime<Utc>),
    String(String),
    Object(HashMap<String, Entry>),
    Binary(Vec<u8>), // New variant to store binary data
}

impl<'de> Deserialize<'de> for Entry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let entry = match value {
            Value::Bool(b) => Entry::Boolean(b),
            Value::Number(num) => {
                if let Some(f) = num.as_f64() {
                    match Decimal::try_from(f) {
                        Ok(decimal) => Entry::Decimal(decimal),
                        Err(_) => return Err(serde::de::Error::custom("Invalid decimal type")),
                    }
                } else {
                    return Err(serde::de::Error::custom("Invalid number type"));
                }
            }
            Value::String(s) => {
                if let Ok(binary_data) = STANDARD.decode(&s) {
                    Entry::Binary(binary_data)
                } else if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                    Entry::Date(date)
                } else if let Ok(time) = NaiveTime::parse_from_str(&s, "%H:%M:%S") {
                    Entry::Time(time)
                } else if let Ok(timestamp) = DateTime::parse_from_rfc3339(&s) {
                    Entry::Timestamp(timestamp.with_timezone(&Utc))
                } else {
                    Entry::String(s)
                }
            }
            Value::Object(map) => {
                let object = map
                    .into_iter()
                    .map(|(k, v)| {
                        let entry = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
                        Ok((k, entry))
                    })
                    .collect::<Result<HashMap<_, _>, _>>()?;
                Entry::Object(object)
            }
            _ => return Err(serde::de::Error::custom("Unsupported JSON type")),
        };

        Ok(entry)
    }
}

impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Entry::Boolean(b) => serializer.serialize_bool(*b),
            Entry::Decimal(d) => serializer.serialize_str(&d.to_string()),
            Entry::Date(date) => serializer.serialize_str(&date.format("%Y-%m-%d").to_string()),
            Entry::Time(time) => serializer.serialize_str(&time.format("%H:%M:%S").to_string()),
            Entry::Timestamp(timestamp) => serializer.serialize_str(&timestamp.to_rfc3339()),
            Entry::String(s) => serializer.serialize_str(s),
            Entry::Binary(data) => {
                let encoded = STANDARD.encode(data);
                serializer.serialize_str(&encoded)
            }
            Entry::Object(map) => map.serialize(serializer),
        }
    }
}

/// Represents a JSON-like object that stores key-value pairs where values are of type `Entry`.
///
/// The `JSON` struct provides methods to serialize, deserialize, and manage data with
/// multiple possible types, as represented by the `Entry` enum.#[derive(Debug, Clone)]
pub struct JSON {
    entries: HashMap<String, Entry>,
}

impl JSON {
    pub fn new() -> Self {
        JSON {
            entries: HashMap::new(),
        }
    }

    /// Deserializes a JSON string into a `JSON` object.
    ///
    /// # Arguments
    ///
    /// * `json_str` - A JSON-formatted string.
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON string cannot be parsed.
    pub fn deserialize(json_str: &str) -> Result<Self, serde_json::Error> {
        let entries: HashMap<String, Entry> = serde_json::from_str(json_str)?;
        Ok(JSON { entries })
    }

    /// Serializes the `JSON` object into a JSON string.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.entries)
    }

    /// Retrieves a reference to an entry by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the entry to retrieve.
    ///
    /// # Returns
    ///
    /// Returns `Some(&Entry)` if the key exists, otherwise `None`.
    pub fn get(&self, key: &str) -> Option<&Entry> {
        self.entries.get(key)
    }

    /// Sets or updates an entry by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for the entry to set or update.
    /// * `entry` - The `Entry` value to associate with the key.
    pub fn set(&mut self, key: String, entry: Entry) {
        self.entries.insert(key, entry);
    }
    
    /// Removes an entry by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the entry to remove.
    ///
    /// # Returns
    ///
    /// Returns `Some(Entry)` if the entry existed, otherwise `None`.
    pub fn remove(&mut self, key: &str) -> Option<Entry> {
        self.entries.remove(key)
    }
}
