use serde::{Deserialize, Serialize};
use crate::schema::questions;

#[derive(Debug, Deserialize, Serialize, Identifiable, Queryable)]
#[primary_key(question_id)]
#[table_name = "questions"]
pub struct Question {
    pub question_id: i32,
    pub question_type: String,
    pub question_text: String,
    pub correct_answer: String,
    pub wrong_answer_1: String,
    pub wrong_answer_2: String,
    pub wrong_answer_3: String,
}
