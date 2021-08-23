table! {
    questions (question_id) {
        question_id -> Int4,
        question_type -> Text,
        question_text -> Text,
        correct_answer -> Text,
        wrong_answer_1 -> Text,
        wrong_answer_2 -> Text,
        wrong_answer_3 -> Text,
    }
}
