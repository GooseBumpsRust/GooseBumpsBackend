#[macro_use]
extern crate rocket;

use goose_bumps_backend_lib::database::Database;
use goose_bumps_backend_lib::models::{example_solana_token, example_uuid, User};
use goose_bumps_backend_lib::solana::{create_contract, mint};
use rocket::State;
use rocket::{get, post, put, serde::json::Json, serde::uuid::Uuid};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct CreateUserRequest {
    #[schemars(example = "example_solana_token")]
    solana_token: String,
}

#[openapi(tag = "Users")]
#[post("/user", data = "<create_user_request>")]
fn create_user(
    database: &State<Arc<Mutex<Database>>>,
    create_user_request: Json<CreateUserRequest>,
) -> Json<User> {
    let user_id = Uuid::new_v4();
    let create_user_request = create_user_request.into_inner();
    let user = User {
        user_id,
        solana_token: create_user_request.solana_token,
        chapter_ids: vec![],
    };
    let mut database = database.try_lock().unwrap();
    database.users.insert(user_id.to_string(), user.clone());
    Json(user)
}

#[openapi(tag = "Users")]
#[get("/user/<user_id>")]
fn get_user(database: &State<Arc<Mutex<Database>>>, user_id: uuid::Uuid) -> Option<Json<User>> {
    let database = database.try_lock().unwrap();
    let user = database.users.get(&user_id.to_string());
    match user {
        Some(user) => Some(Json(user.clone())),
        None => None,
    }
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
fn put_userprogress(
    database: &State<Arc<Mutex<Database>>>,
    put_userprogress_request: Json<PutUserprogressRequest>,
) -> Option<Json<User>> {
    let userprogress_request = put_userprogress_request.into_inner();
    let user_id = userprogress_request.user_id;
    let chapter_id = userprogress_request.chapter_id;
    let mut database = database.try_lock().unwrap();
    let user = database.users.get(&user_id.to_string());
    match user {
        Some(user) => {
            let mut user = user.clone();
            user.chapter_ids.push(chapter_id);
            database.users.insert(user_id.to_string(), user.clone());
            Some(Json(user.clone()))
        }
        None => None,
    }
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
fn post_mint_nft(
    database: &State<Arc<Mutex<Database>>>,
    mint_nft_request: Json<MintNFTRequest>,
) -> () {
    let mint_nft_request = mint_nft_request.into_inner();
    println!("{}", mint_nft_request.challenge_id);
    let database = database.try_lock().unwrap();
    let user = database.users.get(&mint_nft_request.user_id.to_string());
    match user {
        Some(user) => mint(user.solana_token.to_string()),
        None => (),
    }
}

#[launch]
pub fn rocket() -> _ {
    create_contract().unwrap();
    let database = Database::new();
    let database = Arc::new(Mutex::new(database));

    rocket::build()
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
        .manage(database)
}

#[cfg(test)]
mod tests {
    use crate::rocket;
    use crate::CreateUserRequest;

    use goose_bumps_backend_lib::models::User;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json::json;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let body = json!(CreateUserRequest {
            solana_token: "token".to_string()
        });
        let response = client.post("/user").body(body.to_string()).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let user = response.into_json::<User>().unwrap();
        let user_id = user.user_id;

        let response = client
            .get(format!("/user/{}", user_id.to_string()))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
