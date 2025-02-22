#[macro_use] extern crate rocket;

use rocket::form::Form;  // Importação para o uso de Form
use std::sync::Mutex;

// Definição da estrutura Task que vai representar cada tarefa
#[derive(FromForm, Clone)]
struct Task {
    complete: bool,
    description: String,
}

// Armazena a lista de tarefas
#[derive(Default)]
struct TodoStore {
    tasks: Mutex<Vec<Task>>,  // Usamos Mutex para garantir a segurança entre threads
}

// Rota POST para adicionar uma nova tarefa
#[post("/todo", data = "<task>")]
fn new_task(task_store: &rocket::State<TodoStore>, task: Form<Task>) -> String {
    let mut tasks = task_store.tasks.lock().unwrap();
    let new_task = task.into_inner();
    tasks.push(new_task.clone());
    format!("Task added: Description: {}, Complete: {}", new_task.description, new_task.complete)
}

// Rota GET para listar todas as tarefas
#[get("/todo")]
fn list_tasks(task_store: &rocket::State<TodoStore>) -> String {
    let tasks = task_store.tasks.lock().unwrap();
    tasks.iter()
         .map(|t| format!("Description: {}, Complete: {}\n", t.description, t.complete))
         .collect()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TodoStore::default())
        .mount("/", routes![new_task, list_tasks])
}
