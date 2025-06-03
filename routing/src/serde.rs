// so serde is the pacakge that use to get content (de)serialization

// like Serialization and Deserialization


// define it 

// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

// where serde is for JSON, TOML, YAML, etc.

// serde_json is for JSON

// use case

// define a struct

use serde::{Serialize, Deserialize};
use axum::{Router, routing::post, routing::get, Json};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

//  a exmaple static database
static mut USERS: Vec<User> = Vec::new();

// let user = User {
//     id: 1,
//     name: "John".to_string(),
//     email: "john@example.com".to_string(),
// };


async fn create_user(Json(user): Json<User>) -> Json<&'static str> {
 unsafe {
    USERS.push(user);
 }
 Json("User added successfully")
}

async fn get_users() -> Json<Vec<User>> {
    unsafe {
        Json(USERS.clone())
    }
}

#[tokio::main]
async fn main() {
    let app: Router = Rouer::new()
            .route("/users", post(create_user) )
            .route("/users", get(get_users));


        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("Listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
}

// Deserialize into Enums and Nested Types
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Notification {
    Email { address: String },
    SMS { number: String },
}

// JOSN
// { "type": "Email", "address": "abc@example.com" }
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    user: User,
    notifications: Vec<Notification>,
}



// now only left serde using cli tools. 





