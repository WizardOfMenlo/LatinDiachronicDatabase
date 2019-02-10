package lword

import (
	"strings"
	"testing"
)

const r string = ("word1\t1\tfirst ()\nword2\t2\tsecond ()")
const invalid string = "word1\nword2\t\n"

func TestNew(t *testing.T) {
	_, err := NewBasicLemmatizer(strings.NewReader(r))
	if err != nil {
		t.Error("The value should have loaded correctly")
	}

}

func TestInvalidCreate(t *testing.T) {
	_, err := NewBasicLemmatizer(strings.NewReader(invalid))
	if err == nil {
		t.Error("This should have failed")
	}
}

func TestQuery(t *testing.T) {
	l, _ := NewBasicLemmatizer(strings.NewReader(r))
	lemma := l.GetLemma("first")
	if len(lemma) != 1 && lemma[0] != "word1" {
		t.Error("Invalid stemming, should have been word1 was", lemma)
	}

	lemma = l.NormalizeOrDefault("second", []string{"ERROR"})
	if len(lemma) != 1 && lemma[0] != "word2" {
		t.Error("Invalid lemming, should have been word2 was", lemma)
	}
}

func TestQueryFails(t *testing.T) {
	l, _ := NewBasicLemmatizer(strings.NewReader(r))
	nilLemma := l.GetLemma("not in the map")
	if len(nilLemma) != 0 {
		t.Error("Should have errored out")
	}

	lemmas := l.NormalizeOrDefault("not in the map", []string{"ERROR"})
	if len(lemmas) != 1 && lemmas[0] != "ERROR" {
		t.Error("Should have been ERROR got", lemmas)
	}
}
