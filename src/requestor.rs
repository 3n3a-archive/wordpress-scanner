use reqwest;
use std::collections::HashMap;

mod parsers {
    use lol_html::{element, rewrite_str, text, RewriteStrSettings};

    pub fn parse_html(html: &String) -> (String, Vec<String>, Vec<String>) {
        let mut global_title: String = String::new();
        let mut global_css_list = Vec::new();
        let mut global_version: Vec<String> = Vec::new();

        let element_content_handlers = vec![
            text!("head > title", |t| {
                global_title += t.as_str();
                if t.last_in_text_node() {
                    global_title += "";
                }
                Ok(())
            }),
            element!("head > link[rel=\"stylesheet\"]", |e| {
                // push into vector or so
                let href = e.get_attribute("href").unwrap();
                global_css_list.push(href);
                Ok(())
            }),
            element!("head > meta[name=\"generator\"]", |e| {
                let version = e.get_attribute("content").unwrap();
                global_version.push(version);
                Ok(())
            }),
        ];
        rewrite_str(
            html.as_str(),
            RewriteStrSettings {
                element_content_handlers,
                ..RewriteStrSettings::default()
            },
        )
        .unwrap();
        return (global_title, global_css_list, global_version);
    }
}

pub async fn get_site(
    url: &str,
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
    let result = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36")
        .send()
        .await // no '?' because we'd have to use Result as return type
        .unwrap();

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
