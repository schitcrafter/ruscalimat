# needs the builder to have executed
# `cargo build`
# beforehand

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
COPY target/release/ruscalimat-backend /usr/local/bin/ruscalimat

EXPOSE 8080
CMD ["ruscalimat"]
