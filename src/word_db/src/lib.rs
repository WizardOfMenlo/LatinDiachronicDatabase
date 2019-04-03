use latin_utilities::NormalizedLatinString;

pub mod form_data;
pub mod ids;
pub mod interner;

// Strong typedefs for more intuitive api
pub type Form = NormalizedLatinString;
pub type Lemma = NormalizedLatinString;
