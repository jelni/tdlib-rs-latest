# tdlib-rs-latest

This fork builds the [`tdlib-rs`](https://github.com/paper-plane-developers/tdlib-rs) crate using a TL schema from a custom TDLib commit from the `TDLIB_COMMIT_HASH` environment variable.

This is useful in a Docker image.

## Example

`Cargo.toml`

```toml
tdlib = { git = "https://github.com/jelni/tdlib-rs-latest" }
```

`.env`

```dotenv
TDLIB_COMMIT_HASH="2e5319ff360cd2d6dab638a7e0370fe959e4201b"
```

`docker-compose.yml`

```yaml
services:
    example:
        environment:
            TDLIB_COMMIT_HASH: ${TDLIB_COMMIT_HASH}
```

`Dockerfile`

```dockerfile
...
RUN git clone https://github.com/tdlib/td
WORKDIR /td
RUN git checkout $TDLIB_COMMIT_HASH
...
```

This guarantees the crate will be compatible with the TDLib version specified.

The caveat is that you need internet connection during the build. This shouldn't be a problem on a host intended to run a Telegram bot.

Go to https://github.com/paper-plane-developers/tdlib-rs for actual README.
