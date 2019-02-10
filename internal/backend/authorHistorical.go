package backend

import (
	"bufio"
	"errors"
	"io"
	"log"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type status int

const (
	BC      status = iota
	AC      status = iota
	Unknown status = iota
)

type TimeDescr struct {
	century int
	index   status
}

var timeRegex = regexp.MustCompile("(\\d)+[ad]")

func (t TimeDescr) ToString() string {
	if t.index == Unknown {
		return ""
	}
	res := strconv.Itoa(t.century)
	if t.index == BC {
		res += " BC"
	} else {
		res += " AC"
	}
	return res
}

func ParseTime(s string) (TimeDescr, error) {

	if !timeRegex.Match([]byte(s)) {
		return TimeDescr{0, Unknown}, errors.New("Invalid Date Format")
	}

	century := Unknown
	inner := strings.TrimSpace(s)
	if strings.Contains(s, "a") {
		century = BC
		inner = strings.TrimSuffix(inner, "a")
	} else if strings.Contains(s, "d") {
		century = AC
		inner = strings.TrimSuffix(inner, "d")
	}
	centuryNum, err := strconv.Atoi(inner)
	if err != nil {
		return TimeDescr{0, Unknown}, errors.New("Invalid Date Format")
	}

	return TimeDescr{
		centuryNum,
		century,
	}, nil
}

func before(first, second TimeDescr) bool {

	if first.index == Unknown || second.index == Unknown {
		return false
	}

	if first.index == BC && second.index == AC {
		return true
	}

	if first.index == AC && second.index == BC {
		return false
	}

	// Both are after christ
	if first.index == AC {
		return first.century <= second.century
	}

	// BC are opposite way around
	return second.century <= first.century
}

type TimeSpan struct {
	beginning TimeDescr
	end       TimeDescr
}

func nullTS() TimeSpan {
	return TimeSpan{
		TimeDescr{0, Unknown},
		TimeDescr{0, Unknown},
	}
}

var fullMatching = regexp.MustCompile("\\(((\\?)|(\\d+[ad](,\\s*\\d+[ad])*))\\)")

func NewTimeSpan(s string) (TimeSpan, error) {
	if !fullMatching.Match([]byte(s)) {
		return nullTS(), errors.New("Could not match")
	}
	inner := strings.Replace(strings.Replace(s, "(", "", -1), ")", "", -1)

	if strings.Contains(inner, "?") {
		return nullTS(), nil
	}

	if strings.Contains(inner, ",") {
		parts := strings.Split(inner, ",")
		if len(parts) != 2 {
			return nullTS(), errors.New("Invalid Format")
		}

		// We ignore the error because the regex takes care of it
		start, _ := ParseTime(parts[0])
		end, _ := ParseTime(parts[1])

		if !before(start, end) {
			return nullTS(), errors.New("Invalid Ordering")
		}

		return TimeSpan{
			start,
			end,
		}, nil
	}

	// Same as above
	end, _ := ParseTime(inner)
	return TimeSpan{end, end}, nil

}

func (t TimeSpan) Between() []TimeDescr {
	// TODO Add proper testing!!!
	if t.beginning.index == t.end.index && t.beginning.century == t.end.century {
		return []TimeDescr{t.beginning}
	}

	if before(t.end, t.beginning) ||
		t.beginning.index == Unknown ||
		t.end.index == Unknown ||
		t.beginning.index > t.end.index {
		return []TimeDescr{}
	}

	result := []TimeDescr{}

	if t.beginning.index == BC && t.end.index == AC {
		// Add 5BC, 4BC, ... 1BC
		for i := t.beginning.century; i > 0; i-- {
			result = append(result, TimeDescr{i, BC})
		}

		// Add 1AC, 2AC, ... 5AC
		for i := 0; i <= t.end.century; i++ {
			result = append(result, TimeDescr{i, AC})
		}
	} else if t.beginning.index == AC {
		for i := t.beginning.century; i <= t.end.century; i++ {
			result = append(result, TimeDescr{i, AC})
		}
	} else if t.end.index == BC {
		for i := t.end.century; i < t.beginning.century; i++ {
			result = append(result, TimeDescr{i, BC})
		}
	}

	sort.Slice(result, func(i, j int) bool { return before(result[i], result[j]) })

	return result
}

type AuthorHistorical struct {
	mapping map[string]TimeSpan
}

func nullAuth() AuthorHistorical {
	return AuthorHistorical{
		make(map[string]TimeSpan),
	}
}

func NewAuthorHistorical(r io.Reader) (AuthorHistorical, error) {
	mapping := make(map[string]TimeSpan)

	reader := bufio.NewScanner(r)
	for reader.Scan() {
		line := reader.Text()
		parts := strings.Split(line, "#")
		if len(parts) != 2 {
			log.Println("Invalid Line:", line)
			continue
		}
		auth := strings.TrimSpace(parts[0])
		ts := strings.TrimSpace(parts[1])

		span, err := NewTimeSpan(ts)
		if err != nil {
			log.Println("Error: ", err, "at:", line)
			continue
		}

		mapping[auth] = span
	}

	return AuthorHistorical{mapping}, nil
}

func (hist AuthorHistorical) GetAllBeforeMatching(date TimeDescr) []string {
	// Estimate a good matching ratio
	authors := make([]string, 0, len(hist.mapping)/2)
	for auth, span := range hist.mapping {
		if before(span.beginning, date) {
			authors = append(authors, auth)
		}
	}
	return authors
}
