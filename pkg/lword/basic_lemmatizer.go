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

// String set would be more efficient
func contains(forms []string, form string) bool {
	for _, v := range forms {
		if v == form {
			return true
		}
	}
	return false
}

// Heuristic to remove some lemmatizer
func dedup(lemmatizerMapping map[string][]string) map[string][]string {
	intermediateLemmaForm := make(map[string][]string)

	// Buid the intermediate mapping
	for form, lemmas := range lemmatizerMapping {
		for _, lemma := range lemmas {
			forms, ok := intermediateLemmaForm[lemma]
			if ok {
				intermediateLemmaForm[lemma] = append(forms, form)
			} else {
				intermediateLemmaForm[lemma] = []string{form}
			}
		}
	}

	finalMapping := make(map[string][]string)
	for lemma, forms := range intermediateLemmaForm {
		candidates := make(map[string]struct{})

		//  Init the intersection, with all the lemmas for the form
		startingCands := lemmatizerMapping[forms[0]]
		for _, cand := range startingCands {
			// We don't want to add our own lemma
			if cand != lemma {
				candidates[cand] = struct{}{}
			}
		}

		// Check if all the forms are ambig
		allAmbig := true
		for _, form := range forms {
			candidateLemmas := lemmatizerMapping[form]
			if len(candidateLemmas) <= 1 {
				allAmbig = false
				break
			}

			// Intersect all
			newCand := make(map[string]struct{})
			for origCandidate := range candidates {
				if contains(candidateLemmas, origCandidate) {
					newCand[origCandidate] = struct{}{}
				}
			}
			// Update the value
			candidates = newCand
		}

		if !allAmbig {
			// Copy the trivial case
			for _, form := range forms {
				lemmas, ok := finalMapping[form]
				if !ok {
					finalMapping[form] = []string{}
				}
				if !contains(finalMapping[form], lemma) {
					finalMapping[form] = append(lemmas, lemma)
				}
			}
			continue
		}

		// If we are here, we are not ambiguos
		// Look at all candidates, see if any one of them contains the lemma in hand
		foundAnEncloser := false
		for candidate := range candidates {
			candidateForms := intermediateLemmaForm[candidate]
			// We cannot have proper containment in this case
			if len(candidateForms) == len(forms) {
				break
			}
			containsAll := true
			for _, form := range forms {
				if !contains(candidateForms, form) {
					containsAll = false
				}
			}

			if containsAll {
				foundAnEncloser = true
				break
			}
		}

		// If it is not contained in all the thing, just add it to the mapping
		if !foundAnEncloser {
			for _, form := range forms {
				lemmas, ok := finalMapping[form]
				if !ok {
					finalMapping[form] = []string{}
				}
				if !contains(finalMapping[form], lemma) {
					finalMapping[form] = append(lemmas, lemma)
				}
			}
		}
	}

	return finalMapping
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
		// Remove accented forms
		head := Convert(line[2])

		// Remove aphostrophe, and add suffix
		if strings.Contains(head, "'") {
			head = strings.Replace(head, "'", "", -1) + "2"
		}

		// Make every lemma a form of itself
		if headLemmas, ok := mapping[head]; ok {
			if !contains(headLemmas, head) {
				mapping[head] = append(headLemmas, head)
			}
		} else {
			mapping[head] = []string{head}
		}

		if heads, ok := mapping[form]; ok {
			if !contains(heads, head) {
				mapping[form] = append(heads, head)
			}
		} else {
			mapping[form] = []string{head}
		}
	}

	mapping = dedup(mapping)

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
