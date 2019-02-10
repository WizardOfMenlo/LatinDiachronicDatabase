package server

import (
	"strings"

	"github.com/WizardOfMenlo/LatinDiachronicDatabase/internal/backend"
)

type FilteringCriteria struct {
	authors []string
	all     bool
}

func NewFilteringAll() FilteringCriteria {
	return FilteringCriteria{make([]string, 0), true}
}

func NewNoneFilter() FilteringCriteria {
	return FilteringCriteria{make([]string, 0), false}
}

func ParseFilter(filter string) FilteringCriteria {
	auth := strings.Split(filter, ",")
	if len(auth) == 0 {
		return NewFilteringAll()
	}
	return FilteringCriteria{auth, false}
}

func FromRequest(authKey, dateKey string, vars map[string]string, hist backend.AuthorHistorical) FilteringCriteria {
	criterionA := NewFilteringAll()
	if val, ok := vars[authKey]; ok {
		criterionA = ParseFilter(val)
	}

	criterionB := NewFilteringAll()
	if val, ok := vars[dateKey]; ok {
		time, err := backend.ParseTime(val)
		if err != nil {
			return NewNoneFilter()
		}
		criterionB = FromHistorigram(hist, time)
	}

	return criterionA.Join(criterionB)
}

func FromHistorigram(hist backend.AuthorHistorical, date backend.TimeDescr) FilteringCriteria {
	authors := hist.GetAllBeforeMatching(date)
	return FilteringCriteria{authors, false}
}

func (f FilteringCriteria) Join(other FilteringCriteria) FilteringCriteria {
	// Takes care of T,T T,F
	if f.all {
		return other
	}

	// Takes care of F,T
	if other.all {
		return f
	}

	// Add all of the first
	authorsSet := make(map[string]struct{})
	for _, auth := range f.authors {
		authorsSet[auth] = struct{}{}
	}

	// Take all of the second if in the first
	res := make(map[string]struct{})
	for _, auth := range other.authors {
		if _, ok := authorsSet[auth]; ok {
			res[auth] = struct{}{}
		}
	}

	// Copy to []string
	authors := make([]string, 0, len(res))
	for k := range res {
		authors = append(authors, k)
	}

	return FilteringCriteria{authors, false}
}

func (f FilteringCriteria) apply(s backend.DictionaryMap) backend.DictionaryMap {
	if f.all {
		return s
	}

	union := s.FilterByAuthors(f.authors)
	return union
}
