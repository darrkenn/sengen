# sengen
segen is a random sentence generator that uses a genetic algorithm to determine the most logical sentence.


# Config
A default configuration file(config.toml) is provided, feel free to change the values in order to get the best result.

## Generation and population count
Generations are how many times the algorithm will run, and the population count is the count of chromosomes.
The higher either of these values are, the longer the program takes however it may result in a better final chromosome.
```toml
generations = 1000
population_count = 250
```
## Probabilities
Crossover probability is the chance that two of the best chromosomes in the pool will crossover or merge.
Mutation probability is the chance that a gene in a chromosome will change value.
```toml
crossover_probability = 0.5
mutation_probability = 0.05
```
## Word count
The word count option defines the amount of words the final sentence will be, however if you want structure of the sentence to affect the fitness, then it can only be a max of 8 so far. However, not opting for structure fitness will allow forinfinite words.
```toml
word_count = 5
```
## Fitness
The fitness of a chromosome in this context would be the quality of the sentence. Structure fitness will rate the sentence on how well it fits a common structure, grammar fitness is the quality of the grammer in the sentence (NOT YET IMPLEMENTED). Both of these can be on at the same time which will provide the best quality, but having only one or none works too.
```toml
use_structure_fitness = true
use_grammar_fitness = true
```

## Rates
The rates in the config file are how likely something is to be picked alognside its peers.
### Examples
```toml
[word_type_rates]
noun = 0.30
verb = 0.20
adverb = 0.10
adjective = 0.10
preposition = 0.10
determiner = 0.10
conjunction = 0.10
```
```toml
[noun_rates]
[noun_rates.type_rates]
common = 0.45
proper = 0.28
collective = 0.27
[noun_rates.tangibility_rates]
concrete = 0.55
abstract = 0.45
[noun_rates.countability_rates]
countable = 0.50
uncountable = 0.50
```

## Words
To contribute or modify words, view the respective toml files in the words folder and follow the structure to add a new word(s).
