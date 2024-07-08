#[macro_use] extern crate rocket;
use dotenv::dotenv;

mod utils;
mod routes;
mod database;
mod models;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    // Establish database connection
    let db_pool = database::establish_connection().await;

    rocket::build()
        .mount("/", routes![
            routes::home::home_route,
            routes::auth::login::auth_route,
            routes::auth::login::register_route
        ])
        .manage(db_pool) // Make the database pool available to all routes
}