#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

// Definição da estrutura Task, que será recebida no formato JSON
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    description: String,
    complete: bool,
}

// Rota POST para criar uma nova tarefa
#[post("/todo", data = "<task>")]
fn new_task(task: Json<Task>) -> String {
    let task = task.into_inner();
    format!("New task added: {} - Complete: {}", task.description, task.complete)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![new_task])
}

