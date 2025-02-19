

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
           <form method="post" action="/form">
                <label for="name">Nome:</label>
                <input type="text" name = "name">
                <br>
                <label for="email">Email:</label>
                <input type="email" name="email">
                <br>
                
               <button type="submit">Enviar</button>
            </form>          
         </body> 
    </html>
    "#;
    Content(ContentType,HTML,html.to_string())

}
#[post("/form", data = "<form_input>")]
fn submit_form(form_input: Json<FormInput>) -> String{
    format!("Username: {}\nPassword: {}", form_input.username,   form_input.password)
}

#[get("/")]
fn index() -> String {
    format!("Bem-Vindo a nossa API!")
}



#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, form, submit_form])
}

/*fn main() {
    rocket::Ignite() //não esta aceitando o comando Ignite 
    .mount("/", routes![index, form])
    .launch();
}*/
