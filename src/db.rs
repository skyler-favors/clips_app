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

pub async fn edit_clip(db: &PgPool, id: i32, name: &str, description: &str) {
    let _id = sqlx::query!(
        r#"
        UPDATE clips
        SET name = $1,
        description = $2
        WHERE id = $3
        "#,
        name,
        description,
        id
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn delete_clip(id: i32, db: &PgPool) {
    let _id = sqlx::query!(
        r#"
        DELETE FROM clips
        WHERE id = $1
        "#,
        id
    )
    .execute(db)
    .await
    .unwrap();
}
