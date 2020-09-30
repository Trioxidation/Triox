# About
Triox is an open source cloud server that focuses on speed, reliability and security.

Currently we want to implement the following features before moving on with more ambitious plans:

- [x] JWT authentication
- [ ] File up- and download
- [ ] Frontend

# Rest API

See `API.md`

# Dev Setup

+ Clone this repository
+ Install Rust using [rustup](https://rustup.rs).
+ Install a MySQL-server such as mariadb (`sudo apt install mariadb-server`)
+ Setup database (more below)
+ [optional] setup SSL certificate for HTTPS
+ Compile with `cargo run`

## Database setup

### Creating database user

```sql
CREATE DATABASE triox;
CREATE USER 'triox'@localhost IDENTIFIED BY 'password';
GRANT ALL PRIVILEGES ON triox.* TO 'triox'@localhost;
FLUSH PRIVILEGES;
```

### Install diesel client

```bash
cargo install diesel_cli --no-default-features --features mysql
```

### Add .env for diesel client

```bash
echo DATABASE_URL=mysql://triox:password@localhost/triox > .env
```

### Generate and run migrations

```bash
diesel setup
diesel migration generate users
diesel migration run
```


## SSL setup

### Generating SSL key and certificate for HTTPS

```bash
cd ssl
openssl req -x509 -nodes -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365
cd ..
```



