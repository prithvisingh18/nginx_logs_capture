FROM rust:1.75.0 as development
ENV SERVER_ENV development

RUN mkdir /usr/src/benchmark
COPY . /usr/src/benchmark
WORKDIR /usr/src/benchmark
CMD sleep infinity
