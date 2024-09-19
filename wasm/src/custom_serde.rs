use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Date(
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    pub js_sys::Date,
);

pub fn serialize_date<S>(date: &js_sys::Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_str = date.to_iso_string().as_string().unwrap();
    serializer.serialize_str(&date_str)
}

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<js_sys::Date, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    Ok(js_sys::Date::new(&date_str.into()))
}
