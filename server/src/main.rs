#[macro_use] extern crate rocket;

use std::time::{SystemTime, UNIX_EPOCH};

use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::uuid::Uuid;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Page {
    time: u128,
    blocks: Vec<Block>,
    version: String,
}

impl Page {
    fn new(blocks: Vec<Block>) -> Page {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        Page {
            time: timestamp,
            blocks: blocks,
            version: "0.1.0".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Block {
    id: Uuid,
    #[serde(flatten)]
    data: Data
}

impl Block {
    fn new(data: Data) -> Block {
        Block {
            id: Uuid::new_v4(),
            data: data,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type", content = "data")]
enum Data {
    #[serde(rename = "paragraph")]
    Paragraph {
        text: String
    },
    #[serde(rename = "header")]
    Header {
        text: String,
        level: u8
    },
    #[serde(rename = "list")]
    List {
        style: String,
        items: Vec<Item>
    }
}

type Item = String;

#[get("/page/<page>", format = "json")]
fn get(page: String) -> Option<Json<Page>> {
    match page.as_str() {
        "home" => Some(Json(Page::new(vec![
            Block::new(Data::Header { text: "Data Engineer, Problem Solver".to_owned(), level: 2 }),
            Block::new(Data::Paragraph { text: "<span class=\"highlight\">Data and processes accompany me through my entire professional life. As an expert in data and processes, especially in supply chain management, production and their interfaces, who speaks both the technical and the business language and can interpret in between, I contribute strongly to the understanding and better communication of problems.</span>".to_owned() })
        ]))),
        "impressum" => Some(Json(Page::new(vec![
            Block::new(Data::Header { text: "Angaben gemäß §5 TMG".to_owned(), level: 2 }),
            Block::new(Data::Paragraph { text: "Christopher Scholz<br>An der Dahme 3<br>12527 Berlin".to_owned() }),
            Block::new(Data::Header { text: "Kontakt".to_owned(), level: 2 }),
            Block::new(Data::Paragraph { text: "Email: <a href=\"mailto:website@christopher-scholz.com\">website@christopher-scholz.com</a>".to_owned() })
        ]))),
        _ => None
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[cfg(test)] mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get])
        .register("/", catchers![not_found])
        .attach(CORS)
        .mount("/", routes![all_options])
}