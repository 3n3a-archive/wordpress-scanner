use reqwest::{self, header::{USER_AGENT, self}, Response, Client, Url, StatusCode};
use base64;

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

pub async fn get_site<'r>(
    url: Url,
) -> (
    types::DocumentInfo<'r>,
    types::ReqInfo<'r>,
) {
    let client = reqwest::Client::new();
    let result: Response = make_req(client.clone(), url.as_str()).await;

    let robots_url = create_robots_txt_url(url).to_owned();
    let result_robots = make_req(client.clone(), &robots_url).await;
    println!("robots.txt {}", &result_robots.text().await.unwrap());

    let status: StatusCode = *&result.status().clone();
    let status_code_string: String = status.to_string().to_owned();
    let status_code: &str = Box::leak(status_code_string.into_boxed_str());
    let status_reason: &str = status.canonical_reason().unwrap_or("");
    let status: types::ResStatus = types::ResStatus { status_code: status_code, status_reason: status_reason };

    // Headers before .text()
    let boxed_result: Box<header::HeaderMap> = Box::new(result.headers().clone());
    let leaked_res = Box::leak(boxed_result);
    let headers_clone = leaked_res;
    let mut headers: Vec<types::ResHeader<'r>> = Vec::new();
    for (key, value) in headers_clone.iter() {
        let value_string = value.to_str().unwrap_or(&""); // unwrap_or because it fails with UTF-8 Symbols lol
        let header_singular: types::ResHeader<'r> = types::ResHeader {
            name: key.as_str(),
            value: &value_string,
        };
        headers.push(header_singular);
    }

    // .text() destroys the variable, like kinda
    let source_code = Box::new(result.text().await.unwrap());
    let source_code_str = Box::leak(source_code.into_boxed_str());
    let source_code_b64 = Box::new(base64::encode(&source_code_str));
    let source_code_b64_str = Box::leak(source_code_b64.into_boxed_str());
    let (title, css_list, version) = parsers::parse_html(&source_code_str);
    let title_boxed = Box::new(title);
    let title_str = Box::leak(title_boxed.into_boxed_str());

    let css_urls: Vec<types::SourceUrl<'r>> = Vec::new();
    let js_urls: Vec<types::SourceUrl<'r>> = Vec::new();
    let img_urls: Vec<types::ImageUrl<'r>> = Vec::new();
    let link_urls: Vec<types::SourceUrl<'r>> = Vec::new();

    let document_info: types::DocumentInfo<'r> = types::DocumentInfo {
        source_code: source_code_b64_str,
        page_title: title_str,
        css_urls,
        js_urls,
        img_urls,
        link_urls,
    };

    let req_info: types::ReqInfo<'r> = types::ReqInfo{
        headers,
        is_alive: true,
        status,
        timing: types::ResTiming { response_time: "tbd" },
    };

    (document_info, req_info)
}
