# rustlang-rocket-mongodb
[![Try it on gitpod](https://img.shields.io/badge/try-on%20gitpod-brightgreen.svg)](https://gitpod.io/#https://github.com/louis030195/rustlang-rocket-mongodb)

Forked repo from this [medium article](https://medium.com/@louis.beaumont/rest-api-with-rust-mongodb-10eeb6bd51d7)



## Usage

```bash
echo -e "MONGO_ADDR=localhost
DB_NAME=rustlang-rocket-mongodb
MONGO_PORT=27017" > .env
echo -e "CLIENT_ID=<client id>
CLIENT_SECRET=<client secret>" >> .env
```


```bash
rustup default nightly # rocket requires a nightly or dev version of Rust
```

```bash
cargo run &

