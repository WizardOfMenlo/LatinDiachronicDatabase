package server

import (
	"log"
	"net/http"
	"time"
)

func loggerDecorator(inner http.HandlerFunc, name string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		t := time.Now()
		inner.ServeHTTP(w, r)
		elapsed := time.Since(t)
		log.Printf(
			"%s\t%s\t%s\t%s",
			r.Method,
			r.RequestURI,
			name,
			elapsed,
		)
	}
}
