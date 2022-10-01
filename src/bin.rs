use rocket::{get, post, put, serde::json::Json, serde::uuid::Uuid};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use serde::{Deserialize, Serialize};
use goose_bumps_backend_lib::models::{User, example_solana_token,example_uuid};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct CreateUserRequest {
    #[schemars(example = "example_solana_token")]
    solana_token: String,
}

#[openapi(tag = "Users")]
#[post("/user", data = "<create_user_request>")]
fn create_user(create_user_request: Json<CreateUserRequest>) -> Json<User> {
    let user_id = Uuid::new_v4();
    let create_user_request = create_user_request.into_inner();
    let user = User {
        user_id,
        solana_token: create_user_request.solana_token,
        chapter_ids: vec![],
    };
    Json(user)
}

#[openapi(tag = "Users")]
#[get("/user/<user_id>")]
fn get_user(user_id: uuid::Uuid) -> Option<Json<User>> {
    Some(Json(User {
        user_id,
        solana_token: Uuid::new_v4().to_string(),
        chapter_ids: vec![],
    }))
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct PutUserprogressRequest {
    #[schemars(example = "example_uuid")]
    user_id: uuid::Uuid,
    challenge_id: String,
    chapter_id: String,
}

#[openapi(tag = "Challenge")]
#[put("/userprogress", data = "<put_userprogress_request>")]
fn put_userprogress(put_userprogress_request: Json<PutUserprogressRequest>) -> Option<Json<User>> {
    let userprogress_request = put_userprogress_request.into_inner();
    let user_id = userprogress_request.user_id;
    let chapter_id = userprogress_request.chapter_id;
    Some(Json(User {
        user_id,
        solana_token: Uuid::new_v4().to_string(),
        chapter_ids: vec![chapter_id.clone()],
    }))
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct MintNFTRequest {
    #[schemars(example = "example_uuid")]
    user_id: uuid::Uuid,
    challenge_id: String,
}

#[openapi(tag = "NFT")]
#[post("/mint-nft", data = "<mint_nft_request>")]
fn post_mint_nft(mint_nft_request: Json<MintNFTRequest>) -> () {
    println!("{}", mint_nft_request.into_inner().challenge_id);
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount(
            "/",
            openapi_get_routes![create_user, get_user, put_userprogress, post_mint_nft],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
