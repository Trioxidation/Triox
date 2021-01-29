![CI (Linux)](<https://github.com/AaronErhardt/Triox/workflows/CI%20(Linux)/badge.svg>)
![issues](https://img.shields.io/github/issues/AaronErhardt/Triox?style=flat-square)
[![dependency status](https://deps.rs/repo/github/AaronErhardt/Triox/status.svg)](https://deps.rs/repo/github/AaronErhardt/Triox)


# Triox - a cloud server for the next generation

**A free file hosting server that focuses on speed, reliability and security.**

## Why Triox?

☘️ **Open Source** - We strongly believe in collaboration and transparency.

⚡ **Speed** - Get the most out of your hardware! Triox runs fast, even on weak hardware.

🔒 **Security** - We're using state-of-the-art algorithms and authentication methods to protect your data.

⛓️ **Reliability** - Built on top of the strong guarantees of the [Rust programming language](https://rust-lang.org).

🛫 **Easy Setup** - Triox comes with batteries included and is easy to configure.

🔬 **Modern Technologies** - Authentication with [JWT](https://jwt.io) and a front-end based on [WebAssembly](https://webassembly.org).

## Features

Triox is still in an early stage but is already usable. The features we'd like to add before our first release can be found in [this issue](https://github.com/AaronErhardt/Triox/issues/17).

## Demo servers

Sign in with username `demo_user` and password `demo_password`.

Sadly, we can't allow users to upload files due to legal restrictions. Since we can't guarantee that no illegal data will be uploaded the demo servers run in read-only mode.

- US server: [triox-demo.aaron-erhardt.de](https://triox-demo.aaron-erhardt.de)
- EU server: [triox-demo-eu.aaron-erhardt.de](https://triox-demo-eu.aaron-erhardt.de)

## Contributing

Everyone is welcome to contribute to Triox. We are always open for new ideas, features and improvements.

The easiest way to contribute changes is to fork Triox, and then create a pull request to ask us to pull your changes into our repository. You can find a list of good first issues [here](https://github.com/aaronerhardt/triox/labels/good%20first%20issue).

# Setup

+ Install Rust using [rustup](https://rustup.rs).
+ Install dependencies:
  - pkg-config, common package name: `pkg-config`
  - OpenSSL, common package name: `libssl-dev` or `openssl-devel`
  - MySQL-client, common package name: `libmysqlclient-dev`, `libmariadb-dev-compat` or `mysql-devel`
+ Install a MySQL-server such as mariadb, common package name: `mariadb-server`
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

### Run migrations

```bash
diesel migration run
```


## SSL setup

### Generating SSL key and certificate

```bash
cd ssl
openssl req -x509 -nodes -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365
cd ..
```

Then update `config/local.toml`:
```toml
[ssl]
enabled = true
```
# API Documentation

The API is documented in [`API.md`](https://github.com/AaronErhardt/Triox/blob/master/API.md).
