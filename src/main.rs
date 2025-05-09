#[macro_use] extern crate rocket;

use std::io::{Cursor};
use c2pa::Reader;
use prometheus::process_collector::ProcessCollector;
use rocket::Config;
use rocket::data::{Limits, ToByteUnit};
use rocket::http::{Status, ContentType};
use rocket::fs::TempFile;
use rocket::form::Form;
use tokio::io::AsyncReadExt;
use rocket_prometheus::{PrometheusMetrics};


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
    debug!("Starting file processing");

    // Step 2: Open the TempFile for reading
    let file = match form.file.open().await {
        Ok(file) => file,
        Err(_e) => return (Status::BadRequest, (ContentType::JSON, "{ \"error\": \"Failed to process the file\" }".to_string())),
    };
    // Step 3: Read the TempFile's contents into the buffer
    let mut reader = tokio::io::BufReader::new(file);
    match reader.read_to_end(&mut buffer).await {
        Ok(_) => debug!("Successfully read file contents"),
        Err(_e) => return (Status::BadRequest, (ContentType::JSON, "{ \"error\": \"Failed to read the file contents\" }".to_string())),
    }

    let content_type : &ContentType;
    if let Some(ct) = form.file.content_type() {
        content_type = ct;
    } else {
        return (Status::BadRequest, (ContentType::JSON, "{ \"message\": \"no content-type found\" }".to_string()));
    }

    let stream = Cursor::new(buffer);
    debug!("Starting C2PA validation");
    // Create the reader from the stream
    let reader = match Reader::from_stream(content_type.to_string().as_str(), stream) {
        Ok(reader) => reader,
        Err(e) => return (Status::BadRequest, (ContentType::JSON, format!("{{ \"message\": \"{}\" }}", e.to_string()).to_string())),
    };

    let json_response = reader.json();
    (Status::Ok, (ContentType::JSON, json_response))
}

#[get("/ready")]
fn health() -> &'static str {
    "OK"
}

#[get("/live")]
fn live() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    debug!("Starting server");


    let file_size_limit = std::env::var("FILE_SIZE_LIMIT_MB")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(10);

    let limits = Limits::new()
        .limit("file", file_size_limit.mebibytes())
        .limit("data-form", file_size_limit.mebibytes());

    let config = Config {
        port: 8080,
        address: "0.0.0.0".parse().unwrap(),
        log_level: rocket::config::LogLevel::Debug,
        limits,
        ..Default::default()
    };

    let process_collector = ProcessCollector::for_self();
    
    let prometheus = PrometheusMetrics::new();
    prometheus.registry()
        .register(Box::new(process_collector))
        .unwrap();

    rocket::custom(config)
        .attach(prometheus.clone())
        .mount("/", routes![index, check])
        .mount("/healthz", routes![health, live])
        .mount("/metrics", prometheus)
}
