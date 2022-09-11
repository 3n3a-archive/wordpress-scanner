#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}