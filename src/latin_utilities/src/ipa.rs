use lazy_static::lazy_static;

lazy_static! {
    static ref FIRST_ROUND_CLASSIC: Vec<(&'static str, &'static str)> = {
        let mut res = Vec::new();
        res.push(("tia", "tja"));
        res.push(("tie", "tje"));
        res.push(("tio", "tjo"));
        res.push(("tii", "tji"));
        res.push(("tiu", "tju"));
        res.push(("sci", "ski"));
        res.push(("sce", "ske"));
        res
    };
    static ref FIRST_ROUND_ECCL: Vec<(&'static str, &'static str)> = {
        let mut res = Vec::new();
        res.push(("cae", "tʃae"));
        res.push(("coe", "tʃoe"));
        res.push(("tia", "tsia"));
        res.push(("tie", "tsie"));
        res.push(("tio", "tsio"));
        res.push(("tii", "tsii"));
        res.push(("tiu", "tsiu"));
        res.push(("sci", "ʃi"));
        res.push(("sce", "ʃe"));
        res
    };
    static ref FIRST_ROUND_COMMON: Vec<(&'static str, &'static str)> = {
        let mut res = Vec::new();

        res.push(("au", "au̯"));
        res.push(("eu", "eu̯"));
        res.push(("ui", "ui̯"));
        res.push(("ia", "ja"));
        res.push(("ie", "je"));
        res.push(("io", "jo"));
        res.push(("iu", "ju"));
        res.push(("ii", "ji"));
        res.push(("ph", "pʰ"));
        res.push(("qu", "kʷ"));

        res
    };
    static ref SECOND_ROUND_CLASSIC: Vec<(&'static str, &'static str)> = {
        let mut res = Vec::new();
        res.push(("ae", "ae̯"));
        res.push(("ch", "kʰ"));
        res.push(("gn", "ŋ"));
        res.push(("oe", "oe̯"));
        res.push(("th", "tʰ"));

        res.push(("ua", "wa"));
        res.push(("ue", "we"));
        res.push(("uo", "wo"));
        res.push(("ui", "wi"));
        res.push(("uu", "wu"));

        res.push(("c", "k"));
        res.push(("ā", "aː"));

        res.push(("e", "ɛ"));
        res.push(("ē", "eː"));
        res.push(("ī", "iː"));
        res.push(("ō", "oː"));
        res.push(("u", "ʊ"));
        res.push(("ū", "uː"));
        res.push(("ȳ", "yː"));

        res.push(("y", "ʏ"));

        res
    };
    static ref SECOND_ROUND_ECCL: Vec<(&'static str, &'static str)> = {
        let mut res = Vec::new();
        res.push(("ci", "tʃi"));
        res.push(("cy", "tʃy"));
        res.push(("ch", "k"));
        res.push(("ae", "e"));
        res.push(("gn", "ɲ"));
        res.push(("oe", "e"));
        res.push(("th", "t"));

        res.push(("ua", "va"));
        res.push(("ue", "ve"));
        res.push(("uo", "vo"));
        res.push(("ui", "vi"));
        res.push(("uu", "vu"));

        res.push(("k", "k"));
        res.push(("c", "k"));
        res.push(("ā", "a"));
        res.push(("ē", "e"));
        res.push(("ī", "i"));
        res.push(("ō", "o"));
        res.push(("ū", "u"));
        res.push(("ȳ", "i"));

        res.push(("z", "dz"));
        res.push(("y", "i"));

        res
    };
}

pub fn ipa_latin(s: &str, classical: bool) -> String {
    let mut res = s.to_string();

    if classical {
        for (a, b) in FIRST_ROUND_CLASSIC.iter() {
            res = res.replace(a, b);
        }
    } else {
        for (a, b) in FIRST_ROUND_ECCL.iter() {
            res = res.replace(a, b);
        }
    }

    for (a, b) in FIRST_ROUND_COMMON.iter() {
        res = res.replace(a, b);
    }

    if classical {
        for (a, b) in SECOND_ROUND_CLASSIC.iter() {
            res = res.replace(a, b);
        }
    } else {
        for (a, b) in SECOND_ROUND_ECCL.iter() {
            res = res.replace(a, b);
        }
    }

    res
}
