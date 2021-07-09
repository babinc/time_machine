use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WorldTimeModel {
    pub abbreviation: Option<String>,
    pub client_ip: Option<String>,
    pub datetime: Option<String>,
    pub day_of_week: Option<i32>,
    pub day_of_year: Option<i32>,
    pub dst: Option<bool>,
    pub dst_from: Option<String>,
    pub dst_offset: Option<i32>,
    pub dst_until: Option<String>,
    pub raw_offset: Option<i32>,
    pub timezone: Option<String>,
    pub unixtime: Option<i64>,
    pub utc_datetime: Option<String>,
    pub utc_offset: Option<String>,
    pub week_number: Option<i32>
}
