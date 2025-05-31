use axum::{
    routing::{get, post},
    Router,
    extract::{Path, Query, Json}
};

#[tokio::main]
async fn main(){
let app = Router::new()
    .route("/" , get(root) )
    .route("/public", get(get_public_key).post(post_public))
    .route("/private/secure", get(get_private));
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

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
async fn json(Json(payload): Json<serde_json::Value>) {}
