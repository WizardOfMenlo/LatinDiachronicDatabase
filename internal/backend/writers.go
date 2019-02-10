package backend

import (
	"fmt"
	"io"
	"log"
	"os"
	"sort"
	"strconv"
	"time"
)

// Generate all the needed files
func Generate(mapping DictionaryMap, hist AuthorHistorical) {

	// Use this as the directory name
	dirName := "resultSet_0x" + strconv.FormatInt(time.Now().Unix(), 16)

	err := os.Mkdir(dirName, 0777)
	if err != nil {
		log.Println("Error in creating the directory")
		return
	}

	ambig := mapping.GetAmbiguosWords()

	// Write the maps
	writeAlphaMap(dirName+"/alfa_formatted_lemma.txt", mapping, ambig)
	writeFreqMap(dirName+"/freq_formatted_lemma.txt", mapping, ambig)
	writeHistMap(dirName+"/hist_formatted_lemma.txt", mapping, ambig, hist)
	log.Println("File writing ended")
}

const alphaMode = false
const freqMode = true

func writeStem(f io.Writer, key string, mapping DictionaryMap, sortingMode bool, ambig stringSet) {
	// Write header
	f.Write([]byte("-------------------------------------"))
	totalC := 0
	totalA := 0

	forms := mapping.GetLemma(key)

	// Count the total
	for f, count := range forms {
		if _, ok := ambig[f]; ok {
			totalA += count
		} else {
			totalC += count
		}
	}

	// Write the count for the headword
	f.Write([]byte(fmt.Sprintf("%s count: %d (C:%d, A:%d)\n", key, totalA+totalC, totalC, totalA)))

	type pair struct {
		lemma string
		value int
	}

	// Sort by value
	mappingCount := []pair{}
	i := 0
	for form, val := range forms {
		mappingCount = append(mappingCount, pair{form, val})
		i++
	}

	if sortingMode == freqMode {
		sort.Slice(mappingCount, func(i, j int) bool {
			return mappingCount[i].value > mappingCount[j].value
		})
	} else {
		sort.Slice(mappingCount, func(i, j int) bool {
			return mappingCount[i].lemma < mappingCount[j].lemma
		})
	}

	// Write each form
	for _, form := range mappingCount {

		// If ambiguos add an asterisk
		if _, ok := ambig[form.lemma]; ok {
			f.Write([]byte(fmt.Sprintln("\t", form.lemma, ":", form.value, "(*)")))
		} else {
			f.Write([]byte(fmt.Sprintln("\t", form.lemma, ":", form.value)))
		}
	}

}

func writeAlphaMap(path string, mapping DictionaryMap, ambig stringSet) {
	// Create the file
	f, err := os.Create(path)
	if err != nil {
		log.Println(err)
		return
	}
	defer f.Close()

	// Sort alphabetically
	vec := []string{}
	i := 0
	for _, stem := range mapping.GetKeys() {
		if stem != NotFound {
			vec = append(vec, stem)
			i++
		}
	}
	sort.Strings(vec)

	// Write in order
	for _, stem := range vec {
		writeStem(f, stem, mapping, alphaMode, ambig)
	}

	// Write the not founds at the end
	writeStem(f, NotFound, mapping, alphaMode, ambig)
}

func writeHistoricalInfo(f io.Writer, lemma string, mapping DictionaryMap, historicalData AuthorHistorical) {
	forms := mapping.internal[lemma]
	hist := make(map[TimeDescr]int)
	authors := make(map[string]struct{})
	for _, metadata := range forms {
		for _, record := range metadata {
			author := record.Author
			authors[author] = struct{}{}
			dates := historicalData.mapping[author]
			for _, date := range dates.Between() {
				if _, ok := hist[date]; ok {
					hist[date]++
				} else {
					hist[date] = 1
				}
			}
		}
	}

	interestingSpan := TimeSpan{TimeDescr{10, BC}, TimeDescr{10, AC}}

	f.Write([]byte(fmt.Sprintln("\tUsed by", len(authors), "author(s)")))
	f.Write([]byte("\tHistorical Data:\n"))
	for _, date := range interestingSpan.Between() {
		if _, ok := hist[date]; !ok {
			continue
		}
		f.Write([]byte(fmt.Sprintln("\t\t", date.ToString(), ":", hist[date])))
	}
}

func writeHistMap(path string, mapping DictionaryMap, ambig stringSet, hist AuthorHistorical) {
	// Create the file
	f, err := os.Create(path)
	if err != nil {
		log.Println(err)
		return
	}
	defer f.Close()

	// Sort alphabetically
	vec := []string{}
	i := 0
	for _, stem := range mapping.GetKeys() {
		if stem != NotFound {
			vec = append(vec, stem)
			i++
		}
	}
	sort.Strings(vec)

	// Write in order
	for _, stem := range vec {
		writeStem(f, stem, mapping, alphaMode, ambig)
		writeHistoricalInfo(f, stem, mapping, hist)
	}

	// Write the not founds at the end
	writeStem(f, NotFound, mapping, alphaMode, ambig)
}

func writeFreqMap(path string, mapping DictionaryMap, ambig stringSet) {

	// Create the file
	f, err := os.Create(path)
	if err != nil {
		log.Println(err)
		return
	}
	defer f.Close()

	// Used for sorting by val
	type pair struct {
		lemma string
		value int
	}

	// Sort by value
	mappingCount := []pair{}
	i := 0
	// Create a array of (lemma occurrences)
	for _, lemma := range mapping.GetKeys() {
		forms := mapping.GetLemma(lemma)
		// Skip not founds
		if lemma == NotFound {
			continue
		}
		total := 0
		for _, nums := range forms {
			total += nums
		}
		mappingCount = append(mappingCount, pair{lemma, total})
		i++
	}

	// Sort all
	sort.Slice(mappingCount, func(i, j int) bool {
		return mappingCount[i].value > mappingCount[j].value
	})

	// Write all
	for _, p := range mappingCount {
		writeStem(f, p.lemma, mapping, freqMode, ambig)
	}

	writeStem(f, NotFound, mapping, freqMode, ambig)

}
