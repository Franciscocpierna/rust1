#[macro_use] extern crate rocket;

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("Você é uma pessoa legal de {} anos, {}!", age, name)
    } else {
        format!("{}, precisamos conversar sobre suas atitudes.", name)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
