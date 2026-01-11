use lazy_static::lazy_static;
use serde::Deserialize;

use crate::WordType;

pub trait Word {
    fn word_type() -> WordType
    where
        Self: Sized;
}

pub trait Collection<T> {
    fn select(&self) -> T;
}

// Noun
#[derive(Deserialize)]
pub enum NounType {
    Common,
    Proper,
    Concrete,
    Abstract,
    Countable,
    Uncountable,
    Collective,
    Singular,
    Plural,
}

#[derive(Deserialize)]
pub struct Noun {
    pub word: String,
    pub r#type: NounType,
}

impl Word for Noun {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Noun
    }
}

// Verb
#[derive(Deserialize)]
pub enum VerbType {
    Action,
    Transitive,
    Intransitive,
    Auxiliary,
    Linking,
    Modal,
    Regular,
    Irregular,
}

#[derive(Deserialize)]
pub struct Verb {
    pub word: String,
    pub r#type: VerbType,
}

impl Word for Verb {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Verb
    }
}

// Adverb
#[derive(Deserialize)]
pub enum AdverbType {
    Time,
    Frequency,
    Place,
    Degree,
    Manner,
    Conjunctive,
}

#[derive(Deserialize)]
pub struct Adverb {
    pub word: String,
    pub r#type: AdverbType,
}

impl Word for Adverb {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Adverb
    }
}

// Adjective
#[derive(Deserialize)]
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
pub struct Adjective {
    pub word: String,
    pub r#type: AdjectiveType,
}

impl Word for Adjective {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Adjective
    }
}

// Preposition
#[derive(Deserialize)]
pub enum PrepositionType {
    Place,
    Time,
    Movement,
    Purpose,
}
#[derive(Deserialize)]
pub struct Preposition {
    pub word: String,
    pub r#type: PrepositionType,
}
impl Word for Preposition {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Preposition
    }
}

// Determiner
#[derive(Deserialize)]
pub enum DeterminerType {
    Article,
    Demonstrative,
    Distributive,
    Interrogative,
    Possessive,
    Quantative,
    Relative,
}

#[derive(Deserialize)]
pub struct Determiner {
    pub word: String,
    pub r#type: DeterminerType,
}

impl Word for Determiner {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Determiner
    }
}

// Conjunction
#[derive(Deserialize)]
pub enum ConjunctionType {
    Coordinating,
    Subordinating,
    Correlative,
}
#[derive(Deserialize)]
pub struct Conjunction {
    pub word: String,
    pub r#type: ConjunctionType,
}

impl Word for Conjunction {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Conjunction
    }
}

// Collection of words
#[derive(Deserialize)]
pub struct Nouns {
    pub words: Vec<Noun>,
}
#[derive(Deserialize)]
pub struct Verbs {
    pub words: Vec<Verb>,
}
#[derive(Deserialize)]
pub struct Adverbs {
    pub words: Vec<Adverb>,
}
#[derive(Deserialize)]
pub struct Adjectives {
    pub words: Vec<Adjective>,
}
#[derive(Deserialize)]
pub struct Prepositions {
    pub words: Vec<Preposition>,
}
#[derive(Deserialize)]
pub struct Determiners {
    pub words: Vec<Determiner>,
}
#[derive(Deserialize)]
pub struct Conjunctions {
    pub words: Vec<Conjunction>,
}

lazy_static! {
    pub static ref NOUNS: Nouns = {
        let content = include_str!("../words/nouns.toml");
        let nouns: Nouns = toml::from_str(&content).unwrap();
        nouns
    };
    pub static ref VERBS: Verbs = {
        let content = include_str!("../words/verbs.toml");
        let verbs: Verbs = toml::from_str(&content).unwrap();
        verbs
    };
    pub static ref ADVERBS: Adverbs = {
        let content = include_str!("../words/adverbs.toml");
        let adverbs: Adverbs = toml::from_str(&content).unwrap();
        adverbs
    };
    pub static ref ADJECTIVES: Adjectives = {
        let content = include_str!("../words/adjectives.toml");
        let adjectives: Adjectives = toml::from_str(&content).unwrap();
        adjectives
    };
    pub static ref PREPOSITIONS: Prepositions = {
        let content = include_str!("../words/prepositions.toml");
        let prepositions: Prepositions = toml::from_str(&content).unwrap();
        prepositions
    };
    pub static ref DETERMINERS: Determiners = {
        let content = include_str!("../words/determiners.toml");
        let determiners: Determiners = toml::from_str(&content).unwrap();
        determiners
    };
    pub static ref CONJUNCTIONS: Conjunctions = {
        let content = include_str!("../words/conjunctions.toml");
        let conjunctions: Conjunctions = toml::from_str(&content).unwrap();
        conjunctions
    };
}
