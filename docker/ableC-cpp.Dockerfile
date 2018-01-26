# TODO release mode
FROM ubuntu
RUN apt update
RUN apt install -y build-essential curl libssl-dev pkg-config
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=stable -y
ENV PATH=/root/.cargo/bin:$PATH
RUN mkdir /code
COPY . /code
WORKDIR /code
RUN cargo build --all

FROM openjdk
RUN apt update && apt install -y ant supervisor && rm -rf /var/lib/apt/lists/*
WORKDIR /root/
RUN git clone https://github.com/melt-umn/silver.git
RUN git clone https://github.com/melt-umn/ableC.git
RUN git clone https://github.com/melt-umn/ableC-monto.git
RUN mkdir extensions
WORKDIR /root/extensions
RUN git clone https://github.com/melt-umn/ableC-algebraic-data-types.git
RUN git clone https://github.com/melt-umn/ableC-cilk.git
RUN git clone https://github.com/melt-umn/ableC-regex-lib.git
RUN git clone https://github.com/melt-umn/ableC-regex-pattern-matching.git
WORKDIR /root/silver
RUN git checkout feature/better_errors
RUN ./update
WORKDIR /root/silver/support/bin
RUN mkdir ~/bin
RUN ./install-silver-bin
ENV PATH=/root/bin:$PATH
WORKDIR /root/
# Build ableC-monto.jar
RUN	silver -o ableC-monto.jar \
	-I ableC \
	-I ableC-monto \
	-I extensions/ableC-algebraic-data-types/grammars \
	-I extensions/ableC-cilk/grammars \
	-I extensions/ableC-regex-lib/grammars \
	-I extensions/ableC-regex-pattern-matching/grammars \
	edu:umn:cs:melt:ableC:monto:demo
# N.B. This is debug mode
COPY --from=0 /code/target/debug/monto3-cpp .
COPY misc/docker-demo/ableC-cpp.supervisord.conf supervisord.conf
COPY misc/docker-demo/monto-cpp.toml .
CMD ["supervisord", "-c", "./supervisord.conf"]
