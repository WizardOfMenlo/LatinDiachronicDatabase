# Latin Diachronic Database

![](https://github.com/WizardOfMenlo/LatinDiachronicDatabase/workflows/Rust/badge.svg)

The **Latin Diachronic Database** is a project of Digital Humanities invented by **Tommaso Spinelli** (University of Manchester) and co-developed with **Giacomo Fenzi** (ETH Zurich). This project aims to create an innovative toolkit for the quantitative computational analysis of the Latin language as well as to support and further enhance the digital study of ancient intertextuality.

The program currently offers the possibility to run two different searches, both unique in their genre. Firstly, the program can generate data on the diachronic frequency of Latin words by processing big data. Specifically, the program can parse, lemmatise and record all the lemmas attested in a very large corpus of Latin Literature (309 authors) and epigraphs. The program attributes different inflected word-forms (e.g. genitive sing., dative pl. etc for names; pres. 1 pl., ppf 2 sing. etc for verbs) to the correct headword(s) using a freely adapted version of the open source technology LEMLAT 2.0 (see https://github.com/CIRCSE/LEMLAT3; http://www.lemlat3.eu/; Passarotti, M., Budassi, M., Litta, E., Ruffolo, P. (2017) 'The Lemlat 3.0 Package for Morphological Analysis of Latin', Proceedings of the NoDaLiDa 2017 Workshop on Processing Historical Language 133: 24-31. Linköping University Electronic Press. http://www.ep.liu.se/ecp/article.asp?issue=133&article=006&volume=). The headwords are respectively presented in the nominative case for names and in the first singular person of the present for verbs. Our database (c. 14.000.000 words) so far includes 30.489 headwords and 329.310 forms which are alternatively marked as certain (circa 295.271) and ambiguous (circa 34.039) to alert users of the presence of eventual homographs. More importantly, our program allows users to display the results not only in an alphabetical and a chronological order, but also by absolute frequency and by relative chronological frequency. In addition, for each headword the database records the authors in which the word is attested so that users can decide to see the frequency of a word in a specific century/author.

Secondly, the program can be used to detect the words (and/or the word-forms) shared only by two authors in the entire Latin literature or in specific centuries. This function has been designed to support the digital study of intertextuality. For instance, users can easily see which words first attested in Ovid are used only by Statius in extant Latin literature: by revealing meaningful reuses of extremely rare forms the lists created by our program can contribute significantly to the study of an author’s use of allusivity and style.
### TL;DR
This project revolves around allowing for a flexible and easy to use interface to run quantitative queries on the Latin literature. In particular, we focus on providing tool for statistical computational analysis of the Latin language, diachronic frequency analysis and aid for digital intertextual source. 

## Getting Started

``` 
cargo build --release
```

### Prerequisites

Make sure you have ```rust``` and ```cargo``` installed (Tested with version >= 1.30). 
Instruction to install both are found at [rustup.rs](https://rustup.rs/)


### Installing

Build with ```cargo build``` (using ```cargo build --release``` is slower but reccommended for big data sets), install system wide with ```cargo install```.

The program consists of various binaries, that can be found in the ```target/release``` folder that the build command creates.
In particular the options are:
1. *webserver* A GraphQL powered server that can be used for various queries on the data set
2. *dictionary* Runs the backend on the literature, and generates a human readable summary of the data
3. *intersector* Computes the words uniquely used by a certain author (WIP, will be able to intersect selected authors)
4. *json/csv_export* Export the corpus in the desired format

Usage of each of the programs can be investigated using ```prog_name --help``` or ```cargo run --release --bin prog_name -- --help```.
In general the arguments are as follow:
```
USAGE:
    prog_name [FLAGS] [OPTIONS] --data <DIR> --lemmatizer <LEMM_FILE>

FLAGS:
    -h, --help         Prints help information
    -L, --useLemlat    
    -V, --version      Prints version information

OPTIONS:
    -a, --authors <AUTHORS_FILE>    The file where the authors description is
    -d, --data <DIR>                The folder where the body of literature is located
    -l, --lemmatizer <LEMM_FILE>    The file used to build the lemmatizer
```

## Usage
The program takes three required arguments and one optional one, in the following order: 

 1. **data**: A path which refers to a directory. A well formed corpus contains a list of authors directory, each one of them which contains the texts written by said author. 
 2. **lemm_file**: a file containing a CSV lemmatizer representation, of the form ```form,,,lemma```. We also support LemLat format using the switch. 
 3. **authors**: a file which contains a representation of the **corpus**'s authors chronological relevance, which each line of the form ```author_name #(century(a|d) (, century(a|d))*)```, e.g. ```Publius Ovidius Naso #(1a, 1d)```

## Webserver

Once the arguments are specified, the webserver binary will start a web server on port ```8088``` (In general it will attempt to start it ```0.0.0.0:8088```, if the user wants to use it locally he should change it to ```127.0.0.1:8088``` in ```webserver.rs```). 
For convenience the server will start a [graphiql](https://github.com/graphql/graphiql) instance with documentation and a graphical
interface that facilitates the querying.

A graphical interface that can be used to interface in a more friendly manner can be find [here](https://latin.netlify.com/)

## Contributing
Please feel free to open pull requests, I will take time to review them and merge appropriately. 

## Authors

* **Giacomo Fenzi** - *Developer* - [WizardOfMenlo](https://github.com/WizardOfMenlo)
*  **Tommaso Spinelli** - *Inventor/Latinist* -[tommasospinelli](https://github.com/tommasospinelli)
*  **Jack Leslie** - *Developer* -[jackleslie](https://github.com/jackleslie)

See also the list of [contributors](https://github.com/WizardOfMenlo/LatinDiachronicDatabase/contributors) who participated in this project.

## License

The data deriving from the program are being reviewed and will be soon made available in an open source format. Currently, the copyright is reserved by the authors. For any use different from personal use please contact the authors at the following addresses: (gfenzi@ethz.ch, ts206@st-andrews.ac.uk). 
