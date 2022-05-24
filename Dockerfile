FROM rust:1.60 AS builder

WORKDIR /home/td

COPY . .

RUN  adduser --home /home/td --shell /bin/false td \
  && cargo build --release \
  && cp target/release/td /usr/local/bin/td

# FROM alpine:3.10.1
# COPY --from=builder /usr/local/bin/td /usr/local/bin/td
# RUN  adduser -D td
# WORKDIR /home/td

USER td

ENTRYPOINT [ "/usr/local/bin/td" ]
