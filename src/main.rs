use crate::{
    chromosome::Chromosome,
    rates::{
        AdjectiveTypeRates, AdverbTypeRates, ConjunctionTypeRates, DeterminerTypeRates, NounRates,
        NounTypeRates, PrepositionTypeRates, Rates, VerbTypeRates, WordTypeRates, check_rates,
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
use std::{fs, ops::Deref, process, sync::Arc};

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
    pub noun_rates: NounRates,
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
    check_rates("Word type", &CONFIG.word_type_rates);
    check_rates("Noun type", &CONFIG.noun_rates.type_rates);
    check_rates("Noun tangibility", &CONFIG.noun_rates.tangibility_rates);
    check_rates("Noun countability", &CONFIG.noun_rates.countability_rates);
    check_rates("Verb type", &CONFIG.verb_type_rates);
    check_rates("Adverb type", &CONFIG.adverb_type_rates);
    check_rates("Adjective type", &CONFIG.adjective_type_rates);
    check_rates("Preposition type", &CONFIG.preposition_type_rates);
    check_rates("Determiner type", &CONFIG.determiner_type_rates);
    check_rates("Conjunction type", &CONFIG.conjunction_type_rates);

    let mut population: Vec<Chromosome> = generate_population(CONFIG.population_count);

    for _ in 0..CONFIG.generations {
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
        population
            .par_iter_mut()
            .for_each(|c| c.calculate_fitness());
        sort_population_descending(&mut population);
    }

    sort_population_descending(&mut population);
    let best = &population[0];
    let sentence = construct_sentence(best);
    println!("{sentence}");

    Ok(())
}

fn construct_sentence(chromosome: &Chromosome) -> String {
    let mut words = chromosome
        .genes
        .iter()
        .map(|g| g.word.get_word().to_string())
        .collect::<Vec<String>>();
    words[0] = capitalize(&words[0]);
    let mut sentence = words.join(" ");
    sentence.push_str(".");
    sentence
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(fl) => {
            let mut capitalized_word = String::new();
            capitalized_word.push(fl.to_ascii_uppercase());
            capitalized_word.extend(chars);
            capitalized_word
        }
    }
}
