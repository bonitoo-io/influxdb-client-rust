/*
 * Influx API Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthCheck {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "checks", skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<crate::generated::models::HealthCheck>>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}

impl HealthCheck {
    pub fn new(name: String, status: Status) -> HealthCheck {
        HealthCheck {
            name,
            message: None,
            checks: None,
            status,
            version: None,
            commit: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,
}