FROM rust:1.70-alpine

ARG BRANCH="main"
ARG COMMIT=""
LABEL branch=${BRANCH}
LABEL commit=${COMMIT}
ENV COMMIT=${COMMIT}
ENV BRANCH=${BRANCH}

ADD . .

RUN apk add --no-cache musl-dev
RUN cargo build --release

CMD ["./target/release/www"]
