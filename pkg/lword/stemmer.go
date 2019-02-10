package lword

import (
	"strings"
)

// GetStem implements the Schinke Latin stemming algorithm, with both noun and verb stemming
func GetStem(lemma string) (string, string) {
	verb := GetStemVerb(lemma)
	noun := GetStemNoun(lemma)
	return noun, verb
}

// GetStemNoun implements Schinke Latin stemming algorithm only for noun stemming
func GetStemNoun(lemma string) string {

	lemma = Convert(lemma)

	str, res := isQue(lemma)
	if res {
		return str
	}
	lemma = str

	stem := lemma
	suffixList := strings.Split("ibus ius ae am as em es ia is nt os ud um us a e i o u", " ")
	for _, suff := range suffixList {
		if strings.HasSuffix(lemma, suff) {
			stem = strings.TrimSuffix(lemma, suff)
			break
		}
	}

	if len(stem) >= 2 {
		return stem
	}
	return lemma

}

// GetStemVerb implements Schinke Latin stemming algorithm only for verb stemming
func GetStemVerb(lemma string) string {
	lemma = Convert(lemma)

	str, res := isQue(lemma)
	if res {
		return str
	}
	lemma = str

	stem := lemma
	suffixList := strings.Split("iuntur beris erunt untur iunt mini ntur stis bor ero mur mus ris sti tis tur unt bo ns nt ri m r s t", " ")
	for _, suff := range suffixList {
		if strings.HasSuffix(lemma, suff) {
			var replacementSuf string
			switch suff {
			case "iuntur":
				fallthrough
			case "erunt":
				fallthrough
			case "untur":
				fallthrough
			case "iunt":
				fallthrough
			case "unt":
				replacementSuf = "i"
			case "beris":
				fallthrough
			case "bor":
				fallthrough
			case "bo":
				replacementSuf = "bi"
			case "ero":
				replacementSuf = "eri"
			default:
				replacementSuf = ""
			}
			stem = strings.TrimSuffix(lemma, suff)
			if len(stem) >= 2 {
				stem += replacementSuf
				return stem
			}
			break
		}
	}

	return lemma

}

func isQue(lemma string) (string, bool) {

	// Figure 4 of the paper
	abnormalQueWordsLs := strings.Split("atque quoque neque itaque absque apsque abusque adaeque adusque denique "+
		"deque susque oblique peraeque plenisque quandoque quisque quaeque "+
		"cuiusque cuique quemque quamque quaque quique quorumque quarumque "+
		"quibusque quosque quasque quotusquisque quousque ubique undique usque "+
		"uterque utique utroque utribique torque coque concoque contorque "+
		"detorque decoque excoque extorque obtorque optorque retorque recoque "+
		"attorque incoque intorque praetorque", " ")

	// Convert to map for better lookup
	abnormalQueWords := make(map[string]struct{})
	for _, w := range abnormalQueWordsLs {
		abnormalQueWords[w] = struct{}{}
	}

	if strings.HasSuffix(lemma, "que") {
		if _, ok := abnormalQueWords[lemma]; ok {
			return lemma, true
		}
		lemma = strings.TrimSuffix(lemma, "que")
	}
	return lemma, false
}
