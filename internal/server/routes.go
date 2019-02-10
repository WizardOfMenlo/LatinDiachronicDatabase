package server

import (
	"net/http"
	"net/http/pprof"

	"github.com/gorilla/mux"
)

type route struct {
	Name    string
	Method  string
	Pattern string
	Handler http.HandlerFunc
}

func attachProfiler(r *mux.Router) *mux.Router {
	r.HandleFunc("/debug/pprof", pprof.Index)
	r.HandleFunc("/debug/pprof/cmdline", pprof.Cmdline)
	r.HandleFunc("/debug/pprof/profile", pprof.Profile)
	r.HandleFunc("/debug/pprof/symbol", pprof.Symbol)
	return r
}

func newRouter(s Data) *mux.Router {
	routes := []route{
		// Returns the current status of the application
		route{
			"Status",
			"GET",
			"/",
			s.status,
		},

		// Writes the dictionary files to disk
		route{
			"Write",
			"GET",
			"/writeFiles",
			s.writeFiles,
		},

		// Writes the dictionary file for a certain list of authors
		route{
			"WriteFiltered",
			"GET",
			"/writeFiles/{authorsFiltered}",
			s.writeFiles,
		},

		// Writes the dictionary file for a certain list of authors historically
		route{
			"WriteHistorical",
			"GET",
			"/writeFilesHist/{date}",
			s.writeFiles,
		},

		// Get number of occ for the lemma, plus frequencies of the forms
		route{
			"Lemma",
			"GET",
			"/lemma/{lemma}",
			s.countLemma,
		},

		// Get number of occ for the lemma, plus frequencies of the forms, filtered by author
		route{
			"LemmaFiltered",
			"GET",
			"/lemma/{authorsFiltered}/{lemma}",
			s.countLemma,
		},

		// Get number of occ for the lemma, plus frequencies of the forms, filtered by author historically
		route{
			"LemmaHistorical",
			"GET",
			"/lemmaHist/{date}/{lemma}",
			s.countLemma,
		},

		// Get number of occ for the form
		route{
			"Form",
			"GET",
			"/form/{form}",
			s.countForm,
		},

		// Get number of occ for the form, filtered by author
		route{
			"FormFiltered",
			"GET",
			"/form/{authorsFiltered}/{form}",
			s.countForm,
		},

		// Get number of occ for the form, filtered by author historically
		route{
			"FormHistorical",
			"GET",
			"/formHist/{date}/{form}",
			s.countForm,
		},

		// Given a list of authors A_1, ... A_K, computes Intersection(W(A_i)) - Union(Authors - Union(A_i))
		route{
			"Intersection",
			"GET",
			"/intersection/{authors}",
			s.computeIntersection,
		},

		// Given a list of authors A_1, ... A_K, computes Intersection(W(A_i)) - Union(Authors - Union(A_i)) over a subset of the literature filtered by author
		route{
			"IntersectionFiltered",
			"GET",
			"/intersection/{authorsFilter}/{authors}",
			s.computeIntersection,
		},

		// Given a list of authors A_1, ... A_K, computes Intersection(W(A_i)) - Union(Authors - Union(A_i)) over a subset of the literature filtered by author historically
		route{
			"IntersectionHistorical",
			"GET",
			"/intersectionHist/{date}/{authors}",
			s.computeIntersection,
		},

		// Gives all the EXACT occ of a form in the literature
		route{
			"Form Occurrences",
			"GET",
			"/match/{form}",
			s.getOccurrencesForm,
		},

		// Gives all the EXACT occ of a form in the subset of the literature described
		route{
			"Form Occurrences Filtered",
			"GET",
			"/match/{authorsFilter}/{form}",
			s.getOccurrencesForm,
		},

		// Gives all the EXACT occ of a form in the subset of the literature described historically
		route{
			"Form Occurrences Historical",
			"GET",
			"/matchHist/{date}/{form}",
			s.getOccurrencesForm,
		},

		// Gives all the occ of a lemma in the literature, including the derived forms
		route{
			"Lemma Occurrence",
			"GET",
			"/find/{lemma}",
			s.getOccurrencesLemma,
		},

		// Gives all the occ of a lemma in the literature, including the derived forms filtered by author
		route{
			"Lemma Occurrence Filtered",
			"GET",
			"/find/{authorsFilter}/{lemma}",
			s.getOccurrencesLemma,
		},

		// Gives all the occ of a lemma in the literature, including the derived forms filtered by author historical
		route{
			"Lemma Occurrence Historical",
			"GET",
			"/findHist/{date}/{lemma}",
			s.getOccurrencesLemma,
		},

		route{
			"Ambiguos words",
			"GET",
			"/ambig/",
			s.getAmbiguosForms,
		},

		route{
			"Ambiguos words Filtered",
			"GET",
			"/ambig/{authorsFilter}",
			s.getAmbiguosForms,
		},

		route{
			"Ambiguos words Historical",
			"GET",
			"/ambigHist/{date}",
			s.getAmbiguosForms,
		},

		route{
			"Statistics",
			"GET",
			"/stats/",
			s.getStats,
		},

		route{
			"Statistics Filtered",
			"GET",
			"/stats/{authorsFilter}",
			s.getStats,
		},

		route{
			"Statistics Historical",
			"GET",
			"/statsHist/{date}",
			s.getStats,
		},
	}

	router := mux.NewRouter().StrictSlash(true)
	for _, route := range routes {
		var hand http.HandlerFunc
		hand = route.Handler
		hand = loggerDecorator(hand, route.Name)

		router.
			Methods(route.Method).
			Path(route.Pattern).
			Name(route.Name).
			Handler(hand)
	}

	if s.debugMode {
		attachProfiler(router)
	}

	return router
}
