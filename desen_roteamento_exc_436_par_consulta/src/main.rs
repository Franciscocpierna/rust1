#[macro_use] extern crate rocket;

use rocket::response::status::BadRequest;

#[get("/<param>?<a>&<b>&<c>")]
fn index(param: isize, a: Option<String>, b: Option<u32>, c: Option<bool>) -> Result<String, BadRequest<String>> {
    let a = a.unwrap_or_else(|| "default_value".to_string());
    let b = b.unwrap_or(0);
    let c = c.unwrap_or(false);

    Ok(format!("Received param: {}, a: {}, b: {}, c: {}", param, a, b, c))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
