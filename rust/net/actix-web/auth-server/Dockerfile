FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM ubuntu:latest as runner
COPY --from=builder /usr/local/cargo/bin/auth-server /usr/local/bin/auth-server
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/usr/local/bin/auth-server"]
