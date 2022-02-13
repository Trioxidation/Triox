<div align="center">
<img width="130px" style="border-radius: 100px" alt="Triox logo" src="./docs/assets/logo.svg" />
  <h1>Triox</h1>
  <p>
    <strong>
		Next Generation cloud storage server that is secure, fast, and reliable.
	</strong>
  </p>

[![Build](https://github.com/Trioxidation/Triox/actions/workflows/rust.yml/badge.svg)](https://github.com/Trioxidation/Triox/actions/workflows/rust.yml)
![issues](https://img.shields.io/github/issues/Trioxidation/Triox?style=flat-square)
[![dependency status](https://deps.rs/repo/github/Trioxidation/Triox/status.svg)](https://deps.rs/repo/github/Trioxidation/Triox)
[![codecov](https://codecov.io/gh/Trioxidation/Triox/branch/master/graph/badge.svg?style=flat-square)](https://codecov.io/gh/Trioxidation/Triox)
<br />
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg?style=flat-square)](http://www.gnu.org/licenses/agpl-3.0)
[![Chat](https://img.shields.io/badge/matrix-+triox:matrix.org-purple?style=flat-square)](https://matrix.to/#/+triox:matrix.org)

</div>

## NOTE: Currently, Triox is not actively developed due to limited resources of the dev team.

---

## Why Triox?

☘️ **Open Source** - We strongly believe in collaboration and
transparency.

⚡ **Speed** - Get the most out of your hardware! Triox runs fast, even
on weak hardware.

🔒 **Security** - We're using state-of-the-art algorithms and
authentication methods to protect your data.

⛓️ **Reliability** - Built on top of the strong guarantees of the [Rust
programming language](https://rust-lang.org).

🛫 **Easy Setup** - Triox comes with batteries included and is easy to
configure.

## Features

Triox is still in an early stage but is already usable. The features
we'd like to add before our first release can be found in [this
issue](https://github.com/Trioxidation/Triox/issues/17).

## Demo

### Hosted server

~~Sign in with username `demo_user` and password `demo_password`.~~

~~Sadly, we can't allow users to upload files due to legal restrictions.
Since we can't guarantee that no illegal data will be uploaded the demo
server runs in read-only mode.~~

~~triox-demo.aaron-erhardt.de~~

### Self-hosted:

1. Clone the repository

```bash
git clone https://github.com/Trioxidation/triox && cd triox
```

2. Build and start Triox

```bash
docker-compose up -d --build
```

Triox should be accessible at http://localhost:3000

## Contributing

Everyone is welcome to contribute to Triox. We are always open for new
ideas, features and improvements.

The easiest way to contribute changes is to fork Triox, and then create
a pull request to ask us to pull your changes into our repository. You
can find a list of good first issues
[here](https://github.com/Trioxidation/Triox/labels/good%20first%20issue).

## Setup

See [DEPLOYMENT.md](./docs/DEPLOYMENT.md) for instructions

The API is documented in [`API.md`](./API.md).
