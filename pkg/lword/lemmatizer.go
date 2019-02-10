package lword

// Lemmatizer a way to resolve a string to a lemma
type Lemmatizer interface {
	GetLemma(s string) []string
	NormalizeOrDefault(s string, def []string) []string
}
