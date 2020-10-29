#!/bin/sh

ADDR="http://127.0.0.1:8080"
DEFAULT_HEADERS="Content-Type: application/json"
USER_NAME="http_tester"
PASSWORD="http_tester"
EMAIL="tester@http.com"


# Try signing up a new user
SIGN_UP=`curl -H "$DEFAULT_HEADERS" \
  -X POST \
  -d "{\"user_name\":\"$USER_NAME\",\"password\":\"$PASSWORD\", \"email\": \"$EMAIL\"}" \
  -i -s \
  "$ADDR/sign_up"`

echo "SIGN UP:"
echo "$SIGN_UP"

echo
echo " --- "
echo


# Try signing the user in
SIGN_IN=`curl -H "$DEFAULT_HEADERS" \
  -X POST \
  -d "{\"user_name\":\"$USER_NAME\",\"password\":\"$PASSWORD\"}" \
  -s \
  "$ADDR/sign_in"`

echo "SIGN IN:"
echo "$SIGN_IN"

[ -z "$SIGN_IN" ] && echo "couldn't log in!" && exit 0

JWT="$SIGN_IN"

echo
echo " --- "
echo


# Request user information
USER_INFO=`curl -H "$DEFAULT_HEADERS" \
  -H "Triox-JWT: $JWT" \
  -X GET \
  -i -s \
  "$ADDR/user_info"`

echo "USER INFO:"
echo "$USER_INFO"

echo
