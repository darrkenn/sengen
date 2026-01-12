use serde::Deserialize;

use crate::words::Noun;

pub trait Rates {
    fn add_up(&self) -> bool;
    fn total(&self) -> f32;
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct WordTypeRates {
    pub noun: f32,
    pub verb: f32,
    pub adverb: f32,
    pub adjective: f32,
    pub preposition: f32,
    pub determiner: f32,
    pub conjunction: f32,
}

impl Rates for WordTypeRates {
    fn add_up(&self) -> bool {
        self.total().floor() <= 1.00
    }
    fn total(&self) -> f32 {
        self.noun
            + self.verb
            + self.adverb
            + self.adjective
            + self.preposition
            + self.determiner
            + self.conjunction
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct NounTypeRates {
    pub common: f32,
    pub proper: f32,
    pub concrete: f32,
    pub r#abstract: f32,
    pub countable: f32,
    pub uncountable: f32,
    pub collective: f32,
    pub singular: f32,
    pub plural: f32,
}
impl Rates for NounTypeRates {
    fn add_up(&self) -> bool {
        self.total().floor() <= 1.00
    }
    fn total(&self) -> f32 {
        self.common
            + self.proper
            + self.concrete
            + self.r#abstract
            + self.countable
            + self.uncountable
            + self.collective
            + self.singular
            + self.plural
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct AdjectiveTypeRates {
    pub interrogative: f32,
    pub distributive: f32,
    pub numeral: f32,
    pub proper: f32,
    pub descriptive: f32,
    pub possessive: f32,
    pub quantative: f32,
    pub demonstrative: f32,
}

impl Rates for AdjectiveTypeRates {
    fn add_up(&self) -> bool {
        self.total().floor() <= 1.00
    }
    fn total(&self) -> f32 {
        self.interrogative
            + self.distributive
            + self.numeral
            + self.proper
            + self.descriptive
            + self.possessive
            + self.quantative
            + self.demonstrative
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct VerbTypeRates {
    pub action: f32,
    pub transitive: f32,
    pub intransitive: f32,
    pub auxiliary: f32,
    pub linking: f32,
    pub modal: f32,
    pub regular: f32,
    pub irregular: f32,
}

impl Rates for VerbTypeRates {
    fn add_up(&self) -> bool {
        self.total().floor() <= 1.00
    }
    fn total(&self) -> f32 {
        self.action
            + self.transitive
            + self.intransitive
            + self.auxiliary
            + self.linking
            + self.modal
            + self.regular
            + self.irregular
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct AdverbTypeRates {
    pub time: f32,
    pub frequency: f32,
    pub place: f32,
    pub degree: f32,
    pub manner: f32,
    pub conjunctive: f32,
}

impl Rates for AdverbTypeRates {
    fn add_up(&self) -> bool {
        self.total().floor() <= 1.00
    }
    fn total(&self) -> f32 {
        self.time + self.frequency + self.place + self.degree + self.manner + self.conjunctive
    }
}
