use crate::app::{db::DbPool, forms::feedback::FeedbackForm, models::feedback::Feedback};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/feedback")]
pub async fn list(db_pool: web::Data<DbPool>) -> impl Responder {
    let con_result = db_pool.get();
    if let Err(e) = con_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    let mut con = con_result.unwrap();
    let query_result = web::block(move || Feedback::list(&mut con)).await;
    if let Err(e) = query_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    HttpResponse::Ok().json(query_result.unwrap())
}

#[post("/feedback")]
pub async fn create(
    feedback_form_json: web::Json<FeedbackForm>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let con_result = db_pool.get();
    if let Err(e) = con_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    let feedback_form = feedback_form_json.into_inner();

    let mut con = con_result.unwrap();
    let query_result = web::block(move || Feedback::create(feedback_form, &mut con)).await;
    if let Err(e) = query_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    HttpResponse::Ok().json(query_result.unwrap())
}
