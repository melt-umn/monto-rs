FROM ubuntu

RUN apt update && \
	apt install -y build-essential curl libssl-dev pkg-config supervisor && \
	rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=stable -y
ENV PATH=/root/.cargo/bin:$PATH

RUN cargo install just

RUN mkdir /code
COPY . /code
WORKDIR /code/broker
RUN cargo install

WORKDIR /code/misc/docker-demo
CMD ["/root/.cargo/bin/monto3-broker"]
