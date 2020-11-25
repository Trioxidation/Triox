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
  "user_name": "test_user",
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
  "user_name": "test_user",
  "password": "test_password",
  "email": "test@triox.com"
}
```

Success Response: "user created" as text/plain

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
Path: `/app/files/get/{path}`  
Method: GET  
Auth: JWT  

Success Response: File

### List
Path: `/app/files/list/{path}`  
Method: GET  
Auth: JWT  

Success Response: JSON
```json
{
  "files": ["file1", "file2"],
  "dirs": ["dir1", "dir2"]
}
```

### Upload
Path: `/app/files/upload/{path}`  
Method: POST  
Auth: JWT  
Body: multipart data

Success Response: "upload finished!" as text/plain

### Create directory
Path: `/app/files/create_dir/{path}`  
Method: GET  
Auth: JWT  

Success Response: "directory successfully created!" as text/plain

### Remove
Path: `/app/files/remove/{path}`  
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
    from: "path/to/source",
    to: "path/to/destination"
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
    from: "path/to/source",
    to: "path/to/destination"
}
```

Success Response:  "directory successfully moved!" or "file successfully moved!" as text/plain
