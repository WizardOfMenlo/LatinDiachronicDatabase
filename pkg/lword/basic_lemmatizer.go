package lword

import (
	"bufio"
	"encoding/csv"
	"errors"
	"io"
	"log"
	"strings"
)

// BasicLemmatizer convert a latin word to its base lemma
type BasicLemmatizer struct {
	lemmaMap map[string][]string
}

// NewBasicLemmatizer takes a reader, and uses it to build a Lemmatizer
func NewBasicLemmatizer(f io.Reader) (BasicLemmatizer, error) {

	mapping := make(map[string][]string)
	scan := bufio.NewScanner(f)
	for scan.Scan() {
		line := scan.Text()
		parts := strings.Split(line, "\t")
		if len(parts) < 2 {
			return BasicLemmatizer{nil}, errors.New("Invalid line tokens")
		}
		headword := Convert(strings.TrimSpace(parts[0]))
		associatedLemmas := parts[2:]
		for _, wordData := range associatedLemmas {
			sections := strings.Split(wordData, " ")
			lemma := Convert(strings.TrimSpace(sections[0]))
			mapping[lemma] = append(mapping[lemma], headword)
		}
	}

	l := BasicLemmatizer{lemmaMap: mapping}
	return l, nil
}

func contains(forms []string, form string) bool {
	for _, v := range forms {
		if v == form {
			return true
		}
	}
	return false
}

// NewLemmatizerFromCSV Create from a CSV file
func NewLemmatizerFromCSV(f io.Reader) (BasicLemmatizer, error) {
	mapping := make(map[string][]string)
	reader := csv.NewReader(f)

	for {
		line, err := reader.Read()
		if err == io.EOF {
			break
		} else if err != nil {
			log.Println(err)
			continue
		}
		form := Convert(line[0])
		head := Convert(line[2])

		if heads, ok := mapping[form]; ok {
			if !contains(heads, head) {
				mapping[form] = append(heads, head)
			}
		} else {
			mapping[form] = []string{head}
		}
	}
	return BasicLemmatizer{lemmaMap: mapping}, nil
}

// GetLemma attempt to parse a lemmata, and find the associated lemma
func (l BasicLemmatizer) GetLemma(s string) []string {
	r, ok := l.lemmaMap[Convert(s)]
	if !ok {
		return []string{}
	}
	return r
}

// NormalizeOrDefault attempts to normalizer according to the dictionary, on error returns
func (l BasicLemmatizer) NormalizeOrDefault(s string, def []string) []string {
	r := l.GetLemma(s)
	if len(r) == 0 {
		return def
	}
	return r
}
