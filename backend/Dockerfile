# needs the builder to have executed
# `cargo build`
# beforehand

FROM debian:12-slim
RUN apt update && apt install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY target/release/ruscalimat-backend /usr/local/bin/ruscalimat

EXPOSE 8080
CMD ["ruscalimat"]
