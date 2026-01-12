use genetica::individual::{Generate, Individual, Mutate};
use rand::random_range;

use crate::{
    CONFIG, WORD_THRESHOLDS, WordType,
    words::{
        ADJECTIVES, ADVERBS, CONJUNCTIONS, Collection, DETERMINERS, NOUNS, PREPOSITIONS, VERBS,
        Word,
    },
};

#[derive(Debug)]
pub struct GeneType<'a> {
    pub word: Box<dyn Word + 'a>,
}

fn select_word() -> Box<dyn Word> {
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

impl<'a> Generate for GeneType<'a> {
    fn generate() -> Self {
        GeneType {
            word: select_word(),
        }
    }
}

impl<'a> Mutate for GeneType<'a> {
    fn mutate(&mut self) {
        if rand::random_range(0.00..1.00) <= CONFIG.mutation_probability {
            self.word = select_word()
        }
    }
}

#[derive(Debug)]
pub struct Chromosome<'a> {
    pub genes: Vec<GeneType<'a>>,
    pub fitness: Option<f32>,
}
