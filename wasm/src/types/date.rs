use std::ops::Deref;
use tsify_next::Tsify;
use wasm_bindgen::JsError;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Tsify)]
pub struct InnerDate(js_sys::Date);

impl Deref for InnerDate {
    type Target = js_sys::Date;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<js_sys::Date> for InnerDate {
    fn into(self) -> js_sys::Date {
        self.0
    }
}

impl From<js_sys::Date> for InnerDate {
    fn from(date: js_sys::Date) -> Self {
        Self(date)
    }
}

impl<'de> Deserialize<'de> for InnerDate {
    fn deserialize<D>(deserializer: D) -> Result<InnerDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_str = String::deserialize(deserializer)?;
        let date = js_sys::Date::new(&date_str.into());

        Ok(InnerDate(date))
    }
}

impl TryInto<chrono::NaiveDate> for InnerDate {
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

impl From<chrono::NaiveDate> for InnerDate {
    fn from(date: chrono::NaiveDate) -> Self {
        let date_str = date.to_string();
        let date = js_sys::Date::new(&date_str.into());

        Self(date)
    }
}
