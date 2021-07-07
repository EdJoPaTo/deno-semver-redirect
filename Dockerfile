FROM docker.io/ekidd/rust-musl-builder as builder
WORKDIR /home/rust

# cargo needs a dummy src/main.rs to detect bin mode
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --locked

# We need to touch our real main.rs file or the cached one will be used.
COPY . ./
RUN sudo touch src/main.rs

RUN cargo build --release --locked

# Size optimization
RUN strip target/x86_64-unknown-linux-musl/release/deno-semver-redirect


# Start building the final image
FROM scratch
EXPOSE 8080

COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/deno-semver-redirect /usr/bin/

ENTRYPOINT ["deno-semver-redirect"]
