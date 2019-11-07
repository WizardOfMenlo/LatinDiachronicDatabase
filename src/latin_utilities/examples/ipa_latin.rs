use latin_utilities::ipa::*;

fn main() {
    let input = "scientia";
    println!("Classical: {}", ipa_latin(input, true));
    println!("Eccl: {}", ipa_latin(input, false));
}
