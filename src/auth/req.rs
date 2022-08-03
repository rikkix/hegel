use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common;

#[cfg(feature = "chrono")]
use chrono::{Utc, TimeZone, DateTime};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestSimple {
    pub raw_path: String,
    pub cookies: Option<Vec<String>>,
    pub headers: HashMap<String, String>,
    pub query_string_parameters: Option<HashMap<String,String>>,
    pub request_context: common::RequestContextSimple,
    pub path_parameters: Option<HashMap<String, String>>,
    pub stage_variables: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub version: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub route_arn: String,
    pub identity_source: Vec<String>,
    pub route_key: String,
    pub raw_path: String,
    pub raw_query_string: String,
    pub cookies: Option<Vec<String>>,
    pub headers: HashMap<String, String>,
    pub query_string_parameters: Option<HashMap<String,String>>,
    pub request_context: common::RequestContext,
    pub path_parameters: Option<HashMap<String, String>>,
    pub stage_variables: Option<HashMap<String, String>>,
}


impl RequestSimple {
    pub fn path(&self) -> String {
        self.request_context.http.path.clone()
    }

    pub fn cookies(&self) -> Option<HashMap<String, String>> {
        let cs = self.cookies.clone();
        if cs.is_none() {
            return None
        }
        let cs = cs.unwrap();
        let mut result = HashMap::new();
        for c in cs {
            let spl = c.split("=").collect::<Vec<&str>>();
            if spl.len() != 2 {
                continue;
            }
            result.insert(spl[0].to_string(), spl[1].to_string());
        }
        Some(result)
    }

    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn queries(&self) -> Option<HashMap<String, String>> {
        self.query_string_parameters.clone()
    }

    pub fn params(&self) -> Option<HashMap<String, String>> {
        self.path_parameters.clone()
    }

    pub fn stage(&self) -> String {
        self.request_context.stage.clone()
    }

    pub fn time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.request_context.time_epoch)
    }

    #[cfg(feature = "chrono")]
    pub fn time_chrono(&self) -> DateTime<Utc> {
        Utc.timestamp_millis(self.request_context.time_epoch as i64)
    }

    pub fn method(&self) -> String {
        self.request_context.http.method.clone()
    }

    pub fn ip(&self) -> String {
        self.request_context.http.source_ip.clone()
    }

    pub fn ua(&self) -> String {
        self.request_context.http.user_agent.clone()
    }

    pub fn protocol(&self) -> String {
        self.request_context.http.protocol.clone()
    }
}


impl Request {
    pub fn path(&self) -> String {
        self.request_context.http.path.clone()
    }

    pub fn cookies(&self) -> Option<HashMap<String, String>> {
        let cs = self.cookies.clone();
        if cs.is_none() {
            return None
        }
        let cs = cs.unwrap();
        let mut result = HashMap::new();
        for c in cs {
            let spl = c.split("=").collect::<Vec<&str>>();
            if spl.len() != 2 {
                continue;
            }
            result.insert(spl[0].to_string(), spl[1].to_string());
        }
        Some(result)
    }

    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn queries(&self) -> Option<HashMap<String, String>> {
        self.query_string_parameters.clone()
    }

    pub fn params(&self) -> Option<HashMap<String, String>> {
        self.path_parameters.clone()
    }

    pub fn stage(&self) -> String {
        self.request_context.stage.clone()
    }

    pub fn time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.request_context.time_epoch)
    }

    #[cfg(feature = "chrono")]
    pub fn time_chrono(&self) -> DateTime<Utc> {
        Utc.timestamp_millis(self.request_context.time_epoch as i64)
    }

    pub fn method(&self) -> String {
        self.request_context.http.method.clone()
    }

    pub fn ip(&self) -> String {
        self.request_context.http.source_ip.clone()
    }

    pub fn ua(&self) -> String {
        self.request_context.http.user_agent.clone()
    }

    pub fn protocol(&self) -> String {
        self.request_context.http.protocol.clone()
    }
}