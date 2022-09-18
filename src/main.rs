#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};
use url::Url;

mod requestor;
mod types;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/scan", routes![scan_site])
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
async fn scan_site(input: Form<types::ScanForm<'_>>) -> Template {
    let url_host = Url::parse(input.url).unwrap();
    let (
        source_title, 
        source_code, 
        headers,
        status_code,
        status_reason,
        css_list
    ) = requestor::get_site(input.url).await;

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
        css_list: &css_list.as_slice(),
    };

    Template::render("scan", &context)
}