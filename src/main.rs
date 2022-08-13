#[macro_use]
extern crate rocket;

use std::env;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket::State;
use serde_json::Value;

struct AbbrvConfig {
    manifest: String,
}

#[get("/<key>")]
async fn link(key: &str, config: &State<AbbrvConfig>) -> Result<Redirect, NotFound<String>> {
    let resp = reqwest::get(config.manifest.to_string())
        .await
        .expect("Failed to fetch manifest")
        .json::<Value>()
        .await
        .expect("Failed to deserialize manifest");

    // TODO: Is there a better way to do this?
    let obj_maybe = resp.get(key);
    if obj_maybe.is_none() {
        return Err(NotFound(format!("Link {} was not found", key)));
    }
    let obj = obj_maybe.unwrap();
    let typ = obj.get("type").expect("Missing type").as_str().expect("type is wrong type");
    let url = match typ {
        "static" => obj.get("url").expect("Missing url field").as_str().expect("url is wrong type"),
        _ => panic!("Unsupported type: {}", typ),
    };

    return Ok(Redirect::temporary(url.to_string()))
}

#[launch]
fn rocket() -> _ {
    let m = env::var("ABBRV_MANIFEST").expect("Missing env ABBRV_MANIFEST");
    let config = AbbrvConfig {
        manifest: m
    };

    rocket::build().manage({
        config
    }).mount("/", routes![link])
}
