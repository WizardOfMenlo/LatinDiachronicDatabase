
# Latin Diachronic Database
[![DOI](https://zenodo.org/badge/170006633.svg)](https://zenodo.org/badge/latestdoi/170006633)

The **Latin Diachronic Database** is a project of Digital Humanities invented by **Tommaso Spinelli** (Ph.D. candidate, Classics, St. Andrews University) and co-developed with **Giacomo Fenzi** (Computer Science and Mathematics student, St. Andrews University). This project aims to create an innovative toolkit for the quantitative computational analysis of the Latin language as well as to support and further enhance the digital study of ancient intertextuality.

The program currently offers the possibility to run two different searches, both unique in their genre. Firstly, the program represents the first diachronic frequency dictionary of the Latin language based on big data (c. 10.500.000 forms). In this respect, it is able to scan and record all the lemmas attested in the whole corpus of extant Latin Literature by attributing different inflected word-forms (e.g. genitive sing., dative pl. etc for names; pres. 1 pl., ppf 2 sing. etc for verbs) to the correct headword(s) using the technology LEMLAT. The headwords are respectively presented in the nominative case for names and in the first singular person of the present for verbs. This dictionary is methodologically and quantitatively different from the only other extant Latin frequency dictionary, namely the _Dictionnaire Fréquentielle_ created by the L.A.S.L.A. laboratory of the University of Liège in 1981 (and recently republished) that is based on a limited corpus of 19 authors (for a total of 1 706 7276 lexical occurrences analysed) and on the Forcellini’s *Lexicon Totius Latinitaits* (1864). Our dictionary (c. 10.000.000 words) so far includes 30.489 headwords and 329.310 forms which are alternatively marked as certain (circa 295.271) and ambiguous (circa 34.039) to alert users of the presence of eventual homographs. More importantly, our program allows users to display the results not only in an alphabetical and a chronological order, but also by absolute frequency and by relative chronological frequency. In addition, for each headword the dictionary records the authors in which the word is attested so that users can decide to see the frequency of a word in a specific timeframe/author.

Secondly, the program can be used to detect the words (and/or the word-forms) shared only by two authors in the entire Latin literature. This function has been designed to support the digital study of intertextuality. For instance, users can easily see which words first attested in Ovid are used only by Statius in extant Latin literature: by revealing meaningful reuses of extremely rare forms the lists created by our program can contribute significantly to the study of an author’s use of allusivity and style.
### TL;DR
This project revolves around allowing for a flexible and easy to use interface to run quantitative queries on the Latin literature. In particular, we focus on providing tool for statistical computational analysis of the Latin language, diachronic frequency analysis and aid for digital intertextual source. 

## Getting Started

``` 
mkdir $GOPATH/src/github.com/WizardOfMenlo
cd $GOPATH/src/github.com/WizardOfMenlo
git clone https://github.com/WizardOfMenlo/LatinDiachronicDatabase
cd LatinDiachronicDatabase

# Install Dependencies
go get github.com/gorilla/mux
go get github.com/rs/cors
go get golang.org/x/text/unicode/norm
go get golang.org/x/text/transform

go run cmd/frequential/main.go
```

### Prerequisites

Make sure you have ```go``` installed (Tested with version >= 1.10).  
Also ensure ```GOPATH``` is set to where you wish the project to be (usual choice ```~/go```).
The following dependencies must be installed (with ```go get```):

 - [ ] github.com/gorilla/mux
 - [ ] github.com/rs/cors
 - [ ] golang.org/x/text/unicode/norm
 - [ ] golang.org/x/text/transform


### Installing

Build with ```go build``` , install system wide with ```go install``` or run directly with ```go run```

Running the program with no arguments should return the following message:
```
"Usage: command-name <folder> <lemma_file> <author_file> [debug]"
```

## Usage
The program takes three required arguments and one optional one, in the following order: 

 1. **corpus**: A path which either refers to a directory or zip file. If the path refers to a zip file it will be unzipped in the same directory as where the zip file is located. A well formed corpus contains a list of authors directory, each one of them which contains the texts written by said author. 
 2. **lemma_file**: a file containing a CSV lemmatizer representation, of the form ```form,,,lemma```. 
 3. **authors_list**: a file which contains a representation of the **corpus**'s authors chronological relevance, which each line of the form ```author_name #(century(a|d) (, century(a|d))*)```, e.g. ```Publius Ovidius Naso #(1a, 1d)```
 4. **debug**: an optional flag, if specified a profiler will be attached at ```debug/pprof```

Once the arguments are specified, the program will start a web server on port ```8080```.
The user can then query the program at the routes specified at [routes.go](https://github.com/WizardOfMenlo/LatinDiachronicDatabase/blob/master/internal/server/routes.go) via standard HTTP requests. The list is also reported here for simplicity, following the convention that ```{x}``` refers to a parameter to the command, and that running a command ```/a/{x}``` should be done via querying ```localhost:8080/a/something``` with, of course, the relevant host-name. Unless specified, all responses are in JSON.  
In particular: 

 1. ```/``` : Returns a non-JSON status of whether the server is running correctly
 2. ```/writeFiles```: Writes to disk a dictionary representation of the corpus
 3. ```/lemma/{lemma}```: Returns the numbers of occurrences for ```lemma```, displaying a breakdown of form occurrences
 4. ```/form/{form}```: Returns the numbers of occurrences for ```form```, and shows the ```lemma```s it gets resolved to.
 5. ```/intersection/{authors}```: Given a list of ```authors``` in the form ```A_1, ..., A_K```, it computes ```Intersection(W(A_i)) - Union(Authors - Union(A_i))```
 6. ```/match/{form}```: Gathers all exact occurrences of ```form``` 
 7. ```/find/{lemma}```: Gathers all occurrences of ```lemma```, including derived forms.
 8. ```/ambig```: Displays all words that the lemmatizer resolve ambiguously
 9. ```/stats```: Displays some relevant statistics about the corpus, e.g number of headwords, total words, ecc.
 10. ```/authors```: Returns a list of all authors in the corpus

Furthermore, for each of these routes, say ```/a```, we associate the following:

 1. ```/a/{authorsFilter}```: Runs ```/a``` on the subset of literature defined by the list of authors specified in ```authorsFilter``` (comma separated list). 
 2. ```/aHist/{date]```: Runs ```/a``` on the subset of literature written by authors active before ```date```

## Contributing
At the moment the project is undergoing a Rust rewrite, in order to further modularize it and make future development easier. As such most of the contributing should be done on the ```rust``` branch. I will still be monitoring and reviewing pull requests on the ```golang``` branch, but most of my effort will not be directed there. 

## Authors

* **Giacomo Fenzi** - *Developer* - [WizardOfMenlo](https://github.com/WizardOfMenlo)
*  **Tommaso Spinelli** - *Inventor/Latinist* -[tommasospinelli](https://github.com/tommasospinelli)

See also the list of [contributors](https://github.com/WizardOfMenlo/LatinDiachronicDatabase/contributors) who participated in this project.

## License

Copyright reserved by authors Tommaso Spinelli and Giacomo Fenzi. The data deriving from the program are currently being published as a monograph, for any use different from personal use please contact the authors at the following addresses: (gf45@st-andrews.ac.uk, ts206@st-andrews.ac.uk). 
