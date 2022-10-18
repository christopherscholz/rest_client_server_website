use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};

#[test]
fn json_get() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // Try to get a page with a name, which doesn't exist.
    let res = client.get("/page/not-available").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.into_string().unwrap();
    assert!(body.contains("error"));
    assert!(body.contains("Resource was not found."));

    // Try to get a page with a name, which does exist.
    let res = client.get("/page/home").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::Ok);
}
