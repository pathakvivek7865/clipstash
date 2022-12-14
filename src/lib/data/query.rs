use super::model::{self, GetClip};
use crate::data::{DataError, DatabasePool};

type Result<T> = std::result::Result<T, DataError>;

// pub async fn get_clip<M: Into<model::GetClip>>(
//     model: M,
//     pool: &DatabasePool,
// ) -> Result<model::Clip> {
//     let model: GetClip = model.into();
//     let shortcode = model.shortcode.as_str();
//     Ok(sqlx::query_as!(
//         model::Clip,
//         "SELECT * FROM clips WHERE shortcode = ?",
//         shortcode
//     )
//     .fetch_one(pool)
//     .await?)
// }
