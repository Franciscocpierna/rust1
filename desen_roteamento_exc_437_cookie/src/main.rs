#[macro_use] extern crate rocket;

use rocket::http::CookieJar;

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get("message").map(|crumb| format!("Message: {}", crumb.value()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
