use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct ClipInfo {
    id: i32,
    name: String,
    description: String,
    uuid: String,
}

pub async fn get_all_clip_info(db: &PgPool) -> Vec<ClipInfo> {
    // get all the clip infos
    let clips = sqlx::query_as!(ClipInfo, "SELECT * FROM clips")
        .fetch_all(db)
        .await
        .unwrap();
    clips
}

pub async fn get_clip_info(id: i32, db: &PgPool) -> ClipInfo {
    // get all the clip infos
    let clips = sqlx::query_as!(ClipInfo, "SELECT * FROM clips WHERE id = $1", id)
        .fetch_one(db)
        .await
        .unwrap();
    clips
}

pub async fn _insert_new_user(user: (String, String), db: &PgPool) -> Result<i32, sqlx::Error> {
    let id = sqlx::query!(
        r#"
        INSERT INTO users (username,password) 
        VALUES ($1,$2)
        RETURNING id
        "#,
        user.0, user.1
    ).fetch_one(db).await.unwrap();

    Ok(id.id)
}
