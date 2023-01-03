use crate::schema::exceptions::dsl::exceptions as exceptions_dsl;
use crate::{schema::exceptions, utils::calculate_hash};
use diesel::{RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = exceptions)]
pub struct Exception {
    pub id: Option<i32>,
    pub feedback_id: i32,
    pub stack_trace: String,
    pub stack_trace_hash: String,
}

impl Exception {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        exceptions_dsl
            .load::<Exception>(conn)
            .expect("Error loading exceptions")
    }

    pub fn create(feedback_id: &i32, exception: &String, conn: &mut SqliteConnection) -> Self {
        let new_exception = Exception {
            id: None,
            feedback_id: feedback_id.to_owned(),
            stack_trace: exception.to_owned(),
            stack_trace_hash: calculate_hash(&exception).to_string(),
        };
        let inserted_exception: Exception = diesel::insert_into(exceptions_dsl)
            .values(&new_exception)
            .get_result(conn)
            .expect("Error inserting exception");

        inserted_exception
    }
}
