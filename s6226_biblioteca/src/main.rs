//use comprimentos::cumprimentar_usuario;
//use anyhow::Error;
use to_do_list::ToDoList;

fn main() {
    /*let result: Result<String, Error> = cumprimentar_usuario();
    match result{
        Ok(user_name) => println!("Bem-Vindo {} ",user_name),
        Err(err) => eprintln!("Error {}", err),
    }  // aula anterior */
    let mut to_do_list = ToDoList::new();
    to_do_list.add_task("Gravar aula");
    to_do_list.add_task("ir ao mercado");
    to_do_list.add_task("ir ao médico");

    // Marcar a primeira task como concluida
    to_do_list.completed_task(0).unwrap();

    // imprimir a lista de tarefas
    println!("Lista de tarefas");
    for (index, task) in to_do_list.get_tasks().iter().enumerate(){
        println!("{} - {} {}",index + 1, task.description, if task.completed{"[concluida]"} else{""});
    }
    // limpar as tarefas
    to_do_list.clear_tasks();

    println!("\nLista de tarefas após limpeza");
    for (index, task) in to_do_list.get_tasks().iter().enumerate(){
        println!("{} - {} {}",index + 1, task.description, if task.completed{"[concluida]"} else{""});
    }
}
