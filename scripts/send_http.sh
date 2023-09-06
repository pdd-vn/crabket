#!/bin/bash

curl -X POST \
     -H 'Accept: text/plain' \
     -H 'Content-Type: text/plain' \
     -d 'ping' "http://127.0.0.1:8686/foo"