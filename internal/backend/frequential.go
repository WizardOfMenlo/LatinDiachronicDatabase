package backend

import (
	"bufio"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"sync"

	"github.com/WizardOfMenlo/LatinDiachronicDatabase/pkg/lword"
)

// NotFound Sentinel value for values the lemmatizer cannot resolve
const NotFound = "NOT_FOUND"

var reg = regexp.MustCompile("[^a-zA-Z\\s]+")

type dataUnit struct {
	value string
	data  FormMetaData
}

func readFile(channelOut chan dataUnit, wg *sync.WaitGroup, path string) {
	// Open the file
	file, err := os.Open(path)
	if err != nil {
		log.Printf("error accessing path %s\n", path)
	}
	defer file.Close()

	// Store metadata
	dir, filename := filepath.Split(path)
	filename = strings.TrimSuffix(filename, filepath.Ext(filename))
	author := filepath.Base(dir)

	lineNumber := 0

	// For each line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := reg.ReplaceAllString(lword.Convert(scanner.Text()), " ")

		// Divide into words, and send over the channel
		words := strings.Fields(line)
		for _, w := range words {
			channelOut <- dataUnit{
				value: w,
				data: FormMetaData{
					Author:     author,
					FileName:   filename,
					Path:       path,
					LineNumber: lineNumber,
				},
			}
		}
		lineNumber++
	}
	// Signal we are done
	wg.Done()
}

func consume(lemmatizer lword.Lemmatizer, mapChan chan dataUnit, done chan DictionaryMap, errStream chan error) {
	lemmaMap := newDictionary()
	log.Println("Starting data gathering...")
	// For each word that comes in
	for d := range mapChan {
		// Normalizer and convert
		w := d.value
		w = lword.Convert(strings.ToLower(strings.TrimSpace(w)))
		lemma := lemmatizer.NormalizeOrDefault(w, []string{NotFound})
		for _, v := range lemma {
			lemmaMap.addToMap(v, w, d.data)
		}
	}
	log.Println("Data gathering terminated")

	done <- lemmaMap
}

// LoadDictionary creates the dictionary from the given paths
func LoadDictionary(dataPath, lemmaPath string) (DictionaryMap, lword.Lemmatizer, error) {

	// Open lemmatizer file
	f, errF := os.Open(lemmaPath)

	if errF != nil {
		log.Println("Error opening lemma file", lemmaPath)
		return newDictionary(), lword.BasicLemmatizer{}, errF
	}

	// Create lemmatizer
	lemmatizer, errL := lword.NewLemmatizerFromCSV(f)

	if errL != nil {
		log.Println("Invalid format for ", lemmaPath)
		return newDictionary(), lword.BasicLemmatizer{}, errL
	}

	log.Println("Starting path discovery...")

	// Synchronization created
	mapChan := make(chan dataUnit)
	res := make(chan DictionaryMap)
	errStream := make(chan error)
	var wg sync.WaitGroup

	// Run consumation goroutine
	go consume(lemmatizer, mapChan, res, errStream)

	// For each file in the path
	err := filepath.Walk(dataPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			log.Printf("prevent panic by handling failure accessing a path %q: %v\n", dataPath, err)
			return err
		}

		// Add to waitgroup
		wg.Add(1)
		go readFile(mapChan, &wg, path)
		return nil
	})

	// On error, we don't want to continue
	if err != nil {
		log.Printf("Error walking the tree\n")
		return DictionaryMap{}, lword.BasicLemmatizer{}, err
	}

	log.Println("Path discovery terminated")

	wg.Wait()

	// Signal to consumer that we are done with the operation
	close(mapChan)

	select {
	case err := <-errStream:
		return newDictionary(), lword.BasicLemmatizer{}, err
	case mapping := <-res:
		return mapping, lemmatizer, nil
	}

}
