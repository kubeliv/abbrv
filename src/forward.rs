#[macro_use]
extern crate rocket;

use std::env;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket::State;

struct AbbrvForwardConfig {
    url: String,
}

#[get("/<key>")]
async fn link(key: &str, config: &State<AbbrvForwardConfig>) -> Result<Redirect, NotFound<String>> {
    return Ok(Redirect::temporary(config.url.to_string() + "/" + key));
}

#[launch]
fn rocket() -> _ {
    let url = env::var("ABBRV_URL").expect("Missing env ABBRV_URL");
    let config = AbbrvForwardConfig {
        url: url
    };

    rocket::build().manage({
        config
    }).mount("/", routes![link])
}
