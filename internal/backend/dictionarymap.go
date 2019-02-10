package backend

// DictionaryMap stores information about distribution of words in the latin language
type DictionaryMap struct {
	internal map[string]map[string][]FormMetaData
}

// FormMetaData A struct that empasses the information about a word occurrence
type FormMetaData struct {
	Author     string
	FileName   string
	Path       string
	LineNumber int
}

func newDictionary() DictionaryMap {
	d := DictionaryMap{}
	d.internal = make(map[string]map[string][]FormMetaData)
	return d
}

func (d *DictionaryMap) addToMap(stem, form string, data FormMetaData) {
	// If mapping[stem] aldready has a mapping
	if formMap, ok := d.internal[stem]; ok {
		// If w[form] has no mappings
		if metaArr, ok := formMap[form]; ok {
			formMap[form] = append(metaArr, data)
		} else {
			formMap[form] = []FormMetaData{data}
		}
	} else {
		// Create a new map
		formMap := map[string][]FormMetaData{form: []FormMetaData{data}}
		d.internal[stem] = formMap
	}
}

// GetLemma gets a list of the found forms in the dictionary, along with data about their frequency
func (d DictionaryMap) GetLemma(stem string) map[string]int {
	// Get the forms
	w, ok := d.internal[stem]

	// Empty
	if !ok {
		return make(map[string]int)
	}

	// For each form, get the len
	res := make(map[string]int)
	for f, v := range w {
		res[f] = len(v)
	}
	return res
}

// GetKeys gets the lemmas in the dictionary
func (d DictionaryMap) GetKeys() []string {
	// Make a copy of d.internals keys
	keys := make([]string, 0, len(d.internal))
	for k := range d.internal {
		keys = append(keys, k)
	}
	return keys
}

// GetForms fetches a list of forms for a given lemma
func (d DictionaryMap) GetForms(lemma string) []string {
	vals, ok := d.internal[lemma]
	// If the lemma is not found, return a 0 length arr
	if !ok {
		return make([]string, 0)
	}

	// Get all the forms
	res := make([]string, 0, len(vals))
	for k := range vals {
		res = append(res, k)
	}
	return res
}

// GetFormData fetches the data associated with a given form and lemma
func (d DictionaryMap) GetFormData(lemma string, form string) []FormMetaData {
	return d.internal[lemma][form]
}

type stringSet map[string]struct{}

func addToAuthorMap(lemma string, data FormMetaData, authors []string, lemmaMap map[string]stringSet) bool {
	isOfAuthor := false
	for _, author := range authors {
		if data.Author == author {
			isOfAuthor = true
			lemmaMap[author][lemma] = struct{}{}
		}
	}
	return isOfAuthor
}

func intersection(lemmaMap map[string]stringSet, authors []string) stringSet {
	res := make(stringSet)
	for _, set := range lemmaMap {
		for lemma := range set {
			valid := true
			for _, author := range authors {
				if _, ok := lemmaMap[author][lemma]; !ok {
					valid = false
				}
			}
			if valid {
				res[lemma] = struct{}{}
			}
		}
	}
	return res
}

// ComputeIntersection Given a list of authors, computes all of them which have words in common, but not the literature
func (d DictionaryMap) ComputeIntersection(authors []string) []string {
	lemmaList := make(map[string]stringSet)
	for _, author := range authors {
		lemmaList[author] = stringSet{}
	}

	literature := make(stringSet)

	// Iterate over all the data
	for lemma, forms := range d.internal {
		// Skip the exception case
		if lemma == NotFound {
			continue
		}

		for _, formData := range forms {
			for _, data := range formData {
				isOfAuthor := addToAuthorMap(lemma, data, authors, lemmaList)
				if !isOfAuthor {
					literature[lemma] = struct{}{}
				}
			}
		}
	}

	// Add the exception case
	for form, dataL := range d.internal[NotFound] {
		for _, data := range dataL {
			isOfAuthor := addToAuthorMap(form, data, authors, lemmaList)
			if !isOfAuthor {
				literature[form] = struct{}{}
			}
		}
	}

	intersection := intersection(lemmaList, authors)
	resM := make(stringSet)

	for lemma := range intersection {
		if _, ok := literature[lemma]; !ok {
			resM[lemma] = struct{}{}
		}
	}

	res := make([]string, 0, len(resM))
	for k := range resM {
		res = append(res, k)
	}
	return res
}

// GetAmbiguosWords computes the set of all forms which resolve to two lemmas or more
func (d DictionaryMap) GetAmbiguosWords() map[string]struct{} {
	keys := d.GetKeys()
	formsSet := make(map[string]struct{})
	ambig := make(stringSet)
	for _, k := range keys {
		forms := d.GetForms(k)
		for _, f := range forms {
			if _, ok := formsSet[f]; ok {
				ambig[f] = struct{}{}
			} else {
				formsSet[f] = struct{}{}
			}
		}
	}

	return ambig

}

func (d DictionaryMap) FilterByAuthors(authors []string) DictionaryMap {
	newMap := newDictionary()

	authMap := make(map[string]struct{})
	for _, author := range authors {
		authMap[author] = struct{}{}
	}

	for lemma, formMap := range d.internal {
		for form, metaDataAssoc := range formMap {
			for _, metaData := range metaDataAssoc {
				if _, ok := authMap[metaData.Author]; ok {
					newMap.addToMap(lemma, form, metaData)
				}
			}
		}
	}
	return newMap
}

func JoinDictionaryMaps(maps []DictionaryMap) DictionaryMap {
	newMap := newDictionary()

	for _, mapping := range maps {
		for lemma, formMap := range mapping.internal {
			for form, metaDataAssoc := range formMap {
				metadataSeenSoFar := make(map[FormMetaData]struct{})
				for _, metaData := range metaDataAssoc {
					// If the metadata hasn't been seen so far
					if _, ok := metadataSeenSoFar[metaData]; !ok {
						newMap.addToMap(lemma, form, metaData)
					} else {
						metadataSeenSoFar[metaData] = struct{}{}
					}
				}
			}
		}
	}

	return newMap
}
