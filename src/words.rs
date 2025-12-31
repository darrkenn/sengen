use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub enum AdjectiveType {
    Interrogative,
    Distributive,
    Numeral,
    Proper,
    Descriptive,
    Possessive,
    Quantative,
    Demonstrative,
}

#[derive(Deserialize)]
pub struct Noun {
    pub singular: String,
    pub plural: Option<String>,
    pub proper: bool,
}
#[derive(Deserialize)]
pub struct Verb {
    pub past: String,
    pub present: String,
    pub future: String,
}
#[derive(Deserialize)]
pub struct Adjective {
    pub word: String,
    pub adjective_type: AdjectiveType,
}

#[derive(Deserialize)]
struct Nouns {
    pub nouns: Vec<Noun>,
}
#[derive(Deserialize)]
struct Verbs {
    pub verbs: Vec<Verb>,
}
#[derive(Deserialize)]
struct Adverbs {
    pub adverbs: Vec<String>,
}
#[derive(Deserialize)]
struct Adjectives {
    pub adjectives: Vec<Adjective>,
}

lazy_static! {
    pub static ref NOUNS: Vec<Noun> = {
        let content = include_str!("../words/nouns.toml");
        let nouns: Nouns = toml::from_str(&content).unwrap();
        nouns.nouns
    };
    pub static ref NOUNS_COUNT: usize = NOUNS.len();
    pub static ref VERBS: Vec<Verb> = {
        let content = include_str!("../words/verbs.toml");
        let verbs: Verbs = toml::from_str(&content).unwrap();
        verbs.verbs
    };
    pub static ref VERBS_COUNT: usize = VERBS.len();
    pub static ref ADVERBS: Vec<String> = {
        let content = include_str!("../words/adverbs.toml");
        let adverbs: Adverbs = toml::from_str(&content).unwrap();
        adverbs.adverbs
    };
    pub static ref ADVERBS_COUNT: usize = ADVERBS.len();
    pub static ref ADJECTIVES: Vec<Adjective> = {
        let content = include_str!("../words/adjectives.toml");
        let adjectives: Adjectives = toml::from_str(&content).unwrap();
        adjectives.adjectives
    };
    pub static ref ADJECTIVES_COUNT: usize = ADJECTIVES.len();
}

pub const CONJUNCTIONS_COUNT: usize = 11;
pub const CONJUNCTIONS: [&str; CONJUNCTIONS_COUNT] = [
    "and", "then", "after", "because", "but", "so", "while", "though", "for", "or", "yet",
];

pub const DETERMINERS_COUNT: usize = 10;
pub const DETERMINERS: [&str; DETERMINERS_COUNT] = [
    "the", "a", "an", "this", "that", "these", "those", "some", "many", "few",
];

pub const PREPOSITIONS_COUNT: usize = 8;
pub const PREPOSITIONS: [&str; PREPOSITIONS_COUNT] = [
    "in", "on", "at", "under", "behind", "before", "after", "near",
];
