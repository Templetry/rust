# Build stage
FROM rust:1 AS build
WORKDIR /src
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:stable-slim
COPY --from=build /src/target/release/template-app /usr/local/bin/template-app
EXPOSE 8080
CMD ["template-app"]
