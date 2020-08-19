use rocket::{get, launch, response::content, routes, State};
use rocket_contrib::serve::StaticFiles;
use rocket_cors;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/count")]
fn index(hit_count: State<RwLock<()>>) -> content::Html<String> {
    content::Html(format!("hello"))
}

#[get("/ring")]
fn ring(hit_count: State<RwLock<()>>) -> content::Html<String> {
    content::Html(format!("hello"))
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/app",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../static")),
        )
        .mount("/", routes![index, ring])
        .manage(RwLock::new(()))
        .attach(rocket_cors::CorsOptions::default().to_cors().unwrap())
}
