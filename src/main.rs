use std::{array, fs, sync::OnceLock};
mod structures;
mod words;
use genetica::{
    crossover::single_point_crossover,
    individual::{Generate, Individual, Mutate},
    population::{generate_population, sort_population_descending},
};
use nlprule::{Rules, Tokenizer, rules_filename, tokenizer_filename};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use serde::Deserialize;

use crate::{
    structures::{
        WORD_COUNT_STRUCTURE_EIGHT, WORD_COUNT_STRUCTURE_FIVE, WORD_COUNT_STRUCTURE_FOUR,
        WORD_COUNT_STRUCTURE_SEVEN, WORD_COUNT_STRUCTURE_SIX, WORD_COUNT_STRUCTURE_THREE,
    },
    words::{
        ADJECTIVES, ADJECTIVES_COUNT, ADVERBS, ADVERBS_COUNT, CONJUNCTIONS, CONJUNCTIONS_COUNT,
        DETERMINERS, DETERMINERS_COUNT, NOUNS, NOUNS_COUNT, PREPOSITIONS, PREPOSITIONS_COUNT,
        VERBS, VERBS_COUNT,
    },
};

static TOKENIZER: OnceLock<Tokenizer> = OnceLock::new();
static RULES: OnceLock<Rules> = OnceLock::new();
static STRUCTURE: OnceLock<&[WordType]> = OnceLock::new();

const PC: f32 = 0.6;
const PM: f32 = 0.05;

const WORD_COUNT: usize = 4;

const NOUN_RATE: f32 = 0.30;
const VERB_RATE: f32 = 0.20;
const ADVERB_RATE: f32 = 0.10;
const ADJECTIVE_RATE: f32 = 0.10;
const PREPOSITION_RATE: f32 = 0.10;
const DETERMINER_RATE: f32 = 0.10;

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct GeneType<'a>(pub &'a str, pub WordType);

impl<'a> Generate for GeneType<'a> {
    fn generate() -> Self {
        let random_f32: f32 = rand::random_range(0.00..1.00);
        //This seems stupid
        let (word, word_type) = if random_f32 <= NOUN_RATE {
            (
                NOUNS[rand::random_range(0..*NOUNS_COUNT)].as_str(),
                WordType::Noun,
            )
        } else if random_f32 <= (NOUN_RATE + VERB_RATE) {
            (
                VERBS[rand::random_range(0..*VERBS_COUNT)].as_str(),
                WordType::Verb,
            )
        } else if random_f32 <= (NOUN_RATE + VERB_RATE + ADVERB_RATE) {
            (
                ADVERBS[rand::random_range(0..*ADVERBS_COUNT)].as_str(),
                WordType::Adverb,
            )
        } else if random_f32 <= (NOUN_RATE + VERB_RATE + ADVERB_RATE + ADJECTIVE_RATE) {
            (
                ADJECTIVES[rand::random_range(0..*ADJECTIVES_COUNT)].as_str(),
                WordType::Adjective,
            )
        } else if random_f32
            <= (NOUN_RATE + VERB_RATE + ADVERB_RATE + ADJECTIVE_RATE + PREPOSITION_RATE)
        {
            (
                PREPOSITIONS[rand::random_range(0..PREPOSITIONS_COUNT)],
                WordType::Preposition,
            )
        } else if random_f32
            <= (NOUN_RATE
                + VERB_RATE
                + ADVERB_RATE
                + ADJECTIVE_RATE
                + PREPOSITION_RATE
                + DETERMINER_RATE)
        {
            (
                DETERMINERS[rand::random_range(0..DETERMINERS_COUNT)],
                WordType::Determiner,
            )
        } else {
            (
                CONJUNCTIONS[rand::random_range(0..CONJUNCTIONS_COUNT)],
                WordType::Conjunction,
            )
        };
        GeneType(word, word_type)
    }
}

impl<'a> Mutate for GeneType<'a> {
    fn mutate(&mut self) {
        if rand::random_range(0.00..1.00) <= PM {
            let random_f32: f32 = rand::random_range(0.00..1.00);

            let (word, word_type) = if random_f32 <= NOUN_RATE {
                (
                    NOUNS[rand::random_range(0..*NOUNS_COUNT)].as_str(),
                    WordType::Noun,
                )
            } else if random_f32 <= (NOUN_RATE + VERB_RATE) {
                (
                    VERBS[rand::random_range(0..*VERBS_COUNT)].as_str(),
                    WordType::Verb,
                )
            } else if random_f32 <= (NOUN_RATE + VERB_RATE + ADVERB_RATE) {
                (
                    ADVERBS[rand::random_range(0..*ADVERBS_COUNT)].as_str(),
                    WordType::Adverb,
                )
            } else if random_f32 <= (NOUN_RATE + VERB_RATE + ADVERB_RATE + ADJECTIVE_RATE) {
                (
                    ADJECTIVES[rand::random_range(0..*ADJECTIVES_COUNT)].as_str(),
                    WordType::Adjective,
                )
            } else if random_f32
                <= (NOUN_RATE + VERB_RATE + ADVERB_RATE + ADJECTIVE_RATE + PREPOSITION_RATE)
            {
                (
                    PREPOSITIONS[rand::random_range(0..PREPOSITIONS_COUNT)],
                    WordType::Preposition,
                )
            } else if random_f32
                <= (NOUN_RATE
                    + VERB_RATE
                    + ADVERB_RATE
                    + ADJECTIVE_RATE
                    + PREPOSITION_RATE
                    + DETERMINER_RATE)
            {
                (
                    DETERMINERS[rand::random_range(0..DETERMINERS_COUNT)],
                    WordType::Determiner,
                )
            } else {
                (
                    CONJUNCTIONS[rand::random_range(0..CONJUNCTIONS_COUNT)],
                    WordType::Conjunction,
                )
            };

            self.0 = word;
            self.1 = word_type;
        };
    }
}

#[derive(Debug, Clone, Copy)]
struct Chromosome<'a> {
    genes: [GeneType<'a>; WORD_COUNT],
    fitness: Option<f32>,
}

impl<'a> Individual for Chromosome<'a> {
    type GeneType = GeneType<'a>;
    const GENES_SIZE: usize = WORD_COUNT;
    fn new() -> Self {
        let genes: [GeneType; WORD_COUNT] = array::from_fn(|_| GeneType::generate());
        Chromosome {
            genes,
            fitness: None,
        }
    }

    fn mutate_genes(&mut self) {
        for gene in &mut self.genes {
            gene.mutate();
        }
    }
    fn genes(&self) -> &[Self::GeneType] {
        &self.genes
    }
    fn genes_mut(&mut self) -> &mut [Self::GeneType] {
        &mut self.genes
    }

    fn fitness(&self) -> Option<f32> {
        self.fitness
    }
    fn fitness_mut(&mut self) -> &mut Option<f32> {
        &mut self.fitness
    }

    fn calculate_fitness(&mut self) {
        let tokenizer = TOKENIZER.get().unwrap();
        let rules = RULES.get().unwrap();
        let structure = STRUCTURE.get().unwrap();

        let sentence: String = self.genes.iter().map(|gt| gt.0).collect();
        let suggestion_count = rules.suggest(&sentence, tokenizer).len() as f32;
        let structure_error_count: f32 = structure
            .iter()
            .zip(self.genes)
            .filter(|(wt, gt)| gt.1 != **wt)
            .count() as f32;

        let grammar_fitness: f32 = 0.5 * (1.0 / (suggestion_count + 1.0));
        let structure_fitness: f32 = 0.5 * (1.0 / (structure_error_count + 1.0));
        let fitness = grammar_fitness + structure_fitness;

        self.fitness = Some(fitness)
    }
}

#[derive(Deserialize)]
struct Config {
    pub generations: i32,
    pub population_count: i32,
}

fn main() {
    let config_data = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_data).unwrap();

    let mut tokenizer_bytes: &'static [u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("en")));
    let mut rules_bytes: &'static [u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/", rules_filename!("en")));

    let tokenizer =
        Tokenizer::from_reader(&mut tokenizer_bytes).expect("tokenizer binary is valid");
    let rules = Rules::from_reader(&mut rules_bytes).expect("rules binary is valid");
    let structure: &[WordType] = match WORD_COUNT {
        3 => {
            let rand_num = rand::random_range(0..=1);
            &WORD_COUNT_STRUCTURE_THREE[rand_num]
        }
        4 => &WORD_COUNT_STRUCTURE_FOUR,
        5 => &WORD_COUNT_STRUCTURE_FIVE,
        6 => &WORD_COUNT_STRUCTURE_SIX,
        7 => &WORD_COUNT_STRUCTURE_SEVEN,
        _ => &WORD_COUNT_STRUCTURE_EIGHT,
    };
    TOKENIZER.set(tokenizer).ok();
    RULES.set(rules).ok();
    STRUCTURE.set(structure).ok();

    let mut population: Vec<Chromosome> = generate_population(config.population_count);

    population
        .par_iter_mut()
        .for_each(|c| c.calculate_fitness());

    for _ in 0..config.generations {
        sort_population_descending(&mut population);
        let parent1 = &population[0];
        let parent2 = &population[1];

        let (mut child1, mut child2) = single_point_crossover(parent1, parent2, PC);
        child1.mutate_genes();
        child2.mutate_genes();

        let mut new_population: Vec<Chromosome> = generate_population(config.population_count - 4);

        new_population.push(child1);
        new_population.push(child2);
        new_population.push(*parent1);
        new_population.push(*parent2);

        new_population
            .par_iter_mut()
            .for_each(|c| c.calculate_fitness());
        population = new_population
    }

    sort_population_descending(&mut population);
    let best = &population[0];
    let best_constructed_word = best
        .genes
        .iter()
        .map(|g| g.0.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!(
        "Fitness: {}\nWord: {}.",
        best.fitness.unwrap(),
        best_constructed_word
    );
}
