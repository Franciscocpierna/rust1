#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};
use std::sync::Mutex;  // Para armazenar a lista de usuários de forma compartilhada

// Definição da estrutura User
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: usize,
    name: String,
    age: u8,
    email: String,
}

// Lista de usuários armazenada em memória (simulada para fins de exemplo)
#[derive(Default)]
struct UserStore {
    users: Mutex<Vec<User>>,  // Protegendo o vetor com Mutex para segurança em threads
}

// Inicializa uma lista de usuários de exemplo
#[rocket::launch]
fn rocket() -> _ {
    let store = UserStore::default();
    store.users.lock().unwrap().push(User {
        id: 1,
        name: "Alice".to_string(),
        age: 25,
        email: "alice@example.com".to_string(),
    });
    store.users.lock().unwrap().push(User {
        id: 2,
        name: "Bob".to_string(),
        age: 30,
        email: "bob@example.com".to_string(),
    });

    rocket::build()
        .manage(store)
        .mount("/", routes![new_user, get_user])
}

// Rota para criar um novo usuário (POST /user)
#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user_store: &rocket::State<UserStore>, user: Json<User>) -> String {
    let mut users = user_store.users.lock().unwrap();
    let mut new_user = user.into_inner();
    new_user.id = users.len() + 1;  // Define um ID simples baseado no tamanho da lista
    users.push(new_user.clone());
    format!("User created: Name: {}, Age: {}, Email: {}", new_user.name, new_user.age, new_user.email)
}

// Rota para buscar um usuário pelo ID (GET /user/<id>)
#[get("/user/<id>", format = "json")]
fn get_user(user_store: &rocket::State<UserStore>, id: usize) -> Option<Json<User>> {
    let users = user_store.users.lock().unwrap();
    users.iter().find(|u| u.id == id).map(|user| Json(user.clone()))
}
