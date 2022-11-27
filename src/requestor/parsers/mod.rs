use std::str;
use lol_html::{element, rewrite_str, text, RewriteStrSettings};

use crate::types::{SourceUrl, ImageUrl};

pub struct DocumentSubsetInfo {
    pub title: String,
    pub generator_info: Vec<String>,
    pub css_urls: Vec<SourceUrl>,
    pub js_urls: Vec<SourceUrl>,
    pub link_urls: Vec<SourceUrl>,
    pub img_urls: Vec<ImageUrl>,
}

pub fn parse_html(html: &str) -> DocumentSubsetInfo {
    let mut global_title: String = String::new();
    let mut global_css_list: Vec<SourceUrl> = Vec::new();
    let mut global_js_list: Vec<SourceUrl> = Vec::new();
    let mut global_link_list: Vec<SourceUrl> = Vec::new();
    let mut global_img_list: Vec<ImageUrl> = Vec::new();
    let mut global_generator: Vec<String> = Vec::new();
    let mut global_generator2: Vec<String> = Vec::new();

    let element_content_handlers = vec![
        text!("head > title", |t| {
            global_title += t.as_str();
            if t.last_in_text_node() {
                global_title += "";
            }
            Ok(())
        }),
        element!("head > link[rel=\"stylesheet\"]", |e| {
            let href = e.get_attribute("href").unwrap();
            global_css_list.push(SourceUrl { url: href });
            Ok(())
        }),
        element!("script[src]", |e| {
            let src = e.get_attribute("src").unwrap();
            global_js_list.push(SourceUrl { url: src });
            Ok(())
        }),
        element!("a[href]", |e| {
            let href = e.get_attribute("href").unwrap();
            global_link_list.push(SourceUrl { url: href });
            Ok(())
        }),
        element!("img[src]", |e| {
            let src = e.get_attribute("src").unwrap();
            let alt = e.get_attribute("alt").unwrap_or("".to_string());
            global_img_list.push(ImageUrl {url:src, alt:alt });
            Ok(())
        }),
        element!("head > meta[name=\"generator\"]", |e| {
            let generator = e.get_attribute("content").unwrap();
            global_generator.push(generator);
            Ok(())
        }),
        element!("head > meta[name=\"Generator\"]", |e| {
            let generator = e.get_attribute("content").unwrap();
            global_generator2.push(generator);
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

    let mut generator_collection: Vec<String> = Vec::new();
    generator_collection.append(&mut global_generator);
    generator_collection.append(&mut global_generator2);

    DocumentSubsetInfo { 
        title: global_title, 
        generator_info: generator_collection,
        css_urls: global_css_list, 
        js_urls: global_js_list, 
        link_urls: global_link_list, 
        img_urls: global_img_list 
    }
}