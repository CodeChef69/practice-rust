use sqlx::FromRow;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}




impl User {
    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE username = $1",
            username
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, new_user: &NewUser) -> Result<Self, sqlx::Error> {
        // In a real application, you'd want to hash the password before storing it
        sqlx::query_as!(
            Self,
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
            new_user.username,
            new_user.email,
            new_user.password  // Remember to hash this in a real application!
        )
        .fetch_one(pool)
        .await
    }
}