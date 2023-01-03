use crate::schema::feedbacks::dsl::feedbacks as feedbacks_dsl;
use crate::{app::forms::feedback::FeedbackForm, schema::feedbacks};
use diesel::{RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

use super::exception::Exception;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = feedbacks)]
pub struct Feedback {
    pub id: Option<i32>,
    pub category: String,
    pub email: Option<String>,
    pub message: String,
    pub version_name: Option<String>,
    pub platform: Option<i32>,
    pub platform_version: Option<String>,
    pub branch: Option<String>,
}

impl Feedback {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        feedbacks_dsl
            .load::<Feedback>(conn)
            .expect("Error loading feedbacks")
    }

    pub fn create(feedback_form: FeedbackForm, conn: &mut SqliteConnection) -> Self {
        let new_feedback = Feedback {
            id: None,
            category: feedback_form.category,
            email: feedback_form.email,
            message: feedback_form.message,
            version_name: match feedback_form.device_details.clone() {
                Some(value) => Some(value.version_name),
                None => None,
            },
            platform: match feedback_form.device_details.clone() {
                Some(value) => Some(value.platform as i32),
                None => None,
            },
            platform_version: match feedback_form.device_details.clone() {
                Some(value) => Some(value.platform_version),
                None => None,
            },
            branch: match feedback_form.device_details.clone() {
                Some(value) => Some(value.branch),
                None => None,
            },
        };

        let inserted_feedback: Feedback = diesel::insert_into(feedbacks_dsl)
            .values(&new_feedback)
            .get_result(conn)
            .expect("Error inserting feedback");

        for exception in feedback_form.exceptions {
            Exception::create(&inserted_feedback.id.unwrap(), &exception, conn);
        }

        inserted_feedback
    }
}
