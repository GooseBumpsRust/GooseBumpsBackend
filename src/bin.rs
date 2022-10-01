use rocket::{get, post, serde::json::Json, serde::uuid::Uuid};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    #[schemars(example = "example_uuid")]
    user_id: uuid::Uuid,
    #[schemars(example = "example_solana_token")]
    solana_token: String,
}

fn example_uuid() -> &'static str {
    "fdb12d51-0e3f-4ff8-821e-fbc255d8e413"
}

fn example_solana_token() -> &'static str {
    "fdb12d51-0e3f-4ff8-821e-fbc255d8e413"
}

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
    };
    Json(user)
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![create_user,])
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
