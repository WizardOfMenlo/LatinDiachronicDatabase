package server

import (
	"archive/zip"
	"errors"
	"io"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/WizardOfMenlo/FrequentialDictionary/pkg/lword"
	"github.com/WizardOfMenlo/LatinDiachronicDatabase/internal/backend"
)

// Data encapsulates the state of the server
type Data struct {
	mapping        backend.DictionaryMap
	generated      bool
	lemmatizer     lword.Lemmatizer
	debugMode      bool
	authorHistoric backend.AuthorHistorical

	// Used for cleanup
	filePath string
}

// New creates and loads the data for the server
func New(worksPath, lemmaPath, authorsPath string, debugMode bool) (Data, error) {

	log.Println("Author mapping start")
	// Create a reader
	authorReader, err := os.Open(authorsPath)
	if err != nil {
		return Data{}, err
	}

	// Compute the historigram
	authorHist, err := backend.NewAuthorHistorical(authorReader)
	if err != nil {
		return Data{}, err
	}
	log.Println("Author mapping end")

	// Unzip the dir if in zip format
	log.Println("Started Unzipping")
	worksPath, err = unzipIfNeeded(worksPath)
	if err != nil {
		log.Println("Could not unzip files or find path at", worksPath)
		return Data{}, err
	}
	log.Println("Finished Unzipping")

	mapping, lem, err := backend.LoadDictionary(worksPath, lemmaPath)
	if err != nil {
		return Data{}, err
	}

	return Data{mapping, false, lem, debugMode, authorHist, worksPath}, nil
}

// Listen starts listening on this server
func (s Data) Listen() {
	router := newRouter(s)
	log.Println("Started Server")
	log.Fatal(http.ListenAndServe(":8080", router))
}

// Cleanup is called on sig terminations
func (s Data) Cleanup() {
	os.RemoveAll(s.filePath)
	log.Println("Shutting down...")
}

func unzipIfNeeded(dataPath string) (string, error) {
	fi, err := os.Stat(dataPath)
	if err != nil {
		return "", err
	}

	// If it is a directory, we assume it is our collection
	if fi.IsDir() {
		return dataPath, nil
	}

	if !strings.HasSuffix(dataPath, ".zip") {
		return "", errors.New("File has an invalid suffix")
	}

	outputPath := filepath.Dir(dataPath)
	reader, err := zip.OpenReader(dataPath)
	if err != nil {
		return "", err
	}

	for _, file := range reader.File {
		path := filepath.Join(outputPath, file.Name)

		// Skip directories
		if file.FileInfo().IsDir() {
			os.MkdirAll(path, file.Mode())
			continue
		}

		fileReader, err := file.Open()
		if err != nil {
			return "", err
		}
		defer fileReader.Close()

		targetFile, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, file.Mode())
		if err != nil {
			return "", err
		}
		defer targetFile.Close()

		if _, err := io.Copy(targetFile, fileReader); err != nil {
			return "", err
		}
	}

	// TODO This only works if the zip contains a single folder, same name as the zip
	finalPath := filepath.Join(outputPath, strings.TrimSuffix(filepath.Base(dataPath), ".zip"))

	return finalPath, nil
}
