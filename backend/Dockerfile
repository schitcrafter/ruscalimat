# needs the builder to have executed
# `cargo install --path .`
# beforehand

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY /usr/local/cargo/bin/ruscalimat /usr/local/bin/ruscalimat
CMD ["ruscalimat"]
