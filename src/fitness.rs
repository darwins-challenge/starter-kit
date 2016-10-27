use moonlander_gp::AstNode;
use moonlander_gp::genetic::SimpleFitness;
use rand::Rng;

/// Calculate the fitness of a single program.
///
/// How to do this depends on the domain you are examining and is subject to
/// intense study.
pub fn score_program<P>(program: &P, _: &mut Rng) -> SimpleFitness where P: AstNode {
    SimpleFitness::new(vec![
        ("TODO: extend score card", 1.0)
    ])
}
