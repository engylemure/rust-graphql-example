use chrono::NaiveDate;
use crate::utils::env::ENV;
use async_graphql::SimpleObject;

#[SimpleObject]
pub struct ApiVersion {
    pub version: &'static str,
    pub stage: String,
    pub date: NaiveDate,
}

pub fn api_version() -> ApiVersion {
    let stage = ENV.rust_env.clone();
    let date = {
        NaiveDate::parse_from_str(ENV.api_version_date.as_str(), "%Y-%m-%d").unwrap()
    };
    ApiVersion {
        version: "0.1.0",
        stage,
        date,
    }
}
