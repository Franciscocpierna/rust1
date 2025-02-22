#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

#[get("/")]
fn json() -> Json<&'static str> {
    Json(r#"{"hi": "world"}"#)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![json])
}