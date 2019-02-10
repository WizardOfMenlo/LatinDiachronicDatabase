package main

import (
	"log"
	"os"
	"os/signal"

	"github.com/WizardOfMenlo/LatinDiachronicDatabase/internal/server"
)

func main() {

	// Parse arguments
	args := os.Args
	if len(args) < 4 || len(args) > 5 {
		log.Println("Usage:", args[0], "<folder> <lemma_file> <author_file> [debug]")
		return
	}

	folderName := args[1]
	lemmatizerPath := args[2]
	authorFile := args[3]

	serv, err := server.New(folderName, lemmatizerPath, authorFile, len(args) == 5)
	if err != nil {
		log.Fatalln("Error creating server, failing")
	}

	// Handle interrupts
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	go func() {
		for range c {
			serv.Cleanup()
			os.Exit(1)
		}
	}()

	serv.Listen()

}
