#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::*;
use rocket_contrib::json::JsonValue;

#[get("/")]
fn hello() -> JsonValue {
    json!({
        "hello": "world",
        "awesomesauce": "stuff"
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};
    use rocket_contrib::*;

    #[test]
    fn json() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/").dispatch();
        let json = json!({
            "hello": "world",
            "awesomesauce": "stuff"
        });
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(json.to_string()));
        assert_eq!(response.content_type(), Some(ContentType::JSON))
    }
}
