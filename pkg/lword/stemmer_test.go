package lword

import "testing"

func TestQueConversionTypical(t *testing.T) {
	str, res := isQue("aquilaque")
	if res {
		t.Error("Invalid return")
	}
	if str != "aquila" {
		t.Error("Invalid removal")
	}

}

func TestQueConversionAbnormal(t *testing.T) {
	str, res := isQue("quoque")
	if !res {
		t.Error("Invalid signal")
	}
	if str != "quoque" {
		t.Error("Invalid modification")
	}
}

func TestGetNounStem(t *testing.T) {
	inputs := []string{
		"aquila", "quoque", "ducibus", "a",
	}

	outputs := []string{
		"aquil", "quoque", "duc", "a",
	}

	for i := range inputs {
		res := GetStemNoun(inputs[i])
		if res != outputs[i] {
			t.Errorf("Expected: %s Got: %s", outputs[i], res)
		}
	}
}

func TestGetVerbStem(t *testing.T) {
	inputs := []string{
		"deprehendebatur", "quoque", "ducibus", "a", "aiunt", "accipiuntur", "liberis", "libero",
	}

	outputs := []string{
		"deprehendeba", "quoque", "ducibu", "a", "aiunt", "accipi", "libi", "liberi",
	}

	for i := range inputs {
		res := GetStemVerb(inputs[i])
		if res != outputs[i] {
			t.Errorf("Expected: %s Got: %s", outputs[i], res)
		}
	}
}

func TestGetStem(t *testing.T) {
	noun, verb := GetStem("liberam")
	if noun != "liber" {
		t.Errorf("Expected %s for the noun, got %s", "liber", noun)
	}

	if verb != "libera" {
		t.Errorf("Expected %s for the verb, got %s", "libera", verb)
	}
}
