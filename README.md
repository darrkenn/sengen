# sengen
segen is a random sentence generator that uses a genetic algorithm to determine the most logical sentence.


# Genetic algorithm variables
## Generations and population count
Generations (How many runs) and population count (Amount of candidates) are defined in the config.toml file. 
```toml
generations = 1000
population_count = 250
```
Feel free to play around with these to see what gets you the best result
## Crossover and mutation probabilities
The crossover and mutation probabilities are defined at the top of the main.rs file.
```rust
const PC: f32 = 0.5; // Crossover probability, I recommend keeping it around 0.5
const PM: f32 = 0.05; // Mutation probability, I recommend keeping it around 0.05
```


# Structures
Structures are defined in the structures.rs file
```rust
pub const WORD_COUNT_STRUCTURE_FOUR: [WordType; 4] = [
    WordType::Adjective,
    WordType::Noun,
    WordType::Verb,
    WordType::Noun
]
```

# Words
Words are defined in the respective word type .toml files in the /words folder. To add new words, simply add them to the required file and recompile.
## Word type rates
Word type rate probabilities are defined at the top of the main.rs file. Make sure to keep the total percentage below 1.00.
## Word count
Word count is defined at the top of the main.rs file. I would recommend keeping it lower as longer sentences don't really make alot of sense.
## Adjectives
```toml
{ 
word = "my", 
adjective_type = "Possessive" # Interrogative, Distributive, Numeral, Proper, Descriptive, Quantative, Demonstrative
}
```
## Nouns
```toml
{
singular = "blueberry", 
plural = "blueberries" # Not required
proper = false
}
```
## Verbs
```toml
{
past = "jumped",
present = "jumped",
future = "jump"
}
```

