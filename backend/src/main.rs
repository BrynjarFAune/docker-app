#[macro_use] extern crate rocket;
use rocket_sync_db_pools::{database, diesel};

#[database("mysql_db")]
struct DbConn(diesel::MysqlConnection);

#[get("/")]
async fn index(conn: DbConn) -> String {
/*    let result: Result<String, _> = conn.run(|c| {
        c.query_first("SELECT NOW()")
    }).await;

    match result {
        Ok(Some(time)) => format!("Current time: {}", time),
        Ok(None) => "No result".to_string(),
        Err(_) => "Query failed".to_string(),
    }
*/
    "Backend connected!"
}

#[get("/hello/<name>")]
async fn hello(conn: DbConn, name: &str) -> String {
    format!("Hello, {name}")
}

#[launch]
fn rocket() -> _{
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/", routes![hello])
}
