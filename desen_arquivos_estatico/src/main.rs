use rocket::fs::FileServer;
use rocket::{launch, build, Rocket};

#[launch]
fn rocket() -> Rocket<rocket::Build> {
    build()
        .mount("/public", FileServer::from("static_files"))
}

