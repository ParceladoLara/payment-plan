use chrono::Datelike;
use std::ops::Deref;
use wasm_bindgen::{JsError, JsValue};

use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct Date(js_sys::Date);

impl Deref for Date {
    type Target = js_sys::Date;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<JsValue> for Date {
    fn into(self) -> JsValue {
        self.0.into()
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

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_str = String::deserialize(deserializer)?;

        //Remove UTC and Z from the date string
        let date_str = date_str.replace("UTC", "").replace("Z", "");
        let date_str = date_str.trim();
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
        let year = date.year() as u32;
        let month = date.month() as i32 - 1;
        let day = date.day() as i32;

        let date =
            js_sys::Date::new_with_year_month_day_hr_min_sec_milli(year, month, day, 3, 0, 0, 0);

        Self(date)
    }
}
