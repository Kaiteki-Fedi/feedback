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

#[cfg(test)]
mod tests {
    use actix_web::{
        test::{init_service, read_body, read_body_json, TestRequest},
        web::Data,
        App,
    };
    use serde_json::json;

    use crate::app::{self, db::get_connection_pool, models::feedback::Feedback};

    #[actix_rt::test]
    async fn create_feedback() {
        let con_pool = get_connection_pool();
        let app = init_service(
            App::new()
                .app_data(Data::new(con_pool.clone()))
                .configure(app::init::initialize),
        )
        .await;

        let category_test = "General";
        let message_test = "App is slow";

        let request_body = json!({
            "category": category_test,
            "message": message_test,
            "email": "test@test.com",
            "device_details": {
                "version_name": "version1",
                "platform": "Android",
                "platform_version": "30",
                "branch": "main"
            },
            "exceptions": []
        });

        let resp = TestRequest::post()
            .uri("/feedback")
            .set_json(&request_body)
            .send_request(&app)
            .await;

        assert!(resp.status().is_success(), "Failed create feedback");

        let feedback: Feedback = read_body_json(resp).await;

        assert_eq!(feedback.category, category_test);
        assert_eq!(feedback.message, message_test);
    }

    #[actix_rt::test]
    async fn get_feedbacks() {
        let con_pool = get_connection_pool();
        let app = init_service(
            App::new()
                .app_data(Data::new(con_pool.clone()))
                .configure(app::init::initialize),
        )
        .await;

        let request_body = json!({
            "category": "General",
            "message": "App is slow :(",
            "email": "test@test.com",
            "device_details": {
                "version_name": "version1",
                "platform": "Android",
                "platform_version": "30",
                "branch": "main"
            },
            "exceptions": []
        });

        let resp1 = TestRequest::post()
            .uri("/feedback")
            .set_json(&request_body)
            .send_request(&app)
            .await;

        assert!(resp1.status().is_success(), "Failed to add feedback 1");

        let resp2 = TestRequest::post()
            .uri("/feedback")
            .set_json(&request_body)
            .send_request(&app)
            .await;

        assert!(resp2.status().is_success(), "Failed to add feedback 2");

        let resp3 = TestRequest::get().uri("/feedback").send_request(&app).await;

        assert!(resp3.status().is_success(), "Failed to list feedbacks");

        let body = read_body(resp3).await;
        let feedbacks = serde_json::from_slice::<Vec<Feedback>>(&body).unwrap();

        assert!(
            feedbacks.len() == 2,
            "Number of feedbacks does not match up"
        );
    }
}
