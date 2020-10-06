# Triox - a cloud server for the next generation

## Why Triox?

☘️ **Open source** - We strongly believe in collaboration and transparency.

⚡ **Speed** - Get the most out of your hardware! Triox runs fast, even on less powerful setups.

🔒 **Security** - We're using the Argon2 algorithm to protect your passwords.

⛓️ **Reliability** - Built on top of the strong guarantees of the [Rust programming language](https://rust-lang.org).

🛫 **Easy Setup** - Triox comes with batteries included and is easy to configure.

🔬 **Modern Technologies** - No cookies but authentication with [JWT](https://jwt.io) and a front-end based on [WebAssembly](https://webassembly.org).

## Freatures

Currently we want to implement the following features before moving on with more ambitious plans:

- [x] JWT basic authentication
- [x] File up- and download
- [ ] WebAssebly based front-end

# Setup

+ Install Rust using [rustup](https://rustup.rs).
+ Install a MySQL-server such as mariadb (`sudo apt install mariadb-server`)
+ Setup database (more below)
+ [optional] setup SSL certificate for HTTPS

Now you should be ready to go! Use `cargo run` to compile and start the server.

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


# API Dokumentation

The API is documented in [`API.md`](https://github.com/AaronErhardt/Triox/blob/master/API.md).