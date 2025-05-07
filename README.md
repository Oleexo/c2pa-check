A lightweight Rust API service to verify and extract C2PA manifests from digital content. This project leverages the [C2PA Rust SDK](https://opensource.contentauthenticity.org/docs/rust-sdk/) to retrieve and validate Content Credentials (formerly known as Content Authenticity Initiative metadata) from images and other [supported file formats](https://opensource.contentauthenticity.org/docs/rust-sdk/docs/supported-formats/).
## About C2PA
The Coalition for Content Provenance and Authenticity (C2PA) is an open technical standard that provides publishers, creators, and consumers with opt-in, flexible ways to understand the authenticity of media content.
## Features
- REST API endpoint for manifest verification
- Support for multiple file formats (images, videos, etc.)
- JSON response with full manifest details
- Containerized for easy deployment
- Health/liveness checks for container orchestration

## Prerequisites
- Docker and Docker Compose
- Curl (for testing)

## Installation
### Option 1: Pull the Docker image directly
Pull the pre-built Docker image:
``` bash
docker pull oleexo/c2pa-check
```

### Option 2: Clone and build from source
Clone this repository:

``` bash
git clone https://github.com/yourusername/c2pa-check.git
cd c2pa-check
```
## Usage
### Running the Service
Start the service using Docker Compose:
``` bash
docker compose up
```
The service will be available at `http://localhost:8080`.
### API Endpoints
#### Health Check
``` 
GET /healthz/ready
GET /healthz/live
```
Returns "OK" when the service is running correctly.
#### Homepage
``` 
GET /
```
Returns a simple welcome message.
#### C2PA Verification
``` 
POST /check
```
Submit a file via multipart form data to check for C2PA manifests.
**Parameters:**
- `file`: The file to be verified (multipart/form-data)

**Response:**
- JSON object containing the C2PA manifest data or an error message

### Example
To test with a sample image:
``` bash
curl -X POST http://localhost:8080/check -v \
  -F "file=@/path/to/your/image.jpg" \
  -H "Content-Type: multipart/form-data"
```
## Development
This project is built with:
- Rust 1.86.0
- Rocket 0.5.1 (web framework)
- c2pa 0.49.2 (Content Credentials SDK)
- Tokio 1.44.2 (async runtime)

To build locally:
``` bash
cargo build
```
To run without Docker:
``` bash
cargo run
```