#!/bin/bash

curl -X POST http://localhost:8080/check \
  -F "file=@/home/maxime/dev/perso/c2pa-check/scripts/C.jpg" \
  -H "Content-Type: multipart/form-data"
