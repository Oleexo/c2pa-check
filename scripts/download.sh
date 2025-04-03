#!/bin/bash

curl -s https://api.github.com/repos/contentauth/c2pa-rs/releases/latest \
| grep "c2patool-v.*tar.gz" \
| cut -d : -f 2,3 \
| tr -d \" \
| wget -v -O c2patool.tar.gz -i -
tar -xvzf c2patool.tar.gz