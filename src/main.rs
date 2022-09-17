#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{Template, context};

mod requestor;

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

#[get("/<url>")]
async fn scan_site(url: &str) -> String {
    let text = requestor::get_site(url).await.unwrap();
    text
}