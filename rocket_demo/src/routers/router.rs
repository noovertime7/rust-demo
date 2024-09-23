use crate::controller;

pub fn Init_Route(engine: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    engine.mount("/api", routes![controller::user::UserName])
}