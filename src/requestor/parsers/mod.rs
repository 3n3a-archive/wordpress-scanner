use std::str;
use base64;
use lol_html::{element, rewrite_str, text, RewriteStrSettings};

pub fn parse_html(html: &str) -> (String, Vec<String>, Vec<String>) {
    let mut global_title: String = String::new();
    let mut global_css_list = Vec::new();
    let mut global_version: Vec<String> = Vec::new();

    let mut r_global_version = &mut global_version;

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
        element!("head > meta[name=\"generator\" i]", |e| {
            let r1_gv = &mut r_global_version;
            let version = e.get_attribute("content").unwrap();
            r1_gv.push(version);
            Ok(())
        }),
    ];
    rewrite_str(
        html,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .unwrap();
    return (global_title, global_css_list, global_version);
}