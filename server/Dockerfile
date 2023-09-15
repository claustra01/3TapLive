FROM rust:slim-buster as build
WORKDIR /opt
COPY . .
RUN cargo build --release

FROM debian:buster-slim
EXPOSE 8080
COPY --from=build /opt/target/release/hackz-tyranno /opt/
ENTRYPOINT ["/opt/hackz-tyranno"]