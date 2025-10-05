#!/bin/bash

# Define expected response
expected_response=$(cat "./scripts/C.jpg.manifest.json")

# Execute curl and capture both status code and response
response=$(curl -s -w "\n%{http_code}" -X POST http://localhost:8080/check \
  -F "file=@./scripts/C.jpg" \
  -H "Content-Type: multipart/form-data")

# Extract status code (last line)
status_code=$(echo "$response" | tail -n1)
# Extract response body (everything except the last line)
response_body=$(echo "$response" | sed '$d')

# Print status code and response for debugging
echo "Status code: $status_code"
#echo "Response: $response_body"

# Check status code
if [ "$status_code" -ne 200 ]; then
  echo "Error: Expected status code 200, got $status_code"
  exit 1
fi

# Compare JSON response (requires jq)
if ! command -v jq &> /dev/null; then
  echo "Error: jq is required but not installed. Please install jq first."
  exit 1
fi

# Normalize and compare JSON
if [ "$(echo "$response_body" | jq -c .)" = "$(echo "$expected_response" | jq -c .)" ]; then
  echo "Success: Response matches expected value"
  exit 0
else
  echo "Error: Response does not match expected value"
  echo "Expected: $expected_response"
  echo "Actual: $response_body"
  exit 1
fi