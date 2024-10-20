use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: i32,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: i32,
}


pub async fn select(State(state): State<DbState>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = state.client.query("SELECT id, email, password, name, role FROM \"railway\".\"public\".\"user\"", &[])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .into_iter()
        .map(|row| User {
            id: row.get(0),
            email: row.get(1),
            password: row.get(2),
            name: row.get(3),
            role: row.get(4),
        })
        .collect();

    Ok(Json(users))
}

pub async fn insert_one(
    State(state): State<DbState>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let row = state.client.query_one(
        "INSERT INTO \"railway\".\"public\".\"user\" (id, email, password, name, role) VALUES ($1, $2, $3, $4, $5) RETURNING id, email, password, name, role",
        &[&new_user.id, &new_user.email, &new_user.password, &new_user.name, &new_user.role],
    )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let inserted_user = User {
        id: row.get(0),
        email: row.get(1),
        password: row.get(2),
        name: row.get(3),
        role: row.get(4),
    };

    Ok(Json(inserted_user))
}

pub async fn update_one(
    State(state): State<DbState>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let row = state.client.query_one(
        "CALL update_one_user($1, $2, $3, $4, $5, NULL, NULL, NULL, NULL, NULL)",
        &[&new_user.id, &new_user.email, &new_user.password, &new_user.name, &new_user.role],
    )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let inserted_user = User {
        id: row.get("o_id"),
        email: row.get("o_email"),
        password: row.get("o_password"),
        name: row.get("o_name"),
        role: row.get("o_role"),
    };

    Ok(Json(inserted_user))
}

pub async fn delete_one(
    State(state): State<DbState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = state.client.query_one(
        "CALL delete_user($1, NULL)",
        &[&id],
    )
        .await;

    match result {
        Ok(row) => {
            let deleted: bool = row.get("o_deleted");
            if deleted {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err((StatusCode::NOT_FOUND, format!("User with id {} not found", id)))
            }
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}
