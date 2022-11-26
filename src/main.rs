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
    let (document_info, req_info, framework_inof): (types::DocumentInfo, types::ReqInfo, types::FrameworkInfo) = get_site(url_host.clone()).await;
    
    let scan_result: ScanResult = ScanResult {
        url_info: types::UrlInfo {
            original_url: input.url.to_string(),
            host: url_host.host_str().unwrap().to_string(),
            scheme: url_host.scheme().to_string(),
            port: url_host.port_or_known_default().unwrap().to_string(),
        },
        req_info: req_info,
        document_info: document_info,
        time_info: types::TimeInfo {
            created_at: "Mon 21 November 2022 19:47:22.143 UTC".to_string(),
            timezone: "UTC".to_string()
        },
        framework_info: framework_inof,
    };

    Json(scan_result)
}