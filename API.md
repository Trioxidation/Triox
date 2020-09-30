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
  "password": "test_password"
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