#[macro_use] extern crate rocket;
use rocker_contrib::json::Json;
use serde::Serialize;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};


#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/")]
async fn index() -> Json<Response> {
/*    let result: Result<String, _> = conn.run(|c| {
        c.query_first("SELECT NOW()")
    }).await;

    match result {
        Ok(Some(time)) => format!("Current time: {}", time),
        Ok(None) => "No result".to_string(),
        Err(_) => "Query failed".to_string(),
    }
*/
    let response = Response {
        message: "Backend connected!".to_string(),
    };

    Json(response)
}

#[get("/hello/<name>")]
async fn hello(name: &str) -> String {
    format!("Hello, {name}")
}

#[launch]
fn rocket() -> _{
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]),
        ..Default::default()
    }
    .to_cors()
    .expect("CORS config failed");

    rocket::build()
        .attach(cors)
        .mount("/", routes![index])
        .mount("/", routes![hello])
}
