use super::requestor::get_site;
use reqwest::Url;

use super::types;
use std::time::SystemTime;
use chrono::{DateTime, Utc};


pub async fn scan_site(original_url: &str) -> types::ScanResult {
    let url_host = Url::parse(original_url).unwrap();
    let (document_info, req_info, framework_inof): (types::DocumentInfo, types::ReqInfo, types::FrameworkInfo) = get_site(url_host.clone()).await;
    let time_info = get_time();

    let scan_result: types::ScanResult = types::ScanResult {
        url_info: types::UrlInfo {
            original_url: original_url.to_string(),
            host: url_host.host_str().unwrap().to_string(),
            scheme: url_host.scheme().to_string(),
            port: url_host.port_or_known_default().unwrap().to_string(),
        },
        req_info: req_info,
        document_info: document_info,
        time_info,
        framework_info: framework_inof,
    };

    scan_result
}

fn get_time() -> types::TimeInfo {
    let now  = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339();

    types::TimeInfo {
        created_at: now,
        timezone: "UTC".to_string()
    }
}