# acme-rs
An `ACME` Client for Let's Encrypt written in Rust to request SSL/TLS certificates. This client follows the guidelines specified in `[RFC8555](https://tools.ietf.org/html/rfc8555)`.

## Contents
 - [Features](#Features)
 - [Installation](#Installation)
 - [Usage](#Usage)
 - [Options](#Options)

## Features
- `acme-rs` in its current state does only support the http challenge. The port 80 must not be blocked. <br>
- You have the option to generate you keypair for the certificate first before executing the client. <br>
- By default, acme-rs will send the request to the URL https://acme-v02.api.letsencrypt.org/directory. However, you can manually change the ACME Server URL by using the "--server" flag.

## Installation

The installation process is done via `crates.io`. To install the lates version of this tool just run

```bash
cargo install acme-rs
```

## Usage
`acme-rs` is using the `openssl` rust wrapper crate to generate keys and the csr.

The client will store the certificate and the certificate chain in the files "cert.crt" and "chain.crt"

### Request a certificate
You can request a certificate by using the following command: <br>
```bash
acme-rs [OPTIONS] --domain <domain>
```


## Options
By running the command "acme-rs --help" you can get an overview of all the commands available.

```bash
USAGE:
    acme-rs [FLAGS] [OPTIONS] --domain <domain>

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Enables debug output
    -V, --version    Prints version information

OPTIONS:
    -d, --domain <domain>              The domain to register the certificate for
        --private-key <private-key>    An optional private key file (PEM format) to load the keys
                                       from
        --public-key <public-key>
    -s, --server <server>              The ACME server's URL
```
