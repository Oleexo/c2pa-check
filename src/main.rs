#[macro_use] extern crate rocket;

use std::io::{Cursor};
use c2pa::Reader;
use rocket::Config;
use rocket::http::{Status, ContentType};
use rocket::fs::TempFile;
use rocket::form::Form;
use tokio::io::AsyncReadExt;


#[derive(FromForm)]
struct FileUpload<'r> {
    file: TempFile<'r>,
}

pub struct CawgValidator;

#[get("/")]
fn index() -> &'static str {
    "Hello, I'm C2PA check !"
}

#[post("/check", data = "<form>")]
async fn check(form: Form<FileUpload<'_>>) -> (Status, (ContentType, String)) {

    // Step 1: Create a buffer to hold the file's contents
    let mut buffer = Vec::new();

    // Step 2: Open the TempFile for reading
    let file = match form.file.open().await {
        Ok(file) => file,
        Err(_e) => return (Status::BadRequest, (ContentType::JSON, "{ \"error\": \"Failed to process the file\" }".to_string())),
    };
    // Step 3: Read the TempFile's contents into the buffer
    let mut reader = tokio::io::BufReader::new(file);
    match reader.read_to_end(&mut buffer).await {
        Ok(_) => (),
        Err(_e) => return (Status::BadRequest, (ContentType::JSON, "{ \"error\": \"Failed to read the file contents\" }".to_string())),
    }

    let content_type : &ContentType;
    if let Some(ct) = form.file.content_type() {
        content_type = ct;
    } else {
        return (Status::BadRequest, (ContentType::JSON, "{ \"message\": \"no content-type found\" }".to_string()));
    }

    let stream = Cursor::new(buffer);
    // Create the reader from the stream
    let reader = match Reader::from_stream(content_type.to_string().as_str(), stream) {
        Ok(reader) => reader,
        Err(e) => return (Status::BadRequest, (ContentType::JSON, format!("{{ \"message\": \"{}\" }}", e.to_string()).to_string())),
    };

    let json_response = reader.json();
    (Status::Ok, (ContentType::JSON, json_response))
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[get("/live")]
fn live() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        port: 8080,
        address: "0.0.0.0".parse().unwrap(),
        ..Default::default()
    };
    rocket::custom(config).mount("/", routes![index, check])
        .mount("/live", routes![health, live])
}
