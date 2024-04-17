use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

//use crate::model::{course};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    user_id: Uuid,
    language: String,
}

//impl TryFrom<FormData> for NewCourse {
//    type Error = String;
//
//    fn try_from(value: FormData) -> Result<Course::Error> {
//	let course = Course::parse(value)?;
//	Ok(Course(course));
//    }
//}
pub async fn add_course(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let course_id = Uuid::new_v4();
    match sqlx::query!(
        r#"
    INSERT INTO course(id, name, user_id, language, created_at)
    VALUES ($1, $2, $3, $4, $5) 
            "#,
        &course_id,
        &form.name,
        &form.user_id,
        &form.language,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
