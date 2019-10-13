FROM rust:1.38.0-stretch

WORKDIR /usr/src/myapp

COPY ./myCargo.toml ./Cargo.toml


RUN cargo install sccache
ENV RUSTC_WRAPPER sccache
RUN sccache --start-server
RUN RUSTC_WRAPPER=sccache  cargo install diesel_cli --no-default-features --features postgres
RUN RUSTC_WRAPPER=sccache  cargo install cargo-watch
RUN sccache --show-stats

WORKDIR /usr/src/myapp
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN RUSTC_WRAPPER=sccache  cargo build
RUN sccache --show-stats



