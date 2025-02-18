
#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket::response::content::RawHtml;
use rocket::State;
use std::collections::HashMap;

struct Local{
    temperatura: f64,
    cidade: String,
}

fn obter_temperatura(local: &Local) -> String{
   local.temperatura.to_string()
} 


#[get("/")]
fn index() -> &'static str {
    "bem vindo a nosssa API!"
}
 /*
#[get("/temperatura/<cidade>")] 
fn temperatura(local_state: &State<Local>, cidade: String) -> RawHtml<String>{
  let local = local_state.inner();
  if local.cidade == cidade {
    content::RawHtml(format!("A temperatura da cidade: {} é de {} graus C", cidade, obter_temperatura(local)))
  } else {
    content::RawHtml(format!("não foi possível encontrar a temperatura da cidade: {}", cidade))
  }

}
  */
#[get("/temperatura/<cidade>")]
fn temperatura(local_state: &State<HashMap<String, Local>>, cidade: String) -> RawHtml<String> {
    let local_state = local_state.inner();
    /*if let Some(local) = local_state.get(&cidade) { 
        RawHtml(format!("A temperatura da cidade: {} é de {} graus C", cidade, obter_temperatura(local)))
    } else {
        RawHtml(format!("não foi possível encontrar a temperatura da cidade: {}", cidade))
    }*/ 

    // o codigo acima podem ser substituidos pelo codigo abaixo
    match local_state.get(&cidade) {
        Some(local) => RawHtml(format!("A temperatura da cidade: {} é de {} graus C", cidade, obter_temperatura(local))),
        None => RawHtml(format!("não foi possível encontrar a temperatura da cidade: {}", cidade))
    }
}  


#[launch]
fn rocket() -> _ {
  /* rocket::build().mount("/", routes![index,temperatura])
  .manage(Local{
    temperatura: 20.0, 
    cidade: "São Paulo".to_string()
  })*/
  let mut local_state =  HashMap::<String, Local>::new();
  local_state.insert("São Paulo".to_string(), Local{
    temperatura: 20.0,
    cidade: "São Paulo".to_string()
  });
  local_state.insert("Rio de Janeiro".to_string(), Local{
        temperatura: 40.0,
        cidade: "Rio de Janeiro".to_string()
   });

   local_state.insert("Belo Horizonte".to_string(), Local{
        temperatura: 30.0,
        cidade: "Belo Horizonte".to_string()
   });
   rocket::build()
   .mount("/", routes![index,temperatura])
   .manage(local_state)

}


