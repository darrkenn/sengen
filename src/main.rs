use std::{array, fs};

use rand::{random_range, rngs::ThreadRng, seq::IndexedRandom};
use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
struct Chromozone {
    pub genes: [i32; 8],
    pub fitness: Option<i32>,
}

#[derive(Deserialize)]
struct Config {
    pub generations: i32,
    pub population_count: i32,
}

const PC: f32 = 0.7;
const PM: f32 = 0.05;

fn main() {
    let config_data = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_data).unwrap();

    let mut rng = rand::rng();
    let mut population = generate_population(config.population_count, &mut rng);

    for _ in 0..config.generations {
        let (parent1, parent2) = top_two_fitness(&mut population);

        let mut new_population = generate_population(config.population_count - 2, &mut rng);

        let (parent1, parent2) = crossover((parent1, parent2));
        let mut parent1 = mutate(parent1);
        let mut parent2 = mutate(parent2);
        //Recalculate fitness
        parent1.fitness = Some(fitness(parent1.genes));
        parent2.fitness = Some(fitness(parent2.genes));

        new_population.push(parent1);
        new_population.push(parent2);
        population = new_population;
    }

    let best = best_in_run(&mut population);
    for chromozone in population {
        println!("{:?}", chromozone)
    }
    println!("Best: {:?}", best);
}

fn fitness(values: [i32; 8]) -> i32 {
    (values[0] + values[1]) - (values[2] + values[3]) + (values[4] + values[5])
        - (values[6] + values[7])
}

fn generate_population(population_size: i32, mut rng: &mut ThreadRng) -> Vec<Chromozone> {
    let mut population: Vec<Chromozone> = Vec::new();
    let range: Vec<i32> = (0..10).collect();

    for _ in 0..population_size {
        let mut chromozone = Chromozone {
            genes: array::from_fn(|_| *range.choose(&mut rng).unwrap()),
            fitness: None,
        };
        let fitness = fitness(chromozone.genes);
        chromozone.fitness = Some(fitness);
        population.push(chromozone);
    }
    population
}

fn crossover((mut parent1, mut parent2): (Chromozone, Chromozone)) -> (Chromozone, Chromozone) {
    if rand::random::<f32>() <= PC {
        let split: usize = rand::random_range(0..8);
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

fn mutate(mut parent: Chromozone) -> Chromozone {
    let mutated_genes: [i32; 8] = array::from_fn(|i| {
        let g = parent.genes[i];
        if rand::random_range(0.00..1.00) <= PM {
            random_range(0..=9)
        } else {
            g
        }
    });
    parent.genes = mutated_genes;
    parent
}

fn top_two_fitness(population: &mut Vec<Chromozone>) -> (Chromozone, Chromozone) {
    population.sort_by(|a, b| b.fitness.unwrap().cmp(&a.fitness.unwrap()));
    (population[0], population[1])
}

fn best_in_run(population: &mut Vec<Chromozone>) -> Chromozone {
    population.sort_by(|a, b| b.fitness.unwrap().cmp(&a.fitness.unwrap()));
    population[0]
}
