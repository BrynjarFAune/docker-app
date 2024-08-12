FROM rust:1.80.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

WORKDIR /src

COPY . .

EXPOSE 8080

# Install ping
RUN apt-get update && apt-get install -y iputils-ping

RUN rustup default stable
RUN cargo build

CMD ["cargo", "run"]
