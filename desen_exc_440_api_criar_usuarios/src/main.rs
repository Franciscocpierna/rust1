#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};

// Definição da estrutura User
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    email: String,
}

// Definição da rota para criar um novo usuário
#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: Json<User>) -> String {
    let user = user.into_inner();
    format!("User created: Name: {}, Age: {}, Email: {}", user.name, user.age, user.email)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![new_user])
}