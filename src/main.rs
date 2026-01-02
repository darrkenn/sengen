use std::{array, fs, sync::OnceLock};
mod structures;
mod words;
use genetica::{
    crossover::single_point_crossover,
    individual::{Generate, Individual, Mutate},
    population::{generate_population, sort_population_descending},
};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use serde::Deserialize;

use crate::{
    structures::{
        WORD_COUNT_STRUCTURE_EIGHT, WORD_COUNT_STRUCTURE_FIVE, WORD_COUNT_STRUCTURE_FOUR,
        WORD_COUNT_STRUCTURE_SEVEN, WORD_COUNT_STRUCTURE_SIX, WORD_COUNT_STRUCTURE_THREE,
    },
    words::{
        ADJECTIVES, ADJECTIVES_COUNT, ADVERBS, ADVERBS_COUNT, AdjectiveType, CONJUNCTIONS,
        CONJUNCTIONS_COUNT, DETERMINERS, DETERMINERS_COUNT, NOUNS, NOUNS_COUNT, PREPOSITIONS,
        PREPOSITIONS_COUNT, VERBS, VERBS_COUNT,
    },
};

static STRUCTURE: OnceLock<&[WordType]> = OnceLock::new();

//Crossover probability
const PC: f32 = 0.5;
//Mutation probability
const PM: f32 = 0.05;

// Sentence length
const WORD_COUNT: usize = 5;

// Word type rates
const NOUN_RATE: f32 = 0.30;
const VERB_RATE: f32 = 0.20;
const ADVERB_RATE: f32 = 0.10;
const ADJECTIVE_RATE: f32 = 0.10;
const PREPOSITION_RATE: f32 = 0.10;
const DETERMINER_RATE: f32 = 0.10;

// Noun type rates
const NOUN_PLURAL_RATE: f32 = 0.30;
// Verb type rates
const VERB_PAST_TENSE_RATE: f32 = 0.33;
const VERB_PRESENT_TENSE_RATE: f32 = 0.33;
// Adjective type rates (Minimum is 0.125)
const ADJ_INTERROGATIVE_RATE: f32 = 0.125;
const ADJ_DISTRIBUTIVE_RATE: f32 = 0.125;
const ADJ_NUMERAL_RATE: f32 = 0.125;
const ADJ_PROPER_RATE: f32 = 0.125;
const ADJ_DESCRIPTIVE_RATE: f32 = 0.125;
const ADJ_POSSESIVE_RATE: f32 = 0.125;
const ADJ_QUANTATIVE_RATE: f32 = 0.125;

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

fn select_word<'a>() -> GeneType<'a> {
    let random_f32: f32 = rand::random_range(0.00..1.00);
    let (word, word_type) = if random_f32 <= NOUN_RATE {
        let noun = &NOUNS[rand::random_range(0..*NOUNS_COUNT)];
        let word = if rand::random_range(0.00..1.00) <= NOUN_PLURAL_RATE && noun.plural.is_some() {
            noun.plural.as_ref().unwrap().as_str()
        } else {
            noun.singular.as_str()
        };

        (word, WordType::Noun)
    } else if random_f32 <= (NOUN_RATE + VERB_RATE) {
        let verb = &VERBS[rand::random_range(0..*VERBS_COUNT)];
        let tense_num = rand::random_range(0.00..1.00);
        let word = if tense_num <= VERB_PAST_TENSE_RATE {
            verb.past.as_str()
        } else if tense_num <= VERB_PAST_TENSE_RATE + VERB_PRESENT_TENSE_RATE {
            verb.present.as_str()
        } else {
            verb.future.as_str()
        };

        (word, WordType::Verb)
    } else if random_f32 <= (NOUN_RATE + VERB_RATE + ADVERB_RATE) {
        (
            ADVERBS[rand::random_range(0..*ADVERBS_COUNT)].as_str(),
            WordType::Adverb,
        )
    } else if random_f32 <= (NOUN_RATE + VERB_RATE + ADVERB_RATE + ADJECTIVE_RATE) {
        let type_num = rand::random_range(0.00..1.00);
        // Holy shit this is so stupid
        let adjective_type = match type_num {
            n if n <= ADJ_INTERROGATIVE_RATE => AdjectiveType::Interrogative,
            n if n <= ADJ_INTERROGATIVE_RATE + ADJ_DISTRIBUTIVE_RATE => AdjectiveType::Distributive,
            n if n <= ADJ_INTERROGATIVE_RATE + ADJ_DISTRIBUTIVE_RATE + ADJ_NUMERAL_RATE => {
                AdjectiveType::Numeral
            }
            n if n
                <= ADJ_INTERROGATIVE_RATE
                    + ADJ_DISTRIBUTIVE_RATE
                    + ADJ_NUMERAL_RATE
                    + ADJ_PROPER_RATE =>
            {
                AdjectiveType::Proper
            }
            n if n
                <= ADJ_INTERROGATIVE_RATE
                    + ADJ_DISTRIBUTIVE_RATE
                    + ADJ_NUMERAL_RATE
                    + ADJ_PROPER_RATE
                    + ADJ_DESCRIPTIVE_RATE =>
            {
                AdjectiveType::Descriptive
            }
            n if n
                <= ADJ_INTERROGATIVE_RATE
                    + ADJ_DISTRIBUTIVE_RATE
                    + ADJ_NUMERAL_RATE
                    + ADJ_PROPER_RATE
                    + ADJ_DESCRIPTIVE_RATE
                    + ADJ_QUANTATIVE_RATE =>
            {
                AdjectiveType::Quantative
            }
            n if n
                <= ADJ_INTERROGATIVE_RATE
                    + ADJ_DISTRIBUTIVE_RATE
                    + ADJ_NUMERAL_RATE
                    + ADJ_PROPER_RATE
                    + ADJ_DESCRIPTIVE_RATE
                    + ADJ_QUANTATIVE_RATE
                    + ADJ_POSSESIVE_RATE =>
            {
                AdjectiveType::Possessive
            }
            _ => AdjectiveType::Demonstrative,
        };
        let word = ADJECTIVES
            .iter()
            .find(|adj| adj.adjective_type == adjective_type)
            .unwrap_or(&ADJECTIVES[rand::random_range(0..*ADJECTIVES_COUNT)])
            .word
            .as_str();

        (word, WordType::Adjective)
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

impl<'a> Generate for GeneType<'a> {
    fn generate() -> Self {
        select_word()
    }
}

impl<'a> Mutate for GeneType<'a> {
    fn mutate(&mut self) {
        if rand::random_range(0.00..1.00) <= PM {
            let selected_word = select_word();
            self.0 = selected_word.0;
            self.1 = selected_word.1;
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
        let structure = STRUCTURE.get().unwrap();
        let structure_error_count: f32 = structure
            .iter()
            .zip(self.genes)
            .filter(|(wt, gt)| gt.1 != **wt)
            .count() as f32;

        let structure_fitness: f32 = 1.0 * (1.0 / (structure_error_count + 1.0));
        let fitness = structure_fitness;

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

    let word_rate_count =
        NOUN_RATE + VERB_RATE + ADVERB_RATE + ADJECTIVE_RATE + PREPOSITION_RATE + DETERMINER_RATE;
    if word_rate_count >= 100.00 {
        println!("Word rate count is greater than 100: {word_rate_count}");
    };

    let structure: &[WordType] = match WORD_COUNT {
        3 => {
            let rand_num = rand::random_range(0..=1);
            &WORD_COUNT_STRUCTURE_THREE[rand_num]
        }
        4 => &WORD_COUNT_STRUCTURE_FOUR,
        5 => {
            let rand_num = rand::random_range(0..=3);
            &WORD_COUNT_STRUCTURE_FIVE[rand_num]
        }
        6 => &WORD_COUNT_STRUCTURE_SIX,
        7 => &WORD_COUNT_STRUCTURE_SEVEN,
        _ => &WORD_COUNT_STRUCTURE_EIGHT,
    };
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
    let sentence = construct_sentence(best);
    println!("Fitness: {}\nWord: {}", best.fitness.unwrap(), sentence);
}

fn construct_sentence(chromosome: &Chromosome) -> String {
    let mut words = chromosome
        .genes
        .iter()
        .map(|g| g.0.to_string())
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
