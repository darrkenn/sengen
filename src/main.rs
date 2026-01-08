mod structures;
mod words;

//Crossover probability
const PC: f32 = 0.5;
//Mutation probability
const PM: f32 = 0.05;

// Sentence length
const WORD_COUNT: usize = 5;

// Word type rates
const NOUN_RATE: f32 = 0.20;
const PRONOUN_RATE: f32 = 0.10;
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

fn main() {}
