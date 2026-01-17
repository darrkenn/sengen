use std::{
    fmt::{Debug, Display},
    ops::DerefMut,
    process,
    sync::Arc,
};

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{CONFIG, WordType, rates::Rates};

pub trait Word: Send + Sync + Debug {
    fn word_type() -> WordType
    where
        Self: Sized;
    fn get_word(&self) -> &str;
}

pub trait Collection<T, B>
where
    T: Word,
    B: PartialEq,
{
    fn select(&self) -> Arc<dyn Word + '_>;
    fn find_of_type(&self, r#type: &B) -> Option<&T>;
    fn calculate_thresholds(&mut self);
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum Number {
    Singular,
    Plural,
}
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum Tangibility {
    Concrete,
    Abstract,
}
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum Countability {
    Countable,
    Uncountable,
    Both,
}

// Noun
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum NounType {
    Common,
    Proper,
    Collective,
}

#[derive(Deserialize, Debug)]
pub struct Noun {
    pub word: String,
    pub r#type: NounType,
    pub number: Number,
    pub tangibility: Tangibility,
    pub countability: Countability,
}

// Impl Word for both Noun and &Noun seems stupid but its the least complicated way
impl Word for Noun {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Noun
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}
impl Word for &Noun {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Noun
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}

// Verb
#[derive(Deserialize, PartialEq, Debug, Clone)]
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

#[derive(Deserialize, Debug)]
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
    fn get_word(&self) -> &str {
        &self.word
    }
}

impl Word for &Verb {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Verb
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}

// Adverb
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum AdverbType {
    Time,
    Frequency,
    Place,
    Degree,
    Manner,
    Conjunctive,
}

#[derive(Deserialize, Debug)]
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
    fn get_word(&self) -> &str {
        &self.word
    }
}

impl Word for &Adverb {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Adverb
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}

// Adjective
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum AdjectiveType {
    Interrogative,
    Distributive,
    Numeral,
    Proper,
    Descriptive,
    Possessive,
    Quantitative,
    Demonstrative,
}

#[derive(Deserialize, Debug)]
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
    fn get_word(&self) -> &str {
        &self.word
    }
}

impl Word for &Adjective {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Adjective
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}

// Preposition
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum PrepositionType {
    Place,
    Time,
    Movement,
    Purpose,
}

#[derive(Deserialize, Debug)]
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
    fn get_word(&self) -> &str {
        &self.word
    }
}

impl Word for &Preposition {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Preposition
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}

// Determiner
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub enum DeterminerType {
    Article,
    Demonstrative,
    Distributive,
    Interrogative,
    Possessive,
    Quantifier,
    Relative,
    Negative,
}

#[derive(Deserialize, Debug)]
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
    fn get_word(&self) -> &str {
        &self.word
    }
}

impl Word for &Determiner {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Determiner
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}

// Conjunction
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub enum ConjunctionType {
    Coordinating,
    Subordinating,
    Correlative,
}
#[derive(Deserialize, Debug)]
pub struct Conjunction {
    pub word: String,
    pub r#type: ConjunctionType,
    pub pair: Option<String>,
}

impl Word for Conjunction {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Conjunction
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}

impl Word for &Conjunction {
    fn word_type() -> WordType
    where
        Self: Sized,
    {
        WordType::Conjunction
    }
    fn get_word(&self) -> &str {
        &self.word
    }
}

// Collection of nouns
#[derive(Deserialize)]
pub struct Nouns {
    pub words: Vec<Noun>,
    #[serde(skip)]
    pub thresholds: Option<[(f32, NounType); 3]>,
}
impl Collection<Noun, NounType> for Nouns {
    fn select(&self) -> Arc<dyn Word + '_> {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref noun_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(noun) = self.find_of_type(&noun_type) {
                        return Arc::new(noun);
                    } else {
                        eprintln!("No nouns of type {:?}", &noun_type);
                    };
                }
            }
        }
    }
    fn find_of_type(&self, r#type: &NounType) -> Option<&Noun> {
        self.words.iter().find(|noun| &noun.r#type == r#type)
    }
    fn calculate_thresholds(&mut self) {
        let rates = CONFIG.noun_rates.type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, NounType); 3] = [
            (rates.common, NounType::Common),
            (rates.common + rates.proper, NounType::Proper),
            (rates.common + rates.proper + rates.collective, NounType::Collective)
        ];
        self.thresholds = Some(thresholds)
    }
}

// Collection of verbs
#[derive(Deserialize)]
pub struct Verbs {
    pub words: Vec<Verb>,
    #[serde(skip)]
    pub thresholds: Option<[(f32, VerbType); 8]>,
}
impl Collection<Verb, VerbType> for Verbs {
    fn select(&self) -> Arc<dyn Word + '_> {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref verb_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(verb) = self.find_of_type(&verb_type) {
                        return Arc::new(verb);
                    } else {
                        eprintln!("No verbs of type {:?}", verb_type);
                    }
                }
            }
        }
    }
    fn find_of_type(&self, r#type: &VerbType) -> Option<&Verb> {
        self.words.iter().find(|verb| &verb.r#type == r#type)
    }
    fn calculate_thresholds(&mut self) {
        let rates = CONFIG.verb_type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, VerbType); 8] = [
            (rates.intransitive, VerbType::Intransitive),
            (rates.intransitive + rates.action, VerbType::Action),
            (rates.intransitive + rates.action + rates.transitive, VerbType::Transitive),
            (rates.intransitive + rates.action + rates.transitive + rates.linking, VerbType::Linking),
            (rates.intransitive + rates.action + rates.transitive + rates.linking + rates.modal, VerbType::Modal),
            (rates.intransitive + rates.action + rates.transitive + rates.linking + rates.modal + rates.irregular, VerbType::Irregular),
            (rates.intransitive + rates.action + rates.transitive + rates.linking + rates.modal + rates.irregular + rates.regular, VerbType::Regular),
            (rates.intransitive + rates.action + rates.transitive + rates.linking + rates.modal + rates.irregular + rates.regular + rates.auxiliary, VerbType::Auxiliary),
        ];
        self.thresholds = Some(thresholds)
    }
}

// Colletion of adverbs
#[derive(Deserialize)]
pub struct Adverbs {
    pub words: Vec<Adverb>,
    #[serde(skip)]
    pub thresholds: Option<[(f32, AdverbType); 6]>,
}
impl Collection<Adverb, AdverbType> for Adverbs {
    fn select(&self) -> Arc<dyn Word + '_> {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref adverb_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(adverb) = self.find_of_type(&adverb_type) {
                        return Arc::new(adverb);
                    } else {
                        eprintln!("No adverbs of type {:?}", adverb_type);
                    }
                }
            }
        }
    }
    fn find_of_type(&self, r#type: &AdverbType) -> Option<&Adverb> {
        self.words.iter().find(|adverb| &adverb.r#type == r#type)
    }
    fn calculate_thresholds(&mut self) {
        let rates = CONFIG.adverb_type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, AdverbType); 6] = [
            (rates.frequency, AdverbType::Frequency),
            (rates.frequency + rates.degree, AdverbType::Degree),
            (rates.frequency + rates.degree + rates.time, AdverbType::Time),
            (rates.frequency + rates.degree + rates.time + rates.manner, AdverbType::Manner),
            (rates.frequency + rates.degree + rates.time + rates.manner + rates.conjunctive, AdverbType::Conjunctive),
            (rates.frequency + rates.degree + rates.time + rates.manner + rates.conjunctive + rates.place, AdverbType::Place)
        ];
        self.thresholds = Some(thresholds)
    }
}

#[derive(Deserialize)]
pub struct Adjectives {
    pub words: Vec<Adjective>,
    #[serde(skip)]
    pub thresholds: Option<[(f32, AdjectiveType); 8]>,
}
impl Collection<Adjective, AdjectiveType> for Adjectives {
    fn select(&self) -> Arc<dyn Word + '_> {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref adjective_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(adjective) = self.find_of_type(&adjective_type) {
                        return Arc::new(adjective);
                    } else {
                        eprintln!("No adjectives of type {:?}", adjective_type)
                    }
                }
            }
        }
    }
    fn find_of_type(&self, r#type: &AdjectiveType) -> Option<&Adjective> {
        self.words.iter().find(|adj| &adj.r#type == r#type)
    }
    fn calculate_thresholds(&mut self) {
        let rates = CONFIG.adjective_type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, AdjectiveType); 8] = [
            (rates.numeral, AdjectiveType::Numeral),
            (rates.numeral + rates.interrogative, AdjectiveType::Interrogative),
            (rates.numeral + rates.interrogative + rates.distributive, AdjectiveType::Distributive),
            (rates.numeral + rates.interrogative + rates.distributive + rates.descriptive, AdjectiveType::Descriptive),
            (rates.numeral + rates.interrogative + rates.distributive + rates.descriptive + rates.possessive, AdjectiveType::Possessive),
            (rates.numeral + rates.interrogative + rates.distributive + rates.descriptive + rates.possessive + rates.demonstrative, AdjectiveType::Demonstrative),
            (rates.numeral + rates.interrogative + rates.distributive + rates.descriptive + rates.possessive + rates.demonstrative + rates.quantitative, AdjectiveType::Quantitative),
            (rates.numeral + rates.interrogative + rates.distributive + rates.descriptive + rates.possessive + rates.demonstrative + rates.quantitative + rates.proper, AdjectiveType::Proper),
        ];
        self.thresholds = Some(thresholds)
    }
}

#[derive(Deserialize)]
pub struct Prepositions {
    pub words: Vec<Preposition>,
    #[serde(skip)]
    pub thresholds: Option<[(f32, PrepositionType); 4]>,
}
impl Collection<Preposition, PrepositionType> for Prepositions {
    fn select(&self) -> Arc<dyn Word + '_> {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref preposition_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(prepositon) = self.find_of_type(preposition_type) {
                        return Arc::new(prepositon);
                    } else {
                        eprintln!("No prepositions of type {:?}", preposition_type)
                    }
                }
            }
        }
    }
    fn find_of_type(&self, r#type: &PrepositionType) -> Option<&Preposition> {
        self.words.iter().find(|pre| &pre.r#type == r#type)
    }
    fn calculate_thresholds(&mut self) {
        let rates = CONFIG.preposition_type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, PrepositionType); 4] = [
        (rates.place, PrepositionType::Place),
        (rates.place + rates.time, PrepositionType::Time),
        (rates.place + rates.time + rates.movement, PrepositionType::Movement),
        (rates.place + rates.time + rates.movement + rates.purpose, PrepositionType::Purpose)
        ];
        self.thresholds = Some(thresholds)
    }
}

#[derive(Deserialize)]
pub struct Determiners {
    pub words: Vec<Determiner>,
    #[serde(skip)]
    pub thresholds: Option<[(f32, DeterminerType); 8]>,
}
impl Collection<Determiner, DeterminerType> for Determiners {
    fn select(&self) -> Arc<dyn Word + '_> {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref determiner_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(determiner) = self.find_of_type(determiner_type) {
                        return Arc::new(determiner);
                    } else {
                        eprintln!("No determiners of type {:?}", determiner_type)
                    }
                }
            }
        }
    }
    fn find_of_type(&self, r#type: &DeterminerType) -> Option<&Determiner> {
        self.words.iter().find(|det| &det.r#type == r#type)
    }
    fn calculate_thresholds(&mut self) {
        let rates = CONFIG.determiner_type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, DeterminerType); 8] = [
        (rates.distributive, DeterminerType::Distributive),
        (rates.distributive + rates.article, DeterminerType::Article),
        (rates.distributive + rates.article + rates.demonstrative, DeterminerType::Demonstrative),
        (rates.distributive + rates.article + rates.demonstrative + rates.possessive, DeterminerType::Possessive),
        (rates.distributive + rates.article + rates.demonstrative + rates.possessive + rates.quantifier, DeterminerType::Quantifier),
        (rates.distributive + rates.article + rates.demonstrative + rates.possessive + rates.quantifier + rates.negative, DeterminerType::Negative),
        (rates.distributive + rates.article + rates.demonstrative + rates.possessive + rates.quantifier + rates.negative + rates.relative, DeterminerType::Relative),
        (rates.distributive + rates.article + rates.demonstrative + rates.possessive + rates.quantifier + rates.negative + rates.relative + rates.interrogative, DeterminerType::Interrogative)
        ];
        self.thresholds = Some(thresholds)
    }
}
#[derive(Deserialize)]
pub struct Conjunctions {
    pub words: Vec<Conjunction>,
    #[serde(skip)]
    pub thresholds: Option<[(f32, ConjunctionType); 3]>,
}
impl Collection<Conjunction, ConjunctionType> for Conjunctions {
    fn select(&self) -> Arc<dyn Word + '_> {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref conjunction_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(conjunction) = self.find_of_type(conjunction_type) {
                        return Arc::new(conjunction);
                    } else {
                        eprintln!("No conjunction of type {:?}", conjunction_type)
                    }
                }
            }
        }
    }
    fn find_of_type(&self, r#type: &ConjunctionType) -> Option<&Conjunction> {
        self.words.iter().find(|con| &con.r#type == r#type)
    }
    fn calculate_thresholds(&mut self) {
        let rates = CONFIG.conjunction_type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, ConjunctionType); 3] = [
        (rates.coordinating, ConjunctionType::Coordinating),
        (rates.coordinating + rates.subordinating, ConjunctionType::Subordinating),
        (rates.coordinating + rates.subordinating + rates.correlative, ConjunctionType::Correlative)
        ];
        self.thresholds = Some(thresholds)
    }
}

lazy_static! {
    pub static ref NOUNS: Nouns = {
        let content = include_str!("../words/nouns.toml");
        let mut nouns: Nouns = match toml::from_str(&content) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error with nouns.toml: {e:#}");
                process::exit(1);
            }
        };
        nouns.calculate_thresholds();
        nouns
    };
    pub static ref VERBS: Verbs = {
        let content = include_str!("../words/verbs.toml");
        let mut verbs: Verbs = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error with verbs.toml: {e:#}");
                process::exit(1);
            }
        };
        verbs.calculate_thresholds();
        verbs
    };
    pub static ref ADVERBS: Adverbs = {
        let content = include_str!("../words/adverbs.toml");
        let mut adverbs: Adverbs = match toml::from_str(&content) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error with adverbs.toml: {e:#}");
                process::exit(1);
            }
        };
        adverbs.calculate_thresholds();
        adverbs
    };
    pub static ref ADJECTIVES: Adjectives = {
        let content = include_str!("../words/adjectives.toml");
        let mut adjectives: Adjectives = match toml::from_str(&content) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error with adjectives.toml: {e:#}");
                process::exit(1);
            }
        };
        adjectives.calculate_thresholds();
        adjectives
    };
    pub static ref PREPOSITIONS: Prepositions = {
        let content = include_str!("../words/prepositions.toml");
        let mut prepositions: Prepositions = match toml::from_str(&content) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error with prepositions.toml: {e:#}");
                process::exit(1);
            }
        };
        prepositions.calculate_thresholds();
        prepositions
    };
    pub static ref DETERMINERS: Determiners = {
        let content = include_str!("../words/determiners.toml");
        let mut determiners: Determiners = match toml::from_str(&content) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error with determiners.toml: {e:#}");
                process::exit(1);
            }
        };
        determiners.calculate_thresholds();
        determiners
    };
    pub static ref CONJUNCTIONS: Conjunctions = {
        let content = include_str!("../words/conjunctions.toml");
        let mut conjunctions: Conjunctions = match toml::from_str(&content) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error with conjunctions.toml: {e:#}");
                process::exit(1);
            }
        };
        conjunctions.calculate_thresholds();
        conjunctions
    };
}
