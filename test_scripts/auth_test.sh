#!/bin/sh

ADDR="http://127.0.0.1:8080"
DEFAULT_HEADERS="Content-Type: application/json"

curl -H "$DEFAULT_HEADERS" \
  -X POST \
  -d '{"user_name":"http_tester","password":"http_tester", "email": "tester@http.com"}' \
  -i -s \
  "$ADDR/sign_up"

echo
echo

JWT=`curl -H "$DEFAULT_HEADERS" \
  -X POST \
  -d '{"user_name":"http_tester","password":"http_tester"}' \
  -s \
  "$ADDR/sign_in"`

[ -z "$JWT" ] && echo "couldn't log in!" && exit 0

echo "JWT: $JWT"
echo

curl -H "$DEFAULT_HEADERS" \
  -H "Triox-JWT: $JWT" \
  -X GET \
  -i -s \
  "$ADDR/user_info"

 echo
