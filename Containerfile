FROM rust:1.91
WORKDIR /app
COPY . .

# build wasm file
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release --target wasm32-unknown-unknown
run cp ./target/wasm32-unknown-unknown/release/tic-tac-toe.wasm ./wasm/tic-tac-toe.wasm
run cp -r ./assets ./wasm/assets

# setup web app
RUN cargo install wasm-bindgen-cli
RUN wasm-bindgen --out-name tic-tac-toe \
  --out-dir wasm \
  --target web wasm/tic-tac-toe.wasm
RUN cargo install basic-http-server

EXPOSE 4000
CMD ["basic-http-server", "-a", "0.0.0.0:4000", "wasm"]
