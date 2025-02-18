

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]extern crate rocket;

#[macro_use] extern crate rocket_contrib;
use rocket_contrib::json::Json; 
use rocket::response::Content; 
use rocket::http::ContentType;
use serde::{Deserializer, Serialize};


#[derive(Serialize, Deserialize)]
struct FormInput{
    username: String,
    password: String,
}

#[get("/form")]
fn form() -> Content<String>{
    let html = r#"
    <!DOCTYPE html>
    <html> 
         <head>
           <title>Formulário</title>
         
         </head>
         <body>
            <h1>exemplo de Formulário</h1> 
         </body> 
    </html>
    "#;
    Content(ContentType,HTML,html.to_string())

}


#[get("/")]
fn index() -> String {
    format!("Bem-Vindo a nossa API!")
}



#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, form])
}

/*fn main() {
    rocket::Ignite() //não esta aceitando o comando Ignite 
    .mount("/", routes![index, form])
    .launch();
}*/
