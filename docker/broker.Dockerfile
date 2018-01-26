FROM ubuntu
RUN apt update
RUN apt install -y build-essential curl libssl-dev pkg-config
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=stable -y
ENV PATH=/root/.cargo/bin:$PATH
RUN mkdir /code
COPY . /code
WORKDIR /code
RUN cargo build --all

FROM ubuntu
WORKDIR /root/
# N.B. Debug mode
COPY --from=0 /code/target/debug/monto3-broker .
COPY misc/docker-demo/monto-broker.toml .
CMD ["./monto3-broker"]
