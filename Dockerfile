FROM rust:1.31 as build

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM alpine:latest

WORKDIR /usr/src/app

COPY --from=build /usr/src/app/runner .

CMD [ "runner" ]