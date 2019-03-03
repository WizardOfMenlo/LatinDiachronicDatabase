package lwordtests

import (
	"bufio"
	"os"
	"strings"
	"testing"

	"github.com/WizardOfMenlo/LatinDiachronicDatabase/pkg/lword"
)

func TestDictionary(t *testing.T) {
	file, err := os.Open("testdata/joined.txt")
	if err != nil {
		t.Fatal("Error opening the test file")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		fields := strings.Fields(line)
		if len(fields) != 3 {
			t.Errorf("Invalid test file arguments on line %s", line)
		}
		lemma := fields[0]
		noun := fields[1]
		verb := fields[2]

		nounRes, verbRes := lword.GetStem(lemma)

		if nounRes != noun {
			t.Errorf("Noun: %s, got %s on lemma %s \n", noun, nounRes, lemma)
		}

		if verbRes != verb {
			t.Errorf("Verb: %s, got %s on lemma %s \n", verb, verbRes, lemma)
		}

	}

}
