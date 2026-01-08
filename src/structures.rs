use crate::WordType;

pub const WORD_COUNT_STRUCTURE_THREE: [[WordType; 3]; 2] = [
    [WordType::Noun, WordType::Verb, WordType::Noun],
    [WordType::Determiner, WordType::Noun, WordType::Verb],
];
pub const WORD_COUNT_STRUCTURE_FOUR: [WordType; 4] = [
    WordType::Adjective,
    WordType::Noun,
    WordType::Verb,
    WordType::Noun,
];

pub const WORD_COUNT_STRUCTURE_FIVE: [[WordType; 5]; 4] = [
    [
        WordType::Noun,
        WordType::Determiner,
        WordType::Verb,
        WordType::Adjective,
        WordType::Noun,
    ],
    [
        WordType::Adverb,
        WordType::Verb,
        WordType::Noun,
        WordType::Preposition,
        WordType::Noun,
    ],
    [
        WordType::Noun, //Change to pronoun when implemented
        WordType::Verb,
        WordType::Determiner,
        WordType::Noun,
        WordType::Adverb,
    ],
    [
        WordType::Adjective,
        WordType::Noun,
        WordType::Adverb,
        WordType::Verb,
        WordType::Noun,
    ],
];
pub const WORD_COUNT_STRUCTURE_SIX: [WordType; 6] = [
    WordType::Adjective,
    WordType::Noun,
    WordType::Adverb,
    WordType::Verb,
    WordType::Adjective,
    WordType::Noun,
];
pub const WORD_COUNT_STRUCTURE_SEVEN: [WordType; 7] = [
    WordType::Determiner,
    WordType::Noun,
    WordType::Adverb,
    WordType::Verb,
    WordType::Determiner,
    WordType::Noun,
    WordType::Preposition,
];
pub const WORD_COUNT_STRUCTURE_EIGHT: [WordType; 8] = [
    WordType::Noun,
    WordType::Verb,
    WordType::Conjunction,
    WordType::Determiner,
    WordType::Adjective,
    WordType::Noun,
    WordType::Verb,
    WordType::Adverb,
];
