FROM rust:1.76

WORKDIR /usr/src/bot-discord
COPY . .

RUN cargo install --path .

CMD ["bot-discord"]

EXPOSE 8080

# FROM rust:1.76 as builder
# WORKDIR /usr/src/bot-discord
# COPY . .
# RUN cargo install --path .

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/bot-discord /usr/local/bin/bot-discord
# CMD ["bot-discord"]