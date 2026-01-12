use crate::{
    chromosome::Chromosome,
    rates::{
        AdjectiveTypeRates, AdverbTypeRates, ConjunctionTypeRates, DeterminerTypeRates,
        NounTypeRates, PrepositionTypeRates, Rates, VerbTypeRates, WordTypeRates,
    },
    structures::{
        WORD_COUNT_STRUCTURE_EIGHT, WORD_COUNT_STRUCTURE_FIVE, WORD_COUNT_STRUCTURE_FOUR,
        WORD_COUNT_STRUCTURE_SEVEN, WORD_COUNT_STRUCTURE_SIX, WORD_COUNT_STRUCTURE_THREE,
    },
    words::{Collection, NOUNS, Word},
};
use genetica::{
    crossover::dynamic_length_single_point_crossover,
    individual::{Generate, Individual},
    population::{self, generate_population, sort_population_descending},
};
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
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
    pub static ref WORD_THRESHOLDS: [(f32, WordType); 7] = {
        let rates = CONFIG.word_type_rates;
        #[rustfmt::skip]
        let thresholds: [(f32, WordType); 7] = [
        (rates.adverb, WordType::Adverb),
        (rates.adverb + rates.noun, WordType::Noun),
        (rates.adverb + rates.noun + rates.verb, WordType::Verb),
        (rates.adverb + rates.noun + rates.verb + rates.preposition, WordType::Preposition),
        (rates.adverb + rates.noun + rates.verb + rates.preposition + rates.conjunction, WordType::Conjunction),
        (rates.adverb + rates.noun + rates.verb + rates.preposition + rates.conjunction + rates.determiner, WordType::Determiner),
        (rates.adverb + rates.noun + rates.verb + rates.preposition + rates.conjunction + rates.determiner + rates.adjective, WordType::Adjective)
        ];
        thresholds
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
    pub preposition_type_rates: PrepositionTypeRates,
    pub determiner_type_rates: DeterminerTypeRates,
    pub conjunction_type_rates: ConjunctionTypeRates,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordType {
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
        );
        process::exit(1)
    }
    if !CONFIG.verb_type_rates.add_up() {
        eprintln!(
            "Verb type rates don't add up to be under 1.00. Total = {}",
            CONFIG.verb_type_rates.total()
        );
        process::exit(1)
    }
    if !CONFIG.adverb_type_rates.add_up() {
        eprintln!(
            "Adverb type rates don't add up to be under 1.00. Total = {}",
            CONFIG.verb_type_rates.total()
        );
        process::exit(1)
    }
    if !CONFIG.adjective_type_rates.add_up() {
        eprintln!(
            "Adjective type rates don't add up to be under 1.00. Total = {}",
            CONFIG.adjective_type_rates.total()
        );
        process::exit(1)
    }
    if !CONFIG.preposition_type_rates.add_up() {
        eprintln!(
            "Preposition type rates don't add up to be under 1.00. Total = {}",
            CONFIG.preposition_type_rates.total()
        );
        process::exit(1)
    }
    if !CONFIG.determiner_type_rates.add_up() {
        eprintln!(
            "Determiner type rates don't add up to be under 1.00. Total = {}",
            CONFIG.determiner_type_rates.total()
        );
        process::exit(1)
    }
    if !CONFIG.conjunction_type_rates.add_up() {
        eprintln!(
            "Conjunction type rate don't add up to be under 1.00. Total = {}",
            CONFIG.conjunction_type_rates.total()
        );
        process::exit(1)
    }

    let mut population: Vec<Chromosome> = generate_population(CONFIG.population_count);

    for _ in 0..CONFIG.generations {
        population
            .par_iter_mut()
            .for_each(|c| c.calculate_fitness());
        sort_population_descending(&mut population);
        let parent1 = &population[0];
        let parent2 = &population[1];
        let (mut child1, mut child2) =
            dynamic_length_single_point_crossover(parent1, parent2, CONFIG.crossover_probability);
        child1.mutate_genes();
        child2.mutate_genes();

        let mut new_population: Vec<Chromosome> = generate_population(CONFIG.population_count - 4);

        new_population.push(child1);
        new_population.push(child2);
        new_population.push(parent1.clone());
        new_population.push(parent2.clone());
        population = new_population;
    }

    sort_population_descending(&mut population);
    let best = &population[0];
    println!("{best:?}");

    Ok(())
}
