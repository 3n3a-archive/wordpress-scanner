use reqwest::{self, header::{USER_AGENT, self}, Response, Client, Url, StatusCode};
use base64;
use std::time::{Duration, Instant};

mod config;
mod parsers;

// #[path="./types/mod.rs"]
// pub mod types;

use super::types;

// makes a request with a random user-agent
async fn make_req(client: Client, url: &str) -> Response {
    let result = client.get(url)
        .header(USER_AGENT, config::get_random_user_agent())
        .send()
        .await // no '?' because we'd have to use Result as return type
        .unwrap();
    result
}

fn create_robots_txt_url(url: Url) -> String {
    const ROBOTS_TXT: &'static str = "robots.txt";
    let schema = url.scheme();
    let host = url.host_str().unwrap();
    let port = url.port_or_known_default().unwrap();
    let url: String = format!("{}://{}:{}/{}", schema, host, port, ROBOTS_TXT);
    url
}

pub async fn get_site(
    url: Url,
) -> (
    types::DocumentInfo,
    types::ReqInfo,
) {
    let start = Instant::now();

    let client = reqwest::Client::new();
    let result: Response = make_req(client.clone(), url.as_str()).await;

    let robots_url = create_robots_txt_url(url).to_owned();
    let result_robots = make_req(client.clone(), &robots_url).await;
    println!("robots.txt {}", &result_robots.text().await.unwrap());

    let status = &result.status().clone();
    let status_code = status.to_string();
    let status_reason = status.canonical_reason().unwrap_or("").to_string();
    let status: types::ResStatus = types::ResStatus { 
        status_code: status_code, 
        status_reason: status_reason,
    };

    // Headers before .text()
    let boxed_result: Box<header::HeaderMap> = Box::new(result.headers().clone());
    let leaked_res = Box::leak(boxed_result);
    let headers_clone = leaked_res;
    let mut headers: Vec<types::ResHeader> = Vec::new();
    for (key, value) in headers_clone.iter() {
        let value_string = value.to_str().unwrap_or(&"").to_string(); // unwrap_or because it fails with UTF-8 Symbols lol
        let header_singular: types::ResHeader = types::ResHeader {
            name: key.to_string(),
            value: value_string,
        };
        headers.push(header_singular);
    }

    // .text() destroys the variable, like kinda
    let source_code = result.text().await.unwrap();
    let source_code_b64 = base64::encode(&source_code);
    let (title, css_list, version) = parsers::parse_html(&source_code);

    let css_urls: Vec<types::SourceUrl> = Vec::new();
    let js_urls: Vec<types::SourceUrl> = Vec::new();
    let img_urls: Vec<types::ImageUrl> = Vec::new();
    let link_urls: Vec<types::SourceUrl> = Vec::new();

    let duration: String = start.elapsed().as_millis().to_string() + " ms";

    let document_info: types::DocumentInfo = types::DocumentInfo {
        source_code: source_code_b64,
        page_title: title,
        css_urls,
        js_urls,
        img_urls,
        link_urls,
    };

    let req_info: types::ReqInfo = types::ReqInfo{
        headers,
        is_alive: true,
        status,
        timing: types::ResTiming { response_time: duration },
    };

    (document_info, req_info)
}
