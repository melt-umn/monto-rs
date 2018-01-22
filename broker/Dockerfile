FROM ubuntu

RUN apt update && apt install -y build-essential curl libssl-dev pkg-config supervisor

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=stable -y
ENV PATH=/root/.cargo/bin:$PATH

RUN cargo install just

RUN mkdir /code
WORKDIR /code
COPY . .

RUN just build
RUN cargo install

RUN mkdir -p /var/log/supervisor
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
CMD ["/usr/bin/supervisord"]
