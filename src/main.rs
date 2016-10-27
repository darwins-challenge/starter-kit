#[macro_use]
extern crate moonlander_gp;
extern crate rand;

mod fitness;
mod grammar;

use moonlander_gp::{Population, random_population};
use moonlander_gp::genetic::{SimpleFitness, evolve, Weights, tournament_selection};
use grammar::start::Start;

const POPULATION_SIZE : usize = 100;
const NR_GENERATIONS : usize = 50;
const TOURNAMENT_SIZE : usize = 10;
const MAX_DEPTH : usize = 8;

type StarterKitPopulation = Population<Start, SimpleFitness>;

fn main() {
    let mut rng = rand::StdRng::new().unwrap();

    let weights = Weights {
        reproduce: 10,
        mutate: 20,
        crossover: 70,
        tree_height: MAX_DEPTH as i32,
    };

    let mut pop: StarterKitPopulation = random_population(POPULATION_SIZE, MAX_DEPTH, &mut rng);
    for generation in 0..NR_GENERATIONS {
        pop.score(fitness::score_program, &mut rng);
        println!("Generation {}, best {}, average {}", generation, pop.best_score(), pop.avg_score());

        pop = evolve(pop, &weights, &mut rng, |p, r| tournament_selection(TOURNAMENT_SIZE, p, r));
    }
}

