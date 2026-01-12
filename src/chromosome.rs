use std::sync::Arc;

use genetica::individual::{DynamicLengthIndividual, Generate, Individual, Mutate};
use rand::random_range;

use crate::{
    CONFIG, WORD_THRESHOLDS, WordType,
    words::{
        ADJECTIVES, ADVERBS, CONJUNCTIONS, Collection, DETERMINERS, NOUNS, PREPOSITIONS, VERBS,
        Word,
    },
};

#[derive(Debug, Clone)]
pub struct GeneType {
    pub word: Arc<dyn Word>,
}

fn select_word() -> Arc<dyn Word> {
    let thresholds = WORD_THRESHOLDS.as_ref();
    loop {
        let random_f32 = random_range(0.00..1.00);
        for &(threshold, word_type) in thresholds {
            if random_f32 <= threshold {
                let word = match word_type {
                    WordType::Noun => NOUNS.select(),
                    WordType::Verb => VERBS.select(),
                    WordType::Adverb => ADVERBS.select(),
                    WordType::Adjective => ADJECTIVES.select(),
                    WordType::Preposition => PREPOSITIONS.select(),
                    WordType::Determiner => DETERMINERS.select(),
                    WordType::Conjunction => CONJUNCTIONS.select(),
                };
                return word;
            }
        }
    }
}

impl Generate for GeneType {
    fn generate() -> Self {
        GeneType {
            word: select_word(),
        }
    }
}

impl<'a> Mutate for GeneType {
    fn mutate(&mut self) {
        if rand::random_range(0.00..1.00) <= CONFIG.mutation_probability {
            self.word = select_word()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chromosome {
    pub genes: Vec<GeneType>,
    pub fitness: Option<f32>,
}

impl Individual for Chromosome {
    type GeneType = GeneType;
    fn new() -> Self {
        let genes: Vec<GeneType> = (0..CONFIG.word_count)
            .map(|_| GeneType::generate())
            .collect();
        Chromosome {
            genes,
            fitness: None,
        }
    }
    fn mutate_genes(&mut self) {
        self.genes_mut().iter_mut().for_each(|g| g.mutate());
    }
    fn fitness(&self) -> Option<f32> {
        self.fitness
    }
    fn fitness_mut(&mut self) -> &mut Option<f32> {
        &mut self.fitness
    }
    fn calculate_fitness(&mut self) {
        self.fitness = Some(0.0)
    }
}

impl DynamicLengthIndividual for Chromosome {
    fn genes(&self) -> &Vec<Self::GeneType> {
        &self.genes
    }
    fn genes_mut(&mut self) -> &mut Vec<Self::GeneType> {
        &mut self.genes
    }
}
