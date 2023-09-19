FROM rust:slim-buster
RUN apt update && \
    apt install -y \
      libpq-dev \
      default-libmysqlclient-dev \
      libsqlite3-dev \
      pkg-config \
      libssl-dev
WORKDIR /opt

COPY . .
EXPOSE 8080

RUN cargo build --release
ENTRYPOINT ["./target/release/hackz-tyranno"]