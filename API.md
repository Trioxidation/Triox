## Headers
### JWT Auth
Name: `Triox-JWT`  
Value: JWT (see Auth -> Sign in)  
Example:  
`Triox-JWT: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJodHRwX3Rlc3RlciIsImlkIjoxOTEzMjkyNzI5LCJyb2xlIjowLCJleHAiOjE2MDE0NzgwNTR9.fXdzLaq_UwpwJ6BCqCA7lfPWuBw0Cfi2f485Ptr9t5g`

## Auth
### Sign in
Path: `/sign_in`  
Method: POST  
Auth: None  
Body: JSON
```json
{
  "user_name": "test_user",
  "password": "test_password"
}
```

Success Response: JWT as text/plain

### Sign up
Path: `/sign_up`  
Method: POST  
Auth: None  
Body: JSON
```json
{
  "user_name": "test_user",
  "password": "test_password"
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

### Up (Upload)
Path: `/app/files/up/{path}`  
Method: POST  
Auth: JWT  
Body: multipart data

Success Response: "upload finished" as text/plain