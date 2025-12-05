use std::{array, fs};

use genetica::{
    crossover::single_point_crossover,
    individual::{Generate, Individual, Mutate},
    population::{generate_population, sort_population_descending},
};
use serde::Deserialize;

const PC: f32 = 0.7;
const PM: f32 = 0.05;

const ARRAYSIZE: usize = 12;

const VALUES: [i32; ARRAYSIZE] = [75, 22, 91, 48, 15, 63, 30, 88, 55, 12, 99, 40];
const WEIGHTS: [i32; ARRAYSIZE] = [3, 1, 5, 2, 1, 4, 2, 5, 3, 1, 5, 2];
const MAX_WEIGHT: i32 = 12;

#[derive(Debug, Clone, Copy)]
struct GeneType(pub bool);

impl Generate for GeneType {
    fn generate() -> Self {
        GeneType(rand::random_bool(0.25))
    }
}

impl Mutate for GeneType {
    fn mutate(&mut self) {
        if rand::random_range(0.00..1.00) <= PM {
            self.0 = !self.0
        };
    }
}

#[derive(Debug, Clone, Copy)]
struct Chromosome {
    genes: [GeneType; ARRAYSIZE],
    fitness: Option<i32>,
}

impl Individual for Chromosome {
    type GeneType = GeneType;
    const GENES_SIZE: usize = ARRAYSIZE;
    fn new() -> Self {
        let genes: [GeneType; ARRAYSIZE] = array::from_fn(|_| GeneType::generate());
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

    fn fitness(&self) -> Option<i32> {
        self.fitness
    }
    fn fitness_mut(&mut self) -> &mut Option<i32> {
        &mut self.fitness
    }

    fn calculate_fitness(&mut self) {
        let fitness = {
            let total_weight = total(&self.genes, WEIGHTS);
            if total_weight <= MAX_WEIGHT {
                total(&self.genes, VALUES)
            } else {
                0
            }
        };
        self.fitness = Some(fitness);
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

    let mut population: Vec<Chromosome> = generate_population(config.population_count);

    population.iter_mut().for_each(|c| c.calculate_fitness());

    for _ in 0..config.generations {
        sort_population_descending(&mut population);
        let parent1 = &population[0];
        let parent2 = &population[1];

        let (mut child1, mut child2) = single_point_crossover(parent1, parent2, PC);
        child1.mutate_genes();
        child2.mutate_genes();

        let mut new_population: Vec<Chromosome> = generate_population(config.population_count - 3);

        new_population.push(child1);
        new_population.push(child2);
        new_population.push(*parent1);

        new_population
            .iter_mut()
            .for_each(|c| c.calculate_fitness());
        population = new_population
    }

    sort_population_descending(&mut population);
    let best = &population[0];

    println!(
        "Best result\nFitness: {}\nTotal weight: {}",
        best.fitness.unwrap(),
        total(&best.genes, WEIGHTS)
    );
}

fn total(genes: &[GeneType; ARRAYSIZE], array: [i32; ARRAYSIZE]) -> i32 {
    array
        .iter()
        .zip(genes.iter())
        .filter(|&(_, gene)| gene.0)
        .map(|(value, _)| value)
        .sum()
}
