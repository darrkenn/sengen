use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Noun {
    pub singular: String,
    pub plural: Option<String>,
}
#[derive(Deserialize)]
struct Nouns {
    pub nouns: Vec<Noun>,
}

lazy_static! {
    pub static ref NOUNS: Vec<Noun> = {
        let content = include_str!("../words/nouns.toml");
        let nouns: Nouns = toml::from_str(&content).unwrap();
        nouns.nouns
    };
    pub static ref NOUNS_COUNT: usize = NOUNS.len();
    pub static ref VERBS: Vec<String> = {
        let content = include_str!("../words/verbs.txt");
        let verbs_vec = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();
        verbs_vec
    };
    pub static ref VERBS_COUNT: usize = VERBS.len();
    pub static ref ADVERBS: Vec<String> = {
        let content = include_str!("../words/adverbs.txt");
        let adverbs_vec = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();
        adverbs_vec
    };
    pub static ref ADVERBS_COUNT: usize = ADVERBS.len();
    pub static ref ADJECTIVES: Vec<String> = {
        let content = include_str!("../words/adjectives.txt");
        let adjectives_vec = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();
        adjectives_vec
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
