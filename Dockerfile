FROM ekidd/rust-musl-builder:nightly-2020-04-17 as builder

ARG framework
ARG appName=by-${framework}

WORKDIR /home/rust/src
COPY ./ /home/rust/src/

RUN cargo build --release --target=x86_64-unknown-linux-musl -p ${appName}

FROM alpine:3.10

ARG framework
ARG appName=by-${framework}

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/${appName} /bin/simple-server

CMD ["simple-server"]
