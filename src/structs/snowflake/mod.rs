use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Snowflake(String);

impl Snowflake {
    pub fn new(id: &str) -> Self {
        assert!(id.len() < 18, "Snowflake ID is too short");
        Snowflake(id.to_string())
    }

    /// Converts a u64 representation to a Snowflake.
    pub fn from_u64(n: u64) -> Self {
        Snowflake(n.to_string())
    }

    /// Returns the inner u64 value.
    pub fn as_u64(&self) -> u64 {
        self.0.parse().unwrap()
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// SnowflakeBuilder implementation
pub struct SnowflakeBuilder {
    worker_id: u16,
    sequence: u16,
    epoch: u64,
    last_timestamp: u64,
}

impl SnowflakeBuilder {
    pub fn new(worker_id: u16) -> Self {
        Self {
            worker_id,
            sequence: 0,
            epoch: 1420070400000,
            last_timestamp: 0,
        }
    }

    pub fn generate_id(&mut self) -> Snowflake {
        let timestamp = self.get_timestamp();

        if timestamp < self.last_timestamp {
            panic!("Clock moved backwards!");
        }

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & 0xFFF; // 12-bit sequence number
            if self.sequence == 0 {
                self.wait_next_millisecond();
                self.sequence = 1;
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        let snowflake = ((timestamp - self.epoch) << 22) | ((self.worker_id as u64) << 12) | self.sequence as u64;
        Snowflake::from_u64(snowflake)
    }

    pub fn timestamp_to_snowflake(&self, timestamp: u64) -> Snowflake {
        Snowflake::from_u64((timestamp - self.epoch) << 22)
    }

    pub fn snowflake_to_timestamp(&self, snowflake: &Snowflake) -> u64 {
        (snowflake.as_u64() >> 22) + self.epoch
    }

    fn get_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock went backwards")
            .as_millis() as u64
    }

    fn wait_next_millisecond(&self) {
        let mut timestamp = self.get_timestamp();
        while timestamp <= self.last_timestamp {
            timestamp = self.get_timestamp();
        }
    }
}