// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Nullable<Text>,
        solana_token -> Text,
        chapter_ids -> Text,
    }
}
