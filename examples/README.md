# Exmaples of using cross-krb5

These examples show how to use `cross_krb5`.

Inside a checkout of this repository, you can run an exmaple with the following
command:

```bash
carg run --examples EXAMPLE_NAME
```

On macOS you might want to disable the `iov` feature like this:

```bash
carg run --examples EXAMPLE_NAME --no-default-features
```

## Dependencies

The examples assume the following dependencies:

```toml
[dependencies]
cross-krb5 = "0.1"
```

## Getting started

### Client

- [`proxy_client`](proxy_client.rs) - A simple HTTP client which uses a Kerberos
  ticket to authenticate against a HTTP proxy.
