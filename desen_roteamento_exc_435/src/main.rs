#[macro_use] extern crate rocket;

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("This is the handler for a usize: {}", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("This is the handler for an isize: {}", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> String {
    format!("This is the handler for a string: {}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![user, user_int, user_str])
}