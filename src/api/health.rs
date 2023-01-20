use actix_web::web::{Data, self};
use actix_web::{get, HttpResponse};
use serde_json::json;
use sqlx::PgPool;

#[get("/health")]
pub async fn get(client: Data<PgPool>) -> HttpResponse {
    // Check database connection:
    let result = test_database(client).await;
    if result.is_err() {
        let data = json!({
            "ready": false,
            "reason": "Database connection error"
        });
        return HttpResponse::InternalServerError().json(data);
    }
    HttpResponse::Ok().json(json!({
        "ready": true,
        "reason": "Everything is OK"
    }))
}

pub async fn test_database(
    postgres: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        SELECT 1
        ",
    )
    .execute(postgres.as_ref())
    .await
    .map(|_| ())
}
