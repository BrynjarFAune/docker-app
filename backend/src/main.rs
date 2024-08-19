#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::Serialize;

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

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions {
        allowed_origins: rocket_cors::AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build().attach(cors).mount("/", routes![index])
}
