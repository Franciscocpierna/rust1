#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;

#[get("/page/<path..>")]
async fn get_page(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let base_path = Path::new("static_files");
    let file_path = base_path.join(path);

    NamedFile::open(file_path).await.map_err(|_| NotFound("File not found".into()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_page])
}
