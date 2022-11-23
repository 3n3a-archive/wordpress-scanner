#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};
use types::ScanResult;
use url::Url;

mod requestor;
mod types;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/scan", routes![scan_site])
        .mount("/scan_old", routes![scan_site_old])
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
async fn scan_site(input: Form<types::ScanForm<'_>>) -> Json<types::ScanResult<'_>> {
    let scan_result: ScanResult<'_> = ScanResult {
        url_info: types::UrlInfo {
            original_url: "https://example.com/index.html",
            host: "example.com",
            scheme: "https",
            port: "443",
        },
        req_info: types::ReqInfo {
            headers: vec![types::ResHeader {
                name: "Content-Type",
                value: "application/json",
            }],
            status: types::ResStatus {
                status_code: "200",
                status_reason: "OK",
            },
            is_alive: true,
            timing: types::ResTiming {
                response_time: "2 ms",
            },
        },
        document_info: types::DocumentInfo {
            source_code: "JTNDIURPQ1RZUEUlMjBodG1sJTNFJTNDaHRtbCUyMGxhbmc9JTIyZW4lMjIlM0UlM0NoZWFkJTNFJTNDbWV0YSUyMGNoYXJzZXQ9JTIyVVRGLTglMjIlM0UlM0NtZXRhJTIwaHR0cC1lcXVpdj0lMjJYLVVBLUNvbXBhdGlibGUlMjIlMjBjb250ZW50PSUyMklFPWVkZ2UlMjIlM0UlM0NtZXRhJTIwbmFtZT0lMjJ2aWV3cG9ydCUyMiUyMGNvbnRlbnQ9JTIyd2lkdGg9ZGV2aWNlLXdpZHRoLCUyMGluaXRpYWwtc2NhbGU9MS4wJTIyJTNFJTNDdGl0bGUlM0VEb2N1bWVudCUzQy90aXRsZSUzRSUzQy9oZWFkJTNFJTNDYm9keSUzRSUzQy9ib2R5JTNFJTNDL2h0bWwlM0U=",
            page_title: "Document",
            css_urls: vec![types::SourceUrl {
                url: "style.css"
            }],
            js_urls: vec![types::SourceUrl {
                url: "/script.js"
            }],
            img_urls: vec![types::ImageUrl {
                url: "image.jpg",
                alt: ""
            }],
            link_urls: vec![types::SourceUrl {
                url: "https://www.google.com"
            }],
        },
        time_info: types::TimeInfo {
            created_at: "Mon 21 November 2022 19:47:22.143 UTC",
            timezone: "UTC"
        },
        framework_info: types::FrameworkInfo {
            name: "Wordpress",
            version: "6.2.1",
            server: "Nginx/123",
            detected_vulnerabilities: vec![]
        },
    };

    Json(scan_result)
}

#[post("/", data = "<input>")]
async fn scan_site_old(input: Form<types::ScanForm<'_>>) -> Template {
    let url_host = Url::parse(input.url).unwrap();
    let (source_title, source_code, headers, status_code, status_reason, css_list, version_list) =
        requestor::get_site(url_host.clone()).await;

    // println!("{:#?}", &css_list.as_slice());

    let context = context! {
        title: "Scan Result",
        url: input.url,
        url_host: url_host.host_str(),
        headers: &headers,
        status_code: status_code,
        status_reason: status_reason,
        source_title: source_title,
        source_code: source_code,
        source_version: &version_list.as_slice(),
        css_list: &css_list.as_slice(),
    };

    Template::render("scan", &context)
}
