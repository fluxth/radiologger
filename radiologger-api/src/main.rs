#[macro_use]
extern crate rocket;

mod catchers;

use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
struct ServerInfo {
    r#type: &'static str,
    server: &'static str,
    version: &'static str,
}

#[get("/")]
fn index() -> Json<ServerInfo> {
    Json::from(ServerInfo {
        r#type: "info",
        server: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[derive(Serialize)]
struct AuthenticationResponse<'a> {
    r#type: &'static str,
    username: &'a str,
    token: &'a str,
}

#[post("/authenticate")]
fn authenticate() -> Json<AuthenticationResponse<'static>> {
    AuthenticationResponse<'static> {
        r#type: "result",
        username: "test",
        token: "1234",
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![index])
        .register("/", catchers![catchers::default_catcher])
}
