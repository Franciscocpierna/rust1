
#[macro_use] extern crate rocket;



#[get("/foo/<_>/bar")]
fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

#[get("/<_..>")]
fn everything() -> &'static str {
    "Hey, you're here."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![foo_bar, everything])
}
