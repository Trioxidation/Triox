## Source code
Path: `/source`  
The source code of Triox is available to download at this location.


## Authorization
The authorization process requires the client to send a JWT (JSON Web Token) to the server. This can be done in two ways: Either by using the authorization header or by using a cookie. The authorization header is the preferred authorization method, the cookie header exist primarily for comparability with browsers and some HTTP-client implementations.

### Authorization header
Name: `authorization`  
Type: `Bearer`  
Value: Encoded JWT (see Auth -> Sign in)  
Example:  
`authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJodHRwX3Rlc3RlciIsImlkIjoxOTEzMjkyNzI5LCJyb2xlIjowLCJleHAiOjE2MDE0NzgwNTR9.fXdzLaq_UwpwJ6BCqCA7lfPWuBw0Cfi2f485Ptr9t5g`

### Cookie header
Name: `cookie`  
Cookie-name: `triox_jwt`  
Value: Encoded JWT (see Auth -> Sign in)  
Example:  
`cookie:  triox_jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJodHRwX3Rlc3RlciIsImlkIjoxOTEzMjkyNzI5LCJyb2xlIjowLCJleHAiOjE2MDE0NzgwNTR9.fXdzLaq_UwpwJ6BCqCA7lfPWuBw0Cfi2f485Ptr9t5g`

## Auth
### Sign in
Path: `/sign_in`  
Method: POST  
Auth: None  
Body: JSON
```json
{
  "username": "test_user",
  "password": "test_password",
  "cookie": false
}
```

Success Response:
 
+ "cookie": true -> JWT inside set-cookie header
+ otherwise -> JWT as text/plain

> Note: The "cookie" attribute is optional and can be omitted (default value is false).

### Sign up
Path: `/sign_up`  
Method: POST  
Auth: None  
Body: JSON
```json
{
  "username": "test_user",
  "password": "test_password",
  "email": "test@triox.com"
}
```

Success Response: "user created" as text/plain

### Delete User
Path: `/delete_user`  
Method: POST  
Auth: None  
Body: JSON
```json
{
  "username": "test_user",
  "password": "test_password",
}
```

Success Response: "user successfully deleted" as text/plain

### User information
Path: `/user_info`  
Method: GET  
Auth: JWT  

Success Response: JSON
```json
{
  "sub":"test_user",
  "id":1513292729,
  "role":0,
  "exp":1601478004
}
```

# Apps

## Files
### Get (Download)
Path: `/app/files/get?path=path/to/file`  
Method: GET  
Auth: JWT  

Success Response: File

### List
Path: `/app/files/list?path=path/to/file`  
Method: GET  
Auth: JWT  

Success Response: JSON
```json
{
  "files": [
    {
      "name": "file.zip",
      "size": 7206445,
      "last_modified": 1606818956
    },
    {
      "name": "main.rs",
      "size": 9334,
      "last_modified": 1604945280
    },
  ],
  "directories": [
    {
      "name": "test_folder",
      "last_modified": 1606490838
    },
    {
      "name": "src",
      "last_modified": 1606592435
    },
  ]
}
```

+ last_modified stores a date in unix time (seconds since 00:00:00 UTC on 1 January 1970)
+ size stores the size of files in bytes

### Upload
Path: `/app/files/upload?path=path/to/file`  
Method: POST  
Auth: JWT  
Body: multipart data

Success Response: "upload finished!" as text/plain

### Create directory
Path: `/app/files/create_dir?path=path/to/file`  
Method: GET  
Auth: JWT  

Success Response: "directory successfully created!" as text/plain

### Remove
Path: `/app/files/remove?path=path/to/file`  
Method: GET  
Auth: JWT  

Success Response:  "directory successfully deleted!" or "file successfully deleted!" as text/plain

### Copy
Path: `/app/files/copy`  
Method: POST  
Auth: JWT  
Body: JSON
```json
{
    "from": "path/to/source",
    "to": "path/to/destination"
}
```

Success Response:  "directory successfully copied!" or "file successfully copied!" as text/plain

### Move
Path: `/app/files/move`  
Method: POST  
Auth: JWT  
Body: JSON
```json
{
    "from": "path/to/source",
    "to": "path/to/destination"
}
```

Success Response:  "directory successfully moved!" or "file successfully moved!" as text/plain
