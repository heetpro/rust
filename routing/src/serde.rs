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




async fn main() {
   
}




