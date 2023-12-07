FROM messense/rust-musl-cross:x86_64-musl as builder
RUN rustup update nightly && \
    rustup target add --toolchain nightly x86_64-unknown-linux-musl

RUN rustup default nightly

WORKDIR /app

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust_ddd_template /
ENTRYPOINT [ "/api" ]
EXPOSE 3000