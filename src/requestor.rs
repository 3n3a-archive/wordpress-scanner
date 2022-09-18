use reqwest;
use std::collections::HashMap;

mod parsers {
    use lol_html::{rewrite_str, text, RewriteStrSettings};

    pub fn parse_html_title(html: &String) -> String {
        let mut global_title: String = String::new();
        let element_content_handlers = vec![
            text!("head > title", |t| {
                global_title += t.as_str();
                if t.last_in_text_node() {
                    global_title += "";
                }
                Ok(())
            })
        ];
        rewrite_str(
            html.as_str(),
            RewriteStrSettings {
                element_content_handlers,
                ..RewriteStrSettings::default()
            }
        ).unwrap();
        return global_title;
    }
}

pub async fn get_site(url: &str) -> (String, String, HashMap<String, String>) {
    let client = reqwest::Client::new();
    let result = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36")
        .send()
        .await
        .unwrap();

    // Headers before .text()
    let res_headers = &result.headers().clone();
    let mut headers = HashMap::new();
    for (key, value) in res_headers.iter() {
        let value_string = value.to_str().unwrap_or(&"").to_string();
        headers.insert(key.to_string(), value_string);
    }
    
    // .text() destroys the variable, like kinda 
    let source_code = &result.text().await.unwrap();
    let title = parsers::parse_html_title(&source_code);

    (title, source_code.to_owned(), headers)
}
