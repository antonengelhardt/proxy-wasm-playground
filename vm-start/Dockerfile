FROM rust:1.75.0 AS builder

COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN rustup target add wasm32-wasip1

RUN cargo build --target=wasm32-wasip1 --release

##################################################

FROM envoyproxy/envoy:v1.30-latest

COPY --from=builder /target/wasm32-wasip1/release/vm_start.wasm /etc/envoy/proxy-wasm-plugins/vm_start.wasm

CMD [ "envoy", "-c", "/etc/envoy/envoy.yaml" ]
