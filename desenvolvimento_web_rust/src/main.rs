
#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}
#[get("/users/<id>")]
fn get_user(id : i32) -> String {
    format!("retornando o usuario com  id: {}", id)
}


#[post("/users/<name>")]
fn create_user(name: String){
    println!("criando o usuario com nome: {}", name);
} 

#[delete("/users/<id>")]
fn delete_user(id: i32){
    println!("deletando o usuario com id: {}", id);
}
#[put("/users?<id>&<name>")]
fn update_user(id: u32, name: String){
    println!("atualizando o usuario com id: {} e nome: {}", id, name);
}
#[get("/users?<query>&<page>")]
fn search_user(query: String, page: Option<u32>) -> String {
 
    match page {
        Some(p) => format!("buscando o usuario com a consulta: '{}' na pagina: {}", query, p),
        None => format!(" bascando o usuário com a consulta: '{}' (sem espaecificar a página)", query)
        
    }
}

#[catch(404)]
fn not_found() -> &'static str {
    "Página não encontrada"
}

#[launch]
fn rocket() -> _ {
   // rocket::build().mount("/", routes![index,get_user,create_user,delete_user,update_user,search_user])
   rocket::build()
   .mount("/",routes![index])
   .register("/",catchers![not_found])

}

/*fn main() {
    rocket().launch();
    
}*/
//rocket::build().mount("/", routes![index,hello,number,search])
/*#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}
 #[get("/number/<number>")]
 fn number(number: i32) -> String {
    format!("Number: {}", number)
 }
 */

/* #[get("/search?<query>&<max_results>&<page>")] // exemplo funcionou
 fn search(query: String,max_results: i32,page: i32) -> String {
    format!("Searching for: {}, max_results: {}, page: {}", query, max_results, page)
 }
*/
/*
#[get("/search?<query>&<typ>")]
fn search(query: String, typ: Option<String>) -> String {
    match typ {
        Some(t) => format!("Searching for: {}, (type: {})", query, t),
        None => format!("Searching for: {} (no type specified)", query)
    }
 }
*/    
    

