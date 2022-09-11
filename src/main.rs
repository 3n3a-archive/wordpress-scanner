#[macro_use]
extern crate rocket;

mod requestor;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
}

#[get("/")]
async fn hello() -> String {
    let text = requestor::get_site().await.unwrap();
    text
}