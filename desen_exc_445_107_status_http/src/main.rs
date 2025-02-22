#[macro_use] extern crate rocket;

use rocket::http::Status;

#[get("/")]
fn just_fail() -> Status {
    Status::NotAcceptable
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![just_fail])
}
