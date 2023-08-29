FROM rust:latest as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:latest
WORKDIR /app
COPY --from=builder /app/target/release/k8s-autodeploy /app/bin
CMD ["./bin"]