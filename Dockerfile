# syntax=docker/dockerfile:1.7
FROM scratch
COPY ./dist/spin.toml .
COPY ./dist/spin_http_rust_example.wasm .
