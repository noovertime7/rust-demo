use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
pub  struct Message {
    name: String,
    msg: String,
}

#[get("/user/<name>")]
pub fn UserName(name:String) -> Json<Message> {
    let message = Message {
        name,
        msg: "hello".to_string(),
    };
    Json(message)
}
