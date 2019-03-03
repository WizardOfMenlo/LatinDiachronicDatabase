package server

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"net/http"
	"os"
	"regexp"

	"github.com/WizardOfMenlo/LatinDiachronicDatabase/internal/backend"
	"github.com/WizardOfMenlo/LatinDiachronicDatabase/pkg/lword"
)

type Actor struct {
	internal *Data
	crit     FilteringCriteria
}

func NewActor(s *Data, crit FilteringCriteria) Actor {
	return Actor{s, crit}
}

func (a Actor) countLemma(w http.ResponseWriter, lemma string) {

	type ResultStruct struct {
		Lemma string         `json:"lemma"`
		Total int            `json:"total"`
		Forms map[string]int `json:"forms"`
	}

	mapping := a.crit.apply(a.internal.mapping)

	// Sums the number of occurrences for each form
	total := 0
	forms := mapping.GetLemma(lemma)
	for _, v := range forms {
		total += v
	}

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	res := ResultStruct{lemma, total, forms}
	json.NewEncoder(w).Encode(res)
}

func (a Actor) countForm(w http.ResponseWriter, form string) {

	type ResultStruct struct {
		Form   string   `json:"form"`
		Total  int      `json:"total"`
		Lemmas []string `json:"lemmas"`
	}

	// Get the lemmas associated with the form
	lemmas := a.internal.lemmatizer.NormalizeOrDefault(form, []string{backend.NotFound})
	total := 0

	mapping := a.crit.apply(a.internal.mapping)

	// This is not 100% correct, but given the stemming algo we are using is guaranteed to give the
	// Correct result
	// Slower Alternative : max(lemmas)
	if len(lemmas) != 0 {
		total = mapping.GetLemma(lemmas[0])[form]
	}

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	res := ResultStruct{form, total, lemmas}
	json.NewEncoder(w).Encode(res)
}

func (a Actor) writeFiles(w http.ResponseWriter) {

	mapping := a.crit.apply(a.internal.mapping)

	if !a.internal.generated {
		go backend.Generate(mapping, a.internal.authorHistoric)
		a.internal.generated = true
	}
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	json.NewEncoder(w).Encode("DONE")
}

func (a Actor) computeIntersection(w http.ResponseWriter, authors []string) {

	mapping := a.crit.apply(a.internal.mapping)
	res := mapping.ComputeIntersection(authors)

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	json.NewEncoder(w).Encode(res)
}

func (a Actor) getOccurrencesLemma(w http.ResponseWriter, lemma string) {
	mapping := a.crit.apply(a.internal.mapping)

	// Single lemma -> Multiple forms
	res := getOccurrences([]string{lemma}, mapping.GetForms(lemma), mapping)

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	json.NewEncoder(w).Encode(res)
}

func (a Actor) getOccurrencesForm(w http.ResponseWriter, form string) {
	mapping := a.crit.apply(a.internal.mapping)

	lemmas := a.internal.lemmatizer.GetLemma(form)

	// Multiple lemmas -> Single form
	res := getOccurrences(lemmas, []string{form}, mapping)

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	json.NewEncoder(w).Encode(res)
}

func (a Actor) getAmbiguosForms(w http.ResponseWriter) {
	mapping := a.crit.apply(a.internal.mapping)

	ambig := mapping.GetAmbiguosWords()
	ambigM := make(map[string][]string)
	for k := range ambig {
		ambigM[k] = a.internal.lemmatizer.GetLemma(k)
	}

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	json.NewEncoder(w).Encode(ambigM)

}

func (a Actor) getStats(w http.ResponseWriter) {
	mapping := a.crit.apply(a.internal.mapping)

	// Gather the stats
	keys := mapping.GetKeys()
	headwords := len(keys)
	ambiguos := len(mapping.GetAmbiguosWords())

	entitiesCount := 0
	totalForms := 0
	for _, k := range keys {
		forms := mapping.GetForms(k)
		totalForms += len(forms)
		for _, f := range forms {
			entities := mapping.GetFormData(k, f)
			entitiesCount += len(entities)
		}
	}

	type resultStructStatistics struct {
		Headwords     int `json:"headwords"`
		Ambiguos      int `json:"ambig"`
		Forms         int `json:"forms"`
		TotalEntities int `json:"entities"`
	}

	stats := resultStructStatistics{
		headwords,
		ambiguos,
		totalForms,
		entitiesCount,
	}

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)

	json.NewEncoder(w).Encode(stats)
}

// UTILITIES --------------------------------------------------------------------------------

// Reduce duplicates in a list of metadata
func uniquefy(arr []backend.FormMetaData) []backend.FormMetaData {
	// Create a mapping
	mapping := make(map[string]backend.FormMetaData)
	for _, mData := range arr {
		// Note, this will not work for two of the same in the same row
		hash := fmt.Sprintf("%s%d", mData.Path, mData.LineNumber)

		// If the mapping at the hash does not have a value there, save this data
		if _, ok := mapping[hash]; !ok {
			mapping[hash] = mData
		}
	}

	// Build an array of this data
	res := make([]backend.FormMetaData, 0, len(mapping))
	for _, v := range mapping {
		res = append(res, v)
	}

	return res
}

var reg = regexp.MustCompile("[^a-zA-Z0-9\\s]+")

// Gets the correct line information
func getLine(data backend.FormMetaData) (string, error) {
	// Open the file
	f, err := os.Open(data.Path)
	defer f.Close()
	if err != nil {
		return "", err
	}

	lineNumber := 0

	// For each line
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		if lineNumber == data.LineNumber {
			line := reg.ReplaceAllString(lword.Convert(scanner.Text()), " ")
			return line, nil
		}
		lineNumber++
	}
	return "", errors.New("Invalid Metadata")
}

type resultStructOccurrences struct {
	Author string `json:"author"`
	Text   string `json:"text"`
	Line   string `json:"line"`
}

func getOccurrences(lemmas, forms []string, mapping backend.DictionaryMap) []resultStructOccurrences {
	// Might over allocate, but should be a good estimate
	metaData := make([]backend.FormMetaData, 0, len(lemmas)*len(forms))

	// Each lemma
	for _, lemma := range lemmas {
		// Each form
		for _, form := range forms {
			// Get the metadata for the given lemma form pair
			dataF := mapping.GetFormData(lemma, form)
			// Append this data to our data
			metaData = append(metaData, dataF...)
		}
	}

	metaData = uniquefy(metaData)

	res := make([]resultStructOccurrences, 0, len(metaData))
	for _, data := range metaData {
		// Can be made more efficient by both bundling writes and parallelizing
		line, err := getLine(data)
		// If something is going wrong, I'd be really curious to understand why
		if err != nil {
			log.Printf("%s:%d", data.Path, data.LineNumber)
			log.Println(err)
			continue
		}
		// Append the information
		res = append(res, resultStructOccurrences{Author: data.Author, Text: data.FileName, Line: line})
	}
	return res
}

// UTILS -------------------------------------------------------------------------------------------------------------------------
