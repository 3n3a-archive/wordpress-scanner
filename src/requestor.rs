use reqwest::{self, header::USER_AGENT, Response, Client, Url};
use std::collections::HashMap;

mod config;
mod parsers;

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
    String,
    String,
    HashMap<String, String>,
    String,
    String,
    Vec<String>,
    Vec<String>,
) {
    let client = reqwest::Client::new();
    let result = make_req(client.clone(), url.as_str()).await;

    let robots_url = create_robots_txt_url(url).to_owned();
    let result_robots = make_req(client.clone(), &robots_url).await;
    println!("robots.txt {}", &result_robots.text().await.unwrap());

    let status_code: String = (*(&result.status().to_owned())).to_string();
    let status_reason: String = (*(&result.status().canonical_reason().unwrap_or(""))).to_string();

    // Headers before .text()
    let res_headers = &result.headers().clone(); // clone so can still return it (because .text() takes over ownership)
    let mut headers = HashMap::new();
    for (key, value) in res_headers.iter() {
        let value_string = value.to_str().unwrap_or(&"").to_string(); // unwrap_or because it fails with UTF-8 Symbols lol
        headers.insert(key.to_string(), value_string);
    }

    // .text() destroys the variable, like kinda
    let source_code = &result.text().await.unwrap();
    let (title, css_list, version) = parsers::parse_html(&source_code);

    (
        title,
        source_code.to_owned(),
        headers,
        status_code,
        status_reason,
        css_list,
        version,
    )
}
