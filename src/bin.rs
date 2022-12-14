#[macro_use]
extern crate rocket;

use futures::executor::block_on;
use goose_bumps_backend_lib::database::Database;
use goose_bumps_backend_lib::models::{example_solana_token, example_uuid, User};
use goose_bumps_backend_lib::solana::mint;
use goose_bumps_backend_lib::web3::{deploy_contract, transfer_nft};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header};
use rocket::State;
use rocket::{get, options, post, put, serde::json::Json, serde::uuid::Uuid};
use rocket::{Request, Response};
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

#[options("/user")]
pub async fn options_create_user() {}

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

#[options("/userprogress")]
pub async fn options_userprogress() {}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct MintNFTRequest {
    #[schemars(example = "example_uuid")]
    user_id: uuid::Uuid,
    challenge_id: String,
}

#[openapi(tag = "NFT")]
#[post("/mint-nft", data = "<mint_nft_request>")]
async fn post_mint_nft(
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

#[options("/mint-nft")]
pub async fn options_mint_nft() {}

pub fn example_address() -> &'static str {
    "0x5cf2273601FD25b8CA59d5d22966cD121c1BFafe"
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct TransferNFTRequest {
    #[schemars(example = "example_address")]
    to_address: String,
    token_id: Option<u32>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct TransferNFTResponse {
    transaction_hash: String,
}

#[openapi(tag = "NFT")]
#[post("/transfer-nft", data = "<transfer_nft_request>")]
async fn post_transfer_nft(
    database: &State<Arc<Mutex<Database>>>,
    transfer_nft_request: Json<TransferNFTRequest>,
) -> Json<TransferNFTResponse> {
    let transfer_nft_request = transfer_nft_request.into_inner();
    println!("{}", transfer_nft_request.to_address);
    let token_id = {
        let database = database.try_lock().unwrap();
        let token_counter = database.token_counter;
        transfer_nft_request
            .token_id
            .as_ref()
            .unwrap_or(&token_counter).clone()
    };
    let transaction_hash = transfer_nft(transfer_nft_request.to_address, token_id)
        .await
        .unwrap();
    Json(TransferNFTResponse { transaction_hash })
}

#[options("/transfer-nft")]
pub async fn options_transfer_nft() {}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
pub fn rocket() -> _ {
    let database = Database::new();
    let database = Arc::new(Mutex::new(database));

    block_on(deploy_contract()).unwrap();
    rocket::build()
        .attach(CORS)
        .mount(
            "/",
            routes![
                options_transfer_nft,
                options_mint_nft,
                options_userprogress,
                options_create_user
            ],
        )
        .mount(
            "/",
            openapi_get_routes![
                create_user,
                get_user,
                put_userprogress,
                post_mint_nft,
                post_transfer_nft
            ],
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
