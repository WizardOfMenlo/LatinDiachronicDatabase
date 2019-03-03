package server

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/WizardOfMenlo/LatinDiachronicDatabase/pkg/lword"
	"github.com/gorilla/mux"
)

func (s Data) status(w http.ResponseWriter, r *http.Request) {

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	json.NewEncoder(w).Encode(struct {
		Status string `json:"status"`
	}{
		Status: "OK",
	})
}

func (s Data) countLemma(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	lemma := vars["lemma"]

	criterion := FromRequest("authorsFilter", "date", vars, s.authorHistoric)
	NewActor(&s, criterion).countLemma(w, lemma)
}

func (s Data) countForm(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	// Convert the string, to allow for mistypes?
	form := lword.Convert(strings.ToLower(strings.TrimSpace(vars["form"])))

	criterion := FromRequest("authorsFilter", "date", vars, s.authorHistoric)
	NewActor(&s, criterion).countForm(w, form)
}

func (s *Data) writeFiles(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	criterion := FromRequest("authorsFilter", "date", vars, s.authorHistoric)
	NewActor(s, criterion).writeFiles(w)
}

func (s Data) computeIntersection(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// Parse comma separeted list
	authorString := vars["authors"]
	authors := strings.Split(authorString, ",")

	criterion := FromRequest("authorsFilter", "date", vars, s.authorHistoric)
	NewActor(&s, criterion).computeIntersection(w, authors)
}

func (s Data) getOccurrencesLemma(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	lemma := vars["lemma"]
	lemma = lword.Convert(strings.TrimSpace(strings.ToLower(lemma)))

	criterion := FromRequest("authorsFilter", "date", vars, s.authorHistoric)
	NewActor(&s, criterion).getOccurrencesLemma(w, lemma)
}

func (s Data) getOccurrencesForm(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	form := vars["form"]
	form = lword.Convert(strings.TrimSpace(strings.ToLower(form)))

	criterion := FromRequest("authorsFilter", "date", vars, s.authorHistoric)
	NewActor(&s, criterion).getOccurrencesForm(w, form)
}

func (s Data) getAmbiguosForms(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	criterion := FromRequest("authorsFilter", "date", vars, s.authorHistoric)
	NewActor(&s, criterion).getAmbiguosForms(w)
}

func (s Data) getStats(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	criterion := FromRequest("authorsFilter", "date", vars, s.authorHistoric)
	NewActor(&s, criterion).getStats(w)
}
