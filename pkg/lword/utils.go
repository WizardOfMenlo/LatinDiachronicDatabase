package lword

import (
	"strings"
	"unicode"

	"golang.org/x/text/transform"
	"golang.org/x/text/unicode/norm"
)

func isMn(r rune) bool {
	return unicode.Is(unicode.Mn, r) // Mn: nonspacing marks
}

// Convert to a standard form latin words
func Convert(lemma string) string {

	// Stem normalization and notes elimination
	old := []string{"j", "v", "[", "]", "{", "}", "(", ")", "<", ">"}
	new := []string{"i", "u", "", "", "", "", "", "", "", ""}

	for i := range old {
		lemma = strings.Replace(lemma, old[i], new[i], -1)
	}

	// Unicode normalization
	t := transform.Chain(norm.NFD, transform.RemoveFunc(isMn), norm.NFC)
	result, _, _ := transform.String(t, lemma)

	return result
}
