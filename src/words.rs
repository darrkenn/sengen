use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{CONFIG, WordType};

pub trait Word {
    fn word_type() -> WordType
    where
        Self: Sized;
}

pub trait Collection<T, B>
where
    T: Word,
    B: PartialEq,
{
    fn select(&self) -> &T;
    fn find_of_type(&self, r#type: &B) -> Option<&T>;
    fn calculate_thresholds(&mut self);
}

// Noun
#[derive(Deserialize, PartialEq, Debug, Clone)]
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
#[derive(Deserialize, PartialEq, Debug, Clone)]
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
#[derive(Deserialize, PartialEq, Debug, Clone)]
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
#[derive(Deserialize, PartialEq, Debug, Clone)]
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
#[derive(Deserialize, PartialEq, Clone)]
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
#[derive(Deserialize, PartialEq, Clone)]
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

// Collection of nouns
#[derive(Deserialize)]
pub struct Nouns {
    pub words: Vec<Noun>,
    #[serde(skip)]
    pub thresholds: Option<[(f32, NounType); 9]>,
}
impl Collection<Noun, NounType> for Nouns {
    fn select(&self) -> &Noun {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref noun_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(noun) = self.find_of_type(&noun_type) {
                        return noun;
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
        let rates = CONFIG.noun_type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, NounType); 9] = [
            (rates.singular, NounType::Singular),
            (rates.singular + rates.r#abstract, NounType::Abstract),
            (rates.singular + rates.r#abstract + rates.proper, NounType::Proper),
            (rates.singular + rates.r#abstract + rates.proper + rates.concrete,NounType::Concrete,),
            (rates.singular + rates.r#abstract + rates.proper + rates.concrete + rates.uncountable, NounType::Uncountable),
            (rates.singular + rates.r#abstract + rates.proper + rates.concrete + rates.uncountable + rates.common, NounType::Common),
            (rates.singular + rates.r#abstract + rates.proper + rates.concrete + rates.uncountable + rates.common + rates.collective,NounType::Collective),
            (rates.singular + rates.r#abstract + rates.proper + rates.concrete + rates.uncountable + rates.common + rates.collective + rates.plural, NounType::Plural),
            (rates.singular + rates.r#abstract + rates.proper + rates.concrete + rates.uncountable + rates.common + rates.collective + rates.plural + rates.countable,NounType::Countable),
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
    fn select(&self) -> &Verb {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref verb_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(verb) = self.find_of_type(&verb_type) {
                        return verb;
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
    fn select(&self) -> &Adverb {
        let thresholds = self.thresholds.as_ref().unwrap();
        loop {
            let random_f32: f32 = rand::random_range(0.00..1.00);
            for &(threshold, ref adverb_type) in thresholds {
                if random_f32 <= threshold {
                    if let Some(adverb) = self.find_of_type(&adverb_type) {
                        return adverb;
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
