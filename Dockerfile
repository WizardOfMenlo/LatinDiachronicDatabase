FROM rust:1.31 as build

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM alpine:latest

WORKDIR /usr/src/app

COPY --from=build /usr/src/app/runner .
COPY --from=build /usr/scr/app/LatinDiachronicalData/small_works ./data/works
COPY --from=build /usr/scr/app/LatinDiachronicalData/out.txt ./data/lemm.txt
COPY --from=build /usr/scr/app/LatinDiachronicalData/cronological_authors.txt ./data/cronological_authors.txt

CMD [ "runner", "-d", "data/works/", "-l", "data/lemm.txt", "-a", "data/cronological_authors.txt" ]