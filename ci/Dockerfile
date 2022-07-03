# Builder Image
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  g++ ca-certificates make libc6-dev musl-tools \
  libprotobuf-dev protobuf-compiler

ENV CXX=g++
ENV USER=chiraldb
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /chiral-db-grpc

COPY ./ .

# RUN cargo build --target x86_64-unknown-linux-musl --release
RUN cargo build --target x86_64-unknown-linux-musl


# Work Image
FROM alpine:latest
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /chiral-db-grpc
COPY --from=builder /chiral-db-grpc/target/x86_64-unknown-linux-musl/debug/chiral-db-server ./
# COPY --from=builder /chiral-db-grpc/target/x86_64-unknown-linux-musl/release/chiral-db-server ./
COPY ./ci/chembl_28_chemreps_99.txt ./
USER chiraldb:chiraldb

ENV CHRLDB_CHEMBL_TXTFILE='chembl_28_chemreps_99.txt'
CMD ["/chiral-db-grpc/chiral-db-server"]