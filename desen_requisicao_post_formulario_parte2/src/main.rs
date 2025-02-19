

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]extern crate rocket;

#[macro_use] extern crate rocket_contrib;
use rocket_contrib::json::Json; 
use rocket::response::Content; 
use rocket::http::{ContentType,Status};
use serde::{Deserializer, Serialize};


#[derive(Serialize, Deserialize)]
struct FormInput{
    name: String,
    email: String,
}
fn validate_input(form_input: &FormInput) -> Result<(), String>{
    if form_input.name.is_empty(){
        return Err("Nome não pode ser vazio".to_string());
    }
    if form_input.email.is_empty(){
        return Err("Email não pode ser vazio".to_string());
    }
    Ok(())
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
fn submit_form(form_input: Json<FormInput>) -> Result<Content<String>, Status>{
    match validate_input(&form_input){
        Ok(_) => {
          let message = format!("Nome: {}\nEmail: {}", form_input.name, form_input.email)
          let html =  format!(r#"
          <!DOCTYPE html>
          <html> 
            <head>
            <title>Formulário</title>
            </head>
            <body>
            <h1>Sucesso!</h1> 
            <p>{}</p>
            </body> 
          </html>"#, message);
          Ok(Content(ContentType,HTML,html.to_string()))

        },
        Err(error) => {
            let html = format!(r#"
            <!DOCTYPE html>
            <html> 
                <head>
                <title>Formulário</title>
                </head>
                <body>
                    <h1>Erro!</h1> 
                    <p>{}</p>
                    <p><a href="/form">Voltar</p>
                </body> 
            </html>"#, error);
          //Content(ContentType,HTML,html.to_string())
          Err(Status::UnprocessableEntity) 
        }
    }
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
