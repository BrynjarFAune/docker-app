FROM rust:nightly AS build
WORKDIR /app

COPY . .
RUN cargo build --release

FROM alpine:3.18
WORKDIR /app

COPY --from=build /app/target/release/backend /usr/local/bin/backend

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8080

CMD ["cargo", "run"]
