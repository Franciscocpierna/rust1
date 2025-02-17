
#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket::response::content::RawHtml;
use rocket::State;

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
 
#[get("/temperatura/<cidade>")] 
fn temperatura(local_state: &State<Local>, cidade: String) -> RawHtml<String>{
  let local = local_state.inner();
  if local.cidade == cidade {
    content::RawHtml(format!("A temperatura da cidade: {} é de {} graus C", cidade, obter_temperatura(local)))
  } else {
    content::RawHtml(format!("não foi possível encontrar a temperatura da cidade: {}", cidade))
  }

}
    


#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index,temperatura])
  .manage(Local{
    temperatura: 20.0, 
    cidade: "São Paulo".to_string()
})

}


