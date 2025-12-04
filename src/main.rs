use std::fs;

use genetica::{
    crossover::single_point_crossover,
    individual::{Chromosome, Generate, Mutate},
    population::{generate_population, sort_population_descending},
};
use serde::Deserialize;

const PC: f32 = 0.7;
const PM: f32 = 0.05;

const VALUES: [i32; 12] = [10, 50, 20, 70, 30, 100, 15, 45, 60, 25, 90, 35];
const WEIGHTS: [i32; 12] = [1, 3, 2, 5, 1, 4, 1, 2, 3, 2, 4, 2];
const MAX_WEIGHT: i32 = 10;

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

#[derive(Deserialize)]
struct Config {
    pub generations: i32,
    pub population_count: i32,
}

fn main() {
    let config_data = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_data).unwrap();

    let mut population: Vec<Chromosome<GeneType, 12>> =
        generate_population(config.population_count);

    population.iter_mut().for_each(|c| calculate_fitness(c));

    for _ in 0..config.generations {
        sort_population_descending(&mut population);
        let parent1 = population[0];
        let parent2 = population[1];

        let (mut child1, mut child2) = single_point_crossover(parent1, parent2, PC);
        child1.mutate_genes();
        child2.mutate_genes();

        let mut new_population: Vec<Chromosome<GeneType, 12>> =
            generate_population(config.population_count - 3);

        new_population.push(child1);
        new_population.push(child2);
        new_population.push(parent1);

        new_population.iter_mut().for_each(|c| calculate_fitness(c));
        population = new_population
    }

    sort_population_descending(&mut population);

    println!("Best: {:?}", population[0]);
}

fn calculate_fitness(chromosome: &mut Chromosome<GeneType, 12>) {
    let fitness = {
        let total_weight: i32 = WEIGHTS
            .iter()
            .zip(chromosome.genes.iter())
            .filter(|&(_, gene)| gene.0)
            .map(|(weight, _)| weight)
            .sum();
        if total_weight <= MAX_WEIGHT {
            let total_value = VALUES
                .iter()
                .zip(chromosome.genes.iter())
                .filter(|&(_, gene)| gene.0)
                .map(|(value, _)| value)
                .sum();
            total_value
        } else {
            0
        }
    };
    chromosome.fitness = Some(fitness);
}
