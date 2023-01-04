use crate::app::forms::feedback::DeviceDetailsForm;
use crate::diesel::GroupedBy;
use crate::schema::feedbacks::dsl::feedbacks as feedbacks_dsl;
use crate::{app::forms::feedback::FeedbackForm, schema::feedbacks};
use diesel::{BelongingToDsl, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

use crate::app::models::exception::Exception;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Identifiable)]
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
    pub fn list(conn: &mut SqliteConnection) -> Vec<FeedbackForm> {
        let feedbacks = feedbacks_dsl.load::<Feedback>(conn).unwrap();
        let exceptions = Exception::belonging_to(&feedbacks)
            .load::<Exception>(conn)
            .unwrap()
            .grouped_by(&feedbacks);

        let data = feedbacks.into_iter().zip(exceptions).collect::<Vec<_>>();

        let mut feedbacks: Vec<FeedbackForm> = vec![];

        for feedback in data {
            let form = FeedbackForm {
                category: feedback.0.category,
                email: feedback.0.email,
                message: feedback.0.message,
                device_details: match feedback.0.version_name.clone() {
                    Some(_) => Some(DeviceDetailsForm {
                        version_name: feedback.0.version_name.unwrap(),
                        platform: feedback.0.platform.unwrap().try_into().unwrap(),
                        platform_version: feedback.0.platform_version.unwrap(),
                        branch: feedback.0.branch.unwrap(),
                    }),
                    None => None,
                },
                exceptions: feedback.1.iter().map(|f| f.stack_trace.clone()).collect(),
            };
            feedbacks.push(form);
        }

        feedbacks
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
