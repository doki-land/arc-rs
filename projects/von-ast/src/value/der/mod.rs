use std::fmt::Formatter;

use num::FromPrimitive;
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use super::*;

impl<'de> Deserialize<'de> for VonNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Text::default())
    }
}

impl<'de> Visitor<'de> for Text {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except a `String` or `Text` struct")
    }

    fn visit_str<E>(mut self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value = v.to_string();
        Ok(self)
    }

    fn visit_string<E>(mut self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value = v;
        Ok(self)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Number::default())
    }
}

impl<'de> Visitor<'de> for Number {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except a number `String` or `Number` struct")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match BigDecimal::from_f64(v) {
            Some(s) => Ok(Number { hint: "".to_string(), value: s }),
            None => {
                todo!()
            }
        }
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        unsafe { Ok(Number::from_u128(v).unwrap_unchecked()) }
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        unsafe { Ok(Number::from_i128(v).unwrap_unchecked()) }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        todo!()
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
