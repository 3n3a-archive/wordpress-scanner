#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

mod scanner;
mod requestor;
mod types;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/scan", routes![scan_site])
}

#[get("/")]
fn index() -> Json<types::ServerInfo> {
    Json(types::ServerInfo { name: "Wordpress Scanner".to_string(), version: "0.3.0".to_string() })
}

#[post("/", format = "json", data = "<input>")]
async fn scan_site(input: Json<types::WebScanInput<'_>>) -> Json<types::ScanResult> {
    let scan_result = scanner::scan_site(input.url).await;
    Json(scan_result)
}
