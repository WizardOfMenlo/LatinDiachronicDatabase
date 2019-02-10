package lword

import "testing"

func TestConvert(t *testing.T) {
	inputs := []string{
		"", "hello", "v", "u", "j", "i", "dvra lex, sed lex", "á, é, í, ó, ú, ü, ñ", "In amóre inermus", "Hell<o> t<[here]>"}
	outputs := []string{
		"", "hello", "u", "u", "i", "i", "dura lex, sed lex", "a, e, i, o, u, u, n,", "In amore inermus", "Hello there"}

	for i := range inputs {
		res := Convert(inputs[i])
		if res != outputs[i] {
			t.Errorf("Expected: %s Got: %s", outputs[i], res)
		}
	}
}
