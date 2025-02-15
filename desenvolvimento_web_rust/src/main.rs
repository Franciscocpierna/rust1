
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}
#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}
 #[get("/number/<number>")]
 fn number(number: i32) -> String {
    format!("Number: {}", number)
 }

/* #[get("/search?<query>&<max_results>&<page>")] // exemplo funcionou
 fn search(query: String,max_results: i32,page: i32) -> String {
    format!("Searching for: {}, max_results: {}, page: {}", query, max_results, page)
 }
*/

#[get("/search?<query>&<typ>")]
fn search(query: String, typ: Option<String>) -> String {
    match typ {
        Some(t) => format!("Searching for: {}, (type: {})", query, t),
        None => format!("Searching for: {} (no type specified)", query)
    }
 }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,hello,number,search])
}

/*fn main() {
    rocket().launch();
    
}*/

