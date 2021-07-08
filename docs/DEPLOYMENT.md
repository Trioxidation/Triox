# Deployment Instructions

There are three ways to deploy Triox:

1. Docker
2. Docker compose
3. Bare metal

## Docker

NOTE: We'll publish pre-built images once we reach `alpha`.

1. Build image:

```bash
 cd Triox && docker build -t triox/triox:latest .
```

2. Set configuration in [configuration file](../config/local.toml)

3. Run image:

If you have already have a Postgres instance running, then:

```bash
docker run -p <host-machine-port>:<port-in-configuration-file> \
	--add-host=database:<database-ip-addrss> \
	-e RUST_LOG=debug \
	-e DATABASE_URL="postgres://<db-user>:<db-password>@database:<db-port>/<db-name>" \
	triox/triox:latest
```

If you don't have a Postgres instance running, you can either install
one using a package manager or launch one with docker. A [docker-compose
configuration]('../docker-compose.yml) is available that will launch both
a database instance Triox instance.

## docker-compose

1. Set database password [docker-compose configuration]('../docker-compose.yml).

2. Launch network:

```bash
 docker-compose up -d --build
```

## Bare metal:

The process is tedious, most of this will be automated with a script in
the future.

### 1. Install postgres

For Debian based systems:

```bash
sudo apt install postgresql
```

### 2. Create new user for running `Triox`:

```bash
 sudo useradd -b /srv -m -s /usr/bin/bash triox
```

### 3. Create new user in Postgres

```bash
 sudo -iu postgres # switch to `postgres` user
 psql
```

Postgres shell:

```psql
 postgres=#  CREATE USER triox WITH PASSWORD 'my super long password and yes you need single quotes';
 postgres=#  exit;
```

```bash
createdb -O triox triox # create db 'triox' with 'triox' as owner
```

### 3. Build `Triox`:

To build `Triox`, you need the following dependencies:

1. rust

## Build instructions

### 1. Install Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build Triox

```bash
cargo build --release
```

### 3. Setup TLS

- Generating TLS key and certificate

```bash
cd ssl
openssl req -x509 -nodes -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365
cd ..
```

- Then update `config/local.toml`:

```toml
[ssl] enabled = true
```

### 4. Install package:

```bash
 sudo mkdir /srv/triox/triox/
 sudo cp ./target/release/triox /srv/triox/triox # Copy binary
 sudo cp -r ./config /srv/triox/triox # copy configurations
 sudo cp -r ./static /srv/triox/triox # copy static files
 sudo cp -r ./tls /srv/triox/triox #  copy custom TLS certs
 sudo chown -R triox:triox /srv/triox/triox # change ownership of all copied files to user triox
```

### 5. Systemd service configuration:

1. Copy the following to `/etc/systemd/system/triox.service`:

```systemd
[Unit]
Description=Triox: Next Generation cloud storage server that is secure, fast, and reliable.

[Service]
Type=simple
User=triox
ExecStart=/srv/triox/triox
Restart=on-failure
RestartSec=1
SuccessExitStatus=3 4
RestartForceExitStatus=3 4
SystemCallArchitectures=native
MemoryDenyWriteExecute=true
NoNewPrivileges=true
Environment="RUST_LOG=info"
# set a long, random value for TRIOX_SERVER_SECRET
Environment="TRIOX_SERVER_SECRET="

[Unit]
After=sound.target
Wants=network-online.target
Wants=network-online.target
Requires=postgresql.service
After=syslog.target

[Install]
WantedBy=multi-user.target
```

2. Enable service:

```bash
 sudo systemctl daemon-reload && \
	sudo systemctl enable triox && \ # Auto startup during boot
	sudo systemctl start triox
```
