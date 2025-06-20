use axum::{
    routing::{get, post},
    Router,
    extract::{Path, Query, Json}
};
use std::collections::HashMap;
use serde::Deserialize;

#[tokio::main]
async fn main(){
let app = Router::new()
    .route("/" , get(root) )
    .route("/public", get(get_public_key).post(post_public))
    .route("/private/secure", get(get_private));

//  Merge Router inside main function

    // Router::new().merge(router_hello))

}

async fn root() {}
async fn get_public_key() {}
async fn post_public() {}
async fn get_private() {}

// tottaly same as const [ link ] = usePathname(); in next.js
async fn path(Path(user_id): Path<u32>) {}

// example
async fn get_user(Path(user_id) : Path<u64>) -> String {
    format!("User ID: {}", user_id);  
    return user_id;
}
async fn get_user_team(Path((user_id, team_id)): Path<(u32, u64)>) -> String {
    format!("User ID: {}, Team ID: {}", user_id, team_id)
}



// same as const [ slug ] = useParams(); in next,js
async fn query(Query(params): Query<HashMap<String, String>>) {}

// EXAMPLE to use it 

async fn search(Query(params): Query<HashMap<String, String>>) -> String {
    let name = params.get("name").unwrap_or(&"Guest".to_string());
    format!("Hello, {}!", name)
}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports anyt ype that implements
// `serde::Deserialize`.
async fn json(Json(payload): Json<serde_json::Value>) {}


// example 

#[derive(Debug ,Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}
async fn create_user(Json(payload): Json<CreateUser>) -> String {
    format!("Created user: {} with email: {}", payload.username, payload.email)
}


async fn router_hello () -> Router {
    Router.new()
        .route("/hello", get())
        .route("/hello2", get())

}
async fn hello ()-> String {
    return String("hellow");
}


// define module
// basically in typescript every file is a module but in rust we have to define it.
// it contains function and structs and etc.
pub mod hello {
    pub fn say_hello() -> String {
        return String("hello");
    }
}
