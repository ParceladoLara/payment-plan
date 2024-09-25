use std::ops::Deref;
use wasm_bindgen::{describe::WasmDescribe, JsError};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Date(js_sys::Date);

impl Deref for Date {
    type Target = js_sys::Date;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<js_sys::Date> for Date {
    fn into(self) -> js_sys::Date {
        self.0
    }
}

impl From<js_sys::Date> for Date {
    fn from(date: js_sys::Date) -> Self {
        Self(date)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date_str = self.0.to_iso_string().as_string().unwrap();
        // This causes the JsObject to be serialized as a string, i can't find a way to serialize it as a Date object
        date_str.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_str = String::deserialize(deserializer)?;
        let date = js_sys::Date::new(&date_str.into());

        Ok(Date(date))
    }
}

impl TryInto<chrono::NaiveDate> for Date {
    type Error = JsError;

    fn try_into(self) -> Result<chrono::NaiveDate, Self::Error> {
        let date_str = self
            .0
            .to_iso_string()
            .as_string()
            .ok_or_else(|| JsError::new("Invalid date"))?;
        let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.3fZ")
            .map_err(|_| JsError::new("Invalid date"))?;

        Ok(date)
    }
}

impl From<chrono::NaiveDate> for Date {
    fn from(date: chrono::NaiveDate) -> Self {
        let date_str = date.to_string();
        let date = js_sys::Date::new(&date_str.into());

        Self(date)
    }
}

impl WasmDescribe for Date {
    fn describe() {
        js_sys::Date::describe();
    }
}
