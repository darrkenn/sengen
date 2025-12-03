use std::{array, fs};

use serde::Deserialize;

const GENES_SIZE: usize = 12;
const GENE_LOWER_RANGE: i32 = 0;
const GENE_UPPER_RANGE: i32 = 9;
type GeneType = bool;
const PC: f32 = 0.7;
const PM: f32 = 0.05;

const VALUES: [i32; GENES_SIZE] = [10, 50, 20, 70, 30, 100, 15, 45, 60, 25, 90, 35];
const WEIGHTS: [i32; GENES_SIZE] = [1, 3, 2, 5, 1, 4, 1, 2, 3, 2, 4, 2];
const MAX_WEIGHT: i32 = 10;

trait Generate: Sized {
    fn generate() -> Self;
}

trait Mutate: Sized {
    fn mutate(self) -> Self;
}

impl Generate for bool {
    fn generate() -> Self {
        rand::random_bool(0.25)
    }
}

impl Mutate for bool {
    fn mutate(self) -> Self {
        if rand::random_range(0.00..1.00) <= PM {
            !self
        } else {
            self
        }
    }
}

impl Generate for i32 {
    fn generate() -> Self {
        rand::random_range(GENE_LOWER_RANGE..=GENE_UPPER_RANGE)
    }
}

impl Mutate for i32 {
    fn mutate(self) -> Self {
        if rand::random_range(0.00..1.00) <= PM {
            Self::generate()
        } else {
            self
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Chromozone {
    pub genes: [GeneType; GENES_SIZE],
    pub fitness: Option<i32>,
}

impl Chromozone {
    fn fitness(&mut self) {
        let fitness = {
            let total_weight: i32 = WEIGHTS
                .iter()
                .zip(self.genes.iter())
                .filter(|&(_, gene)| *gene)
                .map(|(weight, _)| weight)
                .sum();
            if total_weight <= MAX_WEIGHT {
                let total_value = VALUES
                    .iter()
                    .zip(self.genes.iter())
                    .filter(|&(_, gene)| *gene)
                    .map(|(value, _)| value)
                    .sum();
                total_value
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

    let mut population = generate_population(config.population_count);

    for _ in 0..config.generations {
        population.sort_by(|a, b| b.fitness.unwrap().cmp(&a.fitness.unwrap()));
        let parent1 = population[0];
        let parent2 = population[1];

        let mut new_population: Vec<Chromozone> = vec![parent1];

        let (mut child1, mut child2) = crossover((parent1, parent2));

        //Mutate genes
        child1.genes = mutate_genes(child1.genes);
        child2.genes = mutate_genes(child2.genes);

        //Recalculate fitness
        child1.fitness();
        child2.fitness();

        new_population.push(child1);
        new_population.push(child2);

        let num_to_add = (config.population_count - 3) as usize;
        for i in 0..num_to_add {
            new_population.push(population[i + 3]);
        }

        population = new_population;
    }

    /*
        for chromozone in &population {
            println!("{:?}", chromozone)
        }
    */

    let best = best_in_run(&mut population);
    println!("Best: {:?}", best);
}

fn generate_genes<GeneType: Generate>() -> [GeneType; GENES_SIZE] {
    array::from_fn(|_| GeneType::generate())
}

fn mutate_genes<GeneType: Mutate + Copy>(genes: [GeneType; GENES_SIZE]) -> [GeneType; GENES_SIZE] {
    array::from_fn(|i| genes[i].mutate())
}

fn generate_population(population_size: i32) -> Vec<Chromozone> {
    let mut population: Vec<Chromozone> = Vec::new();

    for _ in 0..population_size {
        let mut chromozone = Chromozone {
            genes: generate_genes(),
            fitness: None,
        };
        chromozone.fitness();
        population.push(chromozone);
    }
    population
}

fn crossover((mut parent1, mut parent2): (Chromozone, Chromozone)) -> (Chromozone, Chromozone) {
    if rand::random::<f32>() <= PC {
        let split: usize = rand::random_range(0..=GENES_SIZE) as usize;
        let (_p1_start, p1_end) = parent1.genes.split_at(split);
        let (_p2_start, p2_end) = parent2.genes.split_at(split);
        let p1_end = p1_end.to_vec();
        let p2_end = p2_end.to_vec();
        parent1.genes[split..].copy_from_slice(&p2_end);
        parent2.genes[split..].copy_from_slice(&p1_end);
        (parent1, parent2)
    } else {
        (parent1, parent2)
    }
}

fn best_in_run(population: &mut Vec<Chromozone>) -> Chromozone {
    population.sort_by(|a, b| b.fitness.unwrap().cmp(&a.fitness.unwrap()));
    population[0]
}
