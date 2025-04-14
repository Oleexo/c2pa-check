#!/bin/bash

curl -X POST http://localhost:8080/check -v \
  -F "file=@/home/maxime/dev/perso/c2pa-check/scripts/image.jpg" \
  -H "Content-Type: multipart/form-data"
