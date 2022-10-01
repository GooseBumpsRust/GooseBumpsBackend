use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[schemars(example = "example_uuid")]
    pub user_id: uuid::Uuid,
    #[schemars(example = "example_solana_token")]
    pub solana_token: String,
    pub chapter_ids: Vec<String>,
}

pub fn example_uuid() -> &'static str {
    "fdb12d51-0e3f-4ff8-821e-fbc255d8e413"
}

pub fn example_solana_token() -> &'static str {
    "fdb12d51-0e3f-4ff8-821e-fbc255d8e413"
}
