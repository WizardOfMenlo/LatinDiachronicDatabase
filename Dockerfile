FROM golang:1.8

WORKDIR /go/src/github.com/WizardOfMenlo/LatinDiachronicDatabase
COPY . .

RUN go get github.com/gorilla/mux && go get github.com/rs/cors && go get golang.org/x/text/unicode/norm && go get golang.org/x/text/transform && go build -o main.out cmd/frequential/main.go

ENTRYPOINT [ "./main.out", "data/works.zip", "data/out.txt", "data/cronological_authors.txt" ]

EXPOSE 8080
