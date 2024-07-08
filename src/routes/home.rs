use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HomeResponse {
    pub status: String,
    pub message: String,
}

#[get("/")]
pub fn home_route() -> Json<HomeResponse> {
    Json(HomeResponse {
        status: String::from("Online"),
        message: String::from("This is the home route"),
    })
}
