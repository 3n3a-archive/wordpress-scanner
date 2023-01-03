#[macro_use]
extern crate rocket;

use rocket::{serde::{json::Json, Deserialize}, fairing::AdHoc, State};

mod scanner;
mod requestor;
mod types;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Config {
    base_url: String,
    username: String,
    password: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![unauthorized, bad_request])
        .register("/scan", catchers![unauthorized, bad_request])
        .mount("/", routes![index])
        .mount("/scan", routes![scan_site])
        .attach(AdHoc::config::<Config>())
}

#[get("/")]
fn index() -> Json<types::ServerInfo> {
    Json(types::ServerInfo { name: "Wordpress Scanner".to_string(), version: "0.3.1".to_string() })
}

#[post("/", format = "json", data = "<input>")]
async fn scan_site(_key: types::ApiKey, input: Json<types::WebScanInput<'_>>) -> Result<Json<types::ScanResult>, Json<types::ErrorResult>> {
    let scan_result = scanner::scan_site(input.url).await;
    Ok(Json(scan_result))
}

#[catch(401)]
fn unauthorized(status: rocket::http::Status, _req: &rocket::request::Request) -> Json<types::ErrorResult> {
    Json(types::ErrorResult{
        err: "Unauthorized access".to_string(),
        msg: Some("Please ensure you have the correct access".to_string()),
        http_status_code: status.code,
    })
}

#[catch(400)]
fn bad_request(status: rocket::http::Status, _req: &rocket::request::Request) -> Json<types::ErrorResult> {
    Json(types::ErrorResult{
        err: "Bad Request".to_string(),
        msg: Some("Please ensure you entered the correct parameters".to_string()),
        http_status_code: status.code,
    })
}