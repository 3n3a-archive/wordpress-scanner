#[macro_use]
extern crate rocket;
use requestor::get_site;
use reqwest::Url;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};
use types::ScanResult;

// use url::Url;

mod requestor;
mod types;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/scan", routes![scan_site])
        // .mount("/scan_old", routes![scan_site_old])
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    let context = context! {
        title: "Wordpress Scanner"
    };
    Template::render("index", &context)
}

#[post("/", data = "<input>")]
async fn scan_site(input: Form<types::ScanForm<'_>>) -> Json<types::ScanResult> {
    
    let url_host = Url::parse(input.url).unwrap();
    let (document_info, req_info): (types::DocumentInfo, types::ReqInfo) = get_site(url_host.clone()).await;
    
    let scan_result: ScanResult = ScanResult {
        url_info: types::UrlInfo {
            original_url: "https://example.com/index.html".to_string(),
            host: "example.com".to_string(),
            scheme: "https".to_string(),
            port: "443".to_string(),
        },
        req_info: req_info,
        document_info: document_info,
        time_info: types::TimeInfo {
            created_at: "Mon 21 November 2022 19:47:22.143 UTC".to_string(),
            timezone: "UTC".to_string()
        },
        framework_info: types::FrameworkInfo {
            name: "Wordpress".to_string(),
            version: "6.2.1".to_string(),
            server: "Nginx/123".to_string(),
            detected_vulnerabilities: vec![]
        },
    };

    Json(scan_result)
}

// #[post("/", data = "<input>")]
// async fn scan_site_old(input: Form<types::ScanForm<'_>>) -> Template {
//     let url_host = Url::parse(input.url).unwrap();
//     let (source_title, source_code, headers, status_code, status_reason, css_list, version_list) =
//         requestor::get_site(url_host.clone()).await;

//     // println!("{:#?}", &css_list.as_slice());

//     let context = context! {
//         title: "Scan Result",
//         url: input.url,
//         url_host: url_host.host_str(),
//         headers: &headers,
//         status_code: status_code,
//         status_reason: status_reason,
//         source_title: source_title,
//         source_code: source_code,
//         source_version: &version_list.as_slice(),
//         css_list: &css_list.as_slice(),
//     };

//     Template::render("scan", &context)
// }
