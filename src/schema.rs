// @generated automatically by Diesel CLI.

diesel::table! {
    exceptions (id) {
        id -> Nullable<Integer>,
        feedback_id -> Integer,
        stack_trace -> Text,
        stack_trace_hash -> Text,
    }
}

diesel::table! {
    feedbacks (id) {
        id -> Nullable<Integer>,
        category -> Text,
        email -> Nullable<Text>,
        message -> Text,
        version_name -> Nullable<Text>,
        platform -> Nullable<Integer>,
        platform_version -> Nullable<Text>,
        branch -> Nullable<Text>,
    }
}

diesel::joinable!(exceptions -> feedbacks (feedback_id));

diesel::allow_tables_to_appear_in_same_query!(exceptions, feedbacks,);
