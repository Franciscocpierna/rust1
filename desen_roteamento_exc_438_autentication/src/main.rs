#[macro_use] extern crate rocket;

use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};

#[get("/user_id")]
fn user_id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get_private("user_id")
        .map(|crumb| format!("User ID: {}", crumb.value()))
}

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private("user_id");
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[post("/login", data = "<user_id>")]
fn login(user_id: String, cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.add_private(Cookie::new("user_id", user_id));
    Flash::success(Redirect::to("/user_id"), "Successfully logged in.")
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the home page! You can log in or check your user ID."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, user_id, login, logout])
}
