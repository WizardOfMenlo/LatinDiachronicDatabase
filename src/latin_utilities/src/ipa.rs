use lazy_static::lazy_static;

lazy_static! {
    static ref CLASSIC: Replacements = ipa_latin_repl_map(true);
    static ref ECCL: Replacements = ipa_latin_repl_map(false);
}

#[derive(Default, Clone)]
struct Replacements(Vec<(String, String)>);

impl Replacements {
    fn push(&mut self, pair: (impl ToString, impl ToString)) {
        self.0.push((pair.0.to_string(), pair.1.to_string()));
    }

    fn add_all(&mut self, base: &str, repl: &str, tails: &[&str]) {
        for poss in tails {
            self.push((format!("{}{}", base, poss), format!("{}{}", repl, poss)));
        }
    }

    fn inner(self) -> Vec<(String, String)> {
        self.0
    }
}

fn parse_and_get_str(input: &str) -> Vec<&str> {
    input.split(",").map(|s| s.trim()).collect()
}

fn ipa_latin_repl_map(classical: bool) -> Replacements {
    let mut subs = Replacements::default();

    // Weird stuff first

    if classical {
        subs.add_all("aei", "ae̯j", &parse_and_get_str("a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"));
        subs.add_all("āei", "ae̯j", &parse_and_get_str("a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"));
        subs.add_all("ăei", "ae̯j", &parse_and_get_str("a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"));
        subs.add_all("āĕi", "ae̯j", &parse_and_get_str("a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"));

        subs.add_all(
            "āēi", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăēi", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu,eŭ, ēu, ēū"
            ),
        );

        subs.add_all(
            "aeĭ", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū"
            ),
        );

        subs.add_all(
            "āeĭ", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăeĭ", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, eŭ, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āĕĭ", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āēĭ", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăēĭ", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "aeī", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, eŭ, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āeī", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăeī", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āĕī", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āēī", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăēī", "ae̯j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "oeu", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōeu", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏeu", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏēu", "oew",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōēu", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "oeŭ", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōeŭ", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏeŭ", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏēŭ", "oew",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōēŭ", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );
        subs.add_all(
            "oeū", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōeū", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏeū", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏēū", "oew",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōēū", "oe̯w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "sc", "sk",
            &parse_and_get_str(
                "e, i, ē, ī, ȳ, y, ў, ĕ, ĭ, āe, aē, ăē, aĕ, āĕ, āē, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ti", "tj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "c", "k",
            &parse_and_get_str(
                "e, i, ē, ī, ȳ, y, ў, ĕ, ĭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū, aeu, aei, oeu, āeu, āei, ōēu"
            ),
        );

        subs.add_all(
            "u", "w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŭ", "w",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );
    } else {
        subs.add_all("aei", "ɛj", &parse_and_get_str("a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"));
        subs.add_all("āei", "ɛj", &parse_and_get_str("a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"));
        subs.add_all("ăei", "ɛj", &parse_and_get_str("a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"));
        subs.add_all("āĕi", "ɛj", &parse_and_get_str("a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"));
        subs.add_all(
            "āēi", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăēi", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu,eŭ, ēu, ēū"
            ),
        );

        subs.add_all(
            "aeĭ", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū"
            ),
        );

        subs.add_all(
            "āeĭ", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăeĭ", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, eŭ, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āĕĭ", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āēĭ", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăēĭ", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "aeī", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, eŭ, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āeī", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăeī", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āĕī", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "āēī", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ăēī", "ɛj",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eŭ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "oeu", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōeu", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏeu", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏēu", "oev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōēu", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "oeŭ", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōeŭ", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏeŭ", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏēŭ", "oev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōēŭ", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );
        subs.add_all(
            "oeū", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōeū", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏeū", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ŏēū", "oev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ōēū", "ev",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "sc", "ʃ",
            &parse_and_get_str(
                "e, i, ē, ī, ȳ, y, ў, ĕ, ĭ, āe, aē, ăē, aĕ, āĕ, āē, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "ti", "tsi",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );

        subs.add_all(
            "c", "tʃ",
            &parse_and_get_str(
                "e, i, ē, ī, ȳ, y, ў, ĕ, ĭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū, aeu, aei, oeu, āeu, āei, ōēu"
            ),
        );

        subs.add_all(
            "ŭ", "u",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, ēu, ēū"
            ),
        );
    }

    subs.add_all(
            "i", "j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū, aei, āei"
            ),
        );

    subs.add_all(
            "ĭ", "j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū, aei, āei"
            ),
        );

    subs.add_all(
            "ī", "j",
            &parse_and_get_str(
                "a, e, i, o, u, ā, ē, ī, ō, ū, ȳ, ă, ĕ, ĭ, ŏ, ŭ, āe, aē, ăē, aĕ, āĕ, ōe, oē, ŏē, oĕ, ōĕ, eu, ĕu, eŭ, ēu, ēū, aei, āei"
            ),
        );

    // Common 3-length
    subs.push(("āēū", "aeu"));

    if classical {
        // Classical 3-length
        subs.push(("aei", "ae̯i"));
        subs.push(("āei", "ae̯i"));
        subs.push(("ăei", "ae̯i"));
        subs.push(("āĕi", "ae̯i"));
        subs.push(("ăēi", "ae̯i"));
        subs.push(("āēi", "aei"));

        subs.push(("aeī", "ae̯i"));
        subs.push(("āeī", "ae̯i"));
        subs.push(("ăeī", "ae̯i"));
        subs.push(("āĕī", "ae̯i"));
        subs.push(("ăēī", "ae̯i"));
        subs.push(("āēī", "aei"));

        subs.push(("aeĭ", "ae̯ɪ"));
        subs.push(("āeĭ", "ae̯ɪ"));
        subs.push(("ăeĭ", "ae̯ɪ"));
        subs.push(("āĕĭ", "ae̯ɪ"));
        subs.push(("ăēĭ", "ae̯ɪ"));
        subs.push(("āēĭ", "aeɪ"));

        subs.push(("aeu", "ae̯u"));
        subs.push(("āeu", "ae̯u"));
        subs.push(("ăeu", "ae̯u"));
        subs.push(("āĕu", "ae̯u"));
        subs.push(("ăēu", "ae̯u"));
        subs.push(("āēu", "aeu"));

        subs.push(("aeū", "ae̯u"));
        subs.push(("āeū", "ae̯u"));
        subs.push(("ăeū", "ae̯u"));
        subs.push(("āĕū", "ae̯u"));
        subs.push(("ăēū", "ae̯u"));

        subs.push(("aeŭ", "ae̯ʊ"));
        subs.push(("āeŭ", "ae̯ʊ"));
        subs.push(("ăeŭ", "ae̯ʊ"));
        subs.push(("āĕŭ", "ae̯ʊ"));
        subs.push(("ăēŭ", "ae̯ʊ"));
        subs.push(("āēŭ", "aeʊ"));

        subs.push(("oeu", "oe̯u"));
        subs.push(("ōeu", "oe̯u"));
        subs.push(("ŏeu", "oe̯u"));
        subs.push(("ōĕu", "oe̯u"));
        subs.push(("ŏēu", "oeu"));
        subs.push(("ōēu", "oe̯u"));

        subs.push(("oeū", "oe̯u"));
        subs.push(("ōeū", "oe̯u"));
        subs.push(("ŏeū", "oe̯u"));
        subs.push(("ōĕū", "oe̯u"));
        subs.push(("ŏēū", "oeu"));

        subs.push(("oeŭ", "oe̯ʊ"));
        subs.push(("ōeŭ", "oe̯ʊ"));
        subs.push(("ŏeŭ", "oe̯ʊ"));
        subs.push(("ōĕŭ", "oe̯ʊ"));
        subs.push(("ŏēŭ", "oeʊ"));
        subs.push(("ōēŭ", "oe̯ʊ"));
    } else {
        // Eccl 3-lenght
        subs.push(("aei", "ɛi"));
        subs.push(("āei", "ɛi"));
        subs.push(("ăei", "ɛi"));
        subs.push(("āĕi", "ɛi"));
        subs.push(("ăēi", "ɛi"));
        subs.push(("āēi", "ɛi"));

        subs.push(("aeī", "ɛi"));
        subs.push(("āeī", "ɛi"));
        subs.push(("ăeī", "ɛi"));
        subs.push(("āĕī", "ɛi"));
        subs.push(("ăēī", "ɛi"));
        subs.push(("āēī", "ɛi"));

        subs.push(("aeĭ", "ɛi"));
        subs.push(("āeĭ", "ɛi"));
        subs.push(("ăeĭ", "ɛi"));
        subs.push(("āĕĭ", "ɛi"));
        subs.push(("ăēĭ", "ɛi"));
        subs.push(("āēĭ", "ɛi"));

        subs.push(("aeu", "ɛu"));
        subs.push(("āeu", "ɛu"));
        subs.push(("ăeu", "ɛu"));
        subs.push(("āĕu", "ɛu"));
        subs.push(("ăēu", "ɛu"));
        subs.push(("āēu", "ɛu"));

        subs.push(("aeū", "ɛu"));
        subs.push(("āeū", "ɛu"));
        subs.push(("ăeū", "ɛu"));
        subs.push(("āĕū", "ɛu"));
        subs.push(("ăēū", "ɛu"));
        subs.push(("āēū", "ɛu"));

        subs.push(("aeŭ", "ɛu"));
        subs.push(("āeŭ", "ɛu"));
        subs.push(("ăeŭ", "ɛu"));
        subs.push(("āĕŭ", "ɛu"));
        subs.push(("ăēŭ", "ɛu"));
        subs.push(("āēŭ", "aeu"));

        subs.push(("oeu", "eu"));
        subs.push(("ōeu", "eu"));
        subs.push(("ŏeu", "eu"));
        subs.push(("ōĕu", "eu"));
        subs.push(("ŏēu", "oeu"));
        subs.push(("ōēu", "eu"));

        subs.push(("oeū", "eu"));
        subs.push(("ōeū", "eu"));
        subs.push(("ŏeū", "eu"));
        subs.push(("ōĕū", "eu"));
        subs.push(("ŏēū", "oeu"));

        subs.push(("oeŭ", "eu"));
        subs.push(("ōeŭ", "eu"));
        subs.push(("ŏeŭ", "eu"));
        subs.push(("ōĕŭ", "eu"));
        subs.push(("ŏēŭ", "oeu"));
        subs.push(("ōēŭ", "eu"));
    }

    // Common 2-length
    subs.push(("āē", "ae"));
    subs.push(("au", "au̯"));
    subs.push(("āu", "au̯"));
    subs.push(("ău", "au̯"));
    subs.push(("ăū", "au̯"));

    subs.push(("eu", "eu̯"));
    subs.push(("eŭ", "eu̯"));
    subs.push(("ēū", "eu̯"));
    subs.push(("ĕū", "eu̯"));
    subs.push(("ĕu", "eu̯"));
    subs.push(("ēu", "eu̯"));
    subs.push(("qu", "kʷ"));

    if classical {
        // Classical 2-length
        subs.push(("ae", "ae̯"));
        subs.push(("āe", "ae̯"));
        subs.push(("ăe", "ae̯"));
        subs.push(("āĕ", "ae̯"));
        subs.push(("ăē", "ae̯"));
        subs.push(("oe", "oe̯"));
        subs.push(("ōe", "oe̯"));
        subs.push(("ŏe", "oe̯"));
        subs.push(("ŏē", "oe"));
        subs.push(("ōē", "oe̯"));

        subs.push(("ch", "kʰ"));
        subs.push(("gn", "ŋ"));
        subs.push(("ph", "pʰ"));
        subs.push(("th", "tʰ"));
    } else {
        // Eccl 2-lenght
        subs.push(("ae", "ɛ"));
        subs.push(("āe", "ɛ"));
        subs.push(("ăe", "ɛ"));
        subs.push(("āĕ", "ɛ"));
        subs.push(("ăē", "ɛ"));
        subs.push(("oe", "e"));
        subs.push(("ōe", "e"));
        subs.push(("ŏe", "oe"));
        subs.push(("ŏē", "e"));
        subs.push(("ōē", "e"));

        subs.push(("ch", "k"));
        subs.push(("gn", "ɲ"));
        subs.push(("ph", "f"));
        subs.push(("th", "t"));
    }
    // Common 1-length
    subs.push(("ă", "a"));
    subs.push(("c", "k"));
    subs.push(("e", "ɛ"));
    subs.push(("ĕ", "ɛ"));
    subs.push(("ŏ", "ɔ"));
    subs.push(("ụ", "w"));
    subs.push(("x", "ks"));

    if classical {
        // Classical 1-length
        subs.push(("ā", "aː"));
        subs.push(("ē", "eː"));
        subs.push(("ī", "iː"));
        subs.push(("ō", "oː"));
        subs.push(("ū", "uː"));
        subs.push(("ȳ", "yː"));
        subs.push(("i", "ɪ"));
        subs.push(("ĭ", "ɪ"));
        subs.push(("u", "ʊ"));
        subs.push(("ŭ", "ʊ"));
        subs.push(("v", "w"));
        subs.push(("y", "ʏ"));
        subs.push(("ў", "ʏ"));
    } else {
        // Eccl 1-lenght
        subs.push(("ā", "a"));
        subs.push(("ē", "e"));
        subs.push(("ī", "i"));
        subs.push(("ō", "o"));
        subs.push(("ū", "u"));
        subs.push(("ȳ", "i"));
        subs.push(("ĭ", "i"));
        subs.push(("u", "u"));
        subs.push(("ŭ", "u"));

        subs.push(("h", ""));
        subs.push(("y", "i"));
        subs.push(("ў", "i"));
        subs.push(("z", "dz"));
    }

    subs
}

pub fn ipa_latin(s: &str, classical: bool) -> String {
    let mut res = s.to_string();
    let subs = if classical {
        CLASSIC.clone().inner()
    } else {
        ECCL.clone().inner()
    };

    for (s, e) in subs {
        res = res.replace(&s, &e);
    }

    res
}
