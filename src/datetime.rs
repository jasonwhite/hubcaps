use std::fmt;
use std::ops::Deref;

use chrono;
use serde::de::{self, Deserialize, Deserializer, Visitor};

/// A UTC datetime that can be deserialized as either a string or unix
/// timestamp. GitHub is inconsistent in how it handles dates and times. In some
/// cases, it returns an integer indicating the number of seconds since the Unix
/// epoch. In others, it returns a UTC datetime string.
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    pub fn into_inner(self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }
}

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateTimeVisitor;

        impl<'de> Visitor<'de> for DateTimeVisitor {
            type Value = DateTime;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "date time string or seconds since unix epoch")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DateTime(
                    v.parse().map_err(|e| E::custom(format!("{}", e)))?,
                ))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use chrono::offset::LocalResult;
                use chrono::TimeZone;

                match chrono::Utc.timestamp_opt(v, 0) {
                    LocalResult::None => Err(E::custom(format!(
                        "value is not a legal timestamp: {}",
                        v
                    ))),
                    LocalResult::Ambiguous(min, max) => {
                        Err(E::custom(format!(
                            "value is an ambiguous timestamp: \
                             {}, could be either of {}, {}",
                            v, min, max
                        )))
                    }
                    LocalResult::Single(datetime) => Ok(DateTime(datetime)),
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_i64(v as i64)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_any(DateTimeVisitor)
        } else {
            deserializer.deserialize_i64(DateTimeVisitor)
        }
    }
}

