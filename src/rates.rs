use serde::Deserialize;

pub trait Rates {
    fn add_up(&self) -> bool;
    fn total(&self) -> f32;
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct VerbTypeRates {
    pub past: f32,
    pub present: f32,
    pub future: f32,
}

impl Rates for VerbTypeRates {
    fn add_up(&self) -> bool {
        self.total().floor() <= 1.00
    }
    fn total(&self) -> f32 {
        self.past + self.present + self.future
    }
}

#[derive(Deserialize, Debug)]
pub struct NounTypeRates {
    pub singular: f32,
    pub plural: f32,
}

impl Rates for NounTypeRates {
    fn add_up(&self) -> bool {
        self.total().floor() <= 1.00
    }
    fn total(&self) -> f32 {
        self.singular + self.plural
    }
}
