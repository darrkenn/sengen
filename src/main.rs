use crate::{
    rates::{
        AdjectiveTypeRates, AdverbTypeRates, NounTypeRates, Rates, VerbTypeRates, WordTypeRates,
    },
    structures::{
        WORD_COUNT_STRUCTURE_EIGHT, WORD_COUNT_STRUCTURE_FIVE, WORD_COUNT_STRUCTURE_FOUR,
        WORD_COUNT_STRUCTURE_SEVEN, WORD_COUNT_STRUCTURE_SIX, WORD_COUNT_STRUCTURE_THREE,
    },
    words::Word,
};
use genetica::individual::Generate;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs, process};

mod chromosome;
mod rates;
mod structures;
mod words;

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_string = match fs::read_to_string("config.toml") {
            Ok(cs) => cs,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        match toml::from_str(&config_string) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        }
    };
    pub static ref STRUCTURE: &'static [WordType] = {
        if CONFIG.use_structure_fitness {
            let structure: &[WordType] = match CONFIG.word_count {
                3 => &WORD_COUNT_STRUCTURE_THREE[rand::random_range(0..=1)],
                4 => &WORD_COUNT_STRUCTURE_FOUR,
                5 => &WORD_COUNT_STRUCTURE_FIVE[rand::random_range(0..=3)],
                6 => &WORD_COUNT_STRUCTURE_SIX,
                7 => &WORD_COUNT_STRUCTURE_SEVEN,
                _ => &WORD_COUNT_STRUCTURE_EIGHT,
            };
            structure
        } else {
            &[]
        }
    };
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub generations: i32,
    pub population_count: i32,
    pub crossover_probability: f32,
    pub mutation_probability: f32,
    pub use_structure_fitness: bool,
    pub use_grammar_fitness: bool,
    pub word_count: usize,
    pub word_type_rates: WordTypeRates,
    pub noun_type_rates: NounTypeRates,
    pub verb_type_rates: VerbTypeRates,
    pub adverb_type_rates: AdverbTypeRates,
    pub adjective_type_rates: AdjectiveTypeRates,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum WordType {
    Noun,
    Verb,
    Adverb,
    Adjective,
    Conjunction,
    Preposition,
    Determiner,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !CONFIG.word_type_rates.add_up() {
        eprintln!(
            "Word type rates don't add up to be under 1.00. Total = {}",
            CONFIG.word_type_rates.total()
        );
        process::exit(1)
    }
    if !CONFIG.noun_type_rates.add_up() {
        eprintln!(
            "Noun type rates don't add up to be under 1.00. Total = {}",
            CONFIG.noun_type_rates.total()
        )
    }
    if !CONFIG.verb_type_rates.add_up() {
        eprintln!(
            "Verb type rates don't add up to be under 1.00. Total = {}",
            CONFIG.verb_type_rates.total()
        )
    }
    if !CONFIG.adverb_type_rates.add_up() {
        eprintln!(
            "Adverb type rates don't add up to be under 1.00. Total = {}",
            CONFIG.verb_type_rates.total()
        )
    }
    if !CONFIG.adjective_type_rates.add_up() {
        eprintln!(
            "Adjective type rates don't add up to be under 1.00. Total = {}",
            CONFIG.adjective_type_rates.total()
        )
    }

    Ok(())
}
