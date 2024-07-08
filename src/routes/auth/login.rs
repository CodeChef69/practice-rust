use crate::models::user::{ NewUser, User};
use crate::utils::{hash_password, verify_password};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{post, State};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Deserialize)]
pub struct RegistrationRequest {
    username: String,
    email: String,
    password: String,
}
#[post("/auth", data = "<login_request>")]
pub async fn auth_route(
    db: &State<PgPool>,
    login_request: Json<LoginRequest>,
) -> Json<serde_json::Value> {
    let username = &login_request.username;
    let password = &login_request.password;
    match User::find_by_username(&**db, username).await {
        Ok(Some(user)) => {
            if verify_password(password, &user.password_hash) {
                Json(serde_json::json!({
                    "status": "success",
                    "message": "Login successful",
                    "user": {
                        "id": user.id,
                        "username": user.username,
                        "email": user.email
                    }
                }))
            } else {
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Invalid password"
                }))
            }
        },
        Ok(None) => Json(serde_json::json!({
            "status": "error",
            "message": "User not found"
        })),
        Err(e) => Json(serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e)
        })),
    }
}

#[post("/register", data = "<new_user>")]
pub async fn register_route(
    db: &State<PgPool>,
    new_user: Json<RegistrationRequest>,
) -> Json<serde_json::Value> {
    let hashed_password = match hash_password(&new_user.password) {
        Ok(hashed) => hashed,
        Err(e) => {
            return Json(serde_json::json!({
                "status": "error",
                "message": format!("Error hashing password: {}", e)
            }))
        }
    };

    let new_user = NewUser {
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password: hashed_password,
    };

    match User::create(&**db, &new_user).await {
        Ok(user) => Json(serde_json::json!({
            "status": "success",
            "user": user
        })),
        Err(e) => Json(serde_json::json!({
            "status": "error",
            "message": format!("Failed to create user: {}", e)
        })),
    }
}
