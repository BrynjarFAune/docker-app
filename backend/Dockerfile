FROM rust:latest AS builder

WORKDIR /usr/src/backend
COPY . .

RUN cargo install --path .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD [ "backend" ]
