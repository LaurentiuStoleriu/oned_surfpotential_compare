use multivariate_optimization::optimize::*;
use multivariate_optimization::testfuncs::rastrigin;
use rand::{thread_rng, Rng};

fn my_f(args: &[f64]) -> f64 {
    //{ 1.0, 2.0, 10.0, 20.0, 30.0 }
    //p[2] * (x - p[0]) * (x - p[0]) + p[3] * (y - p[1]) * (y - p[1]) + p[4]

    10.0 * (args[0] - 1.0) * (args[0] - 1.0) + 20.0 * (args[1] - 2.0) * (args[1] - 2.0) + 30.0
}

fn main() {

    let my_x = [5.0, 7.0];

    const DIM: usize = 2;
    let search_space: Vec<SearchRange> = (0..DIM)
        .map(|_| SearchRange::Finite {
            low: -10.0,
            high: 10.0,
        })
        .collect();
    
    dbg!(&search_space);

    const POPULATION: usize = 1000;
    const MAX_ITERATIONS: usize = 1000;
    let mut solver = Solver::new(search_space, |params| {
        let cost = my_f(&params);
        BasicSpecimen { params, cost }
    });
    solver.set_speed_factor(0.5);

    let initial_specimens = solver.random_specimens(POPULATION);
    solver.extend_specimens(initial_specimens);
    for _iter in 0..MAX_ITERATIONS {
        let specimens = solver.specimens();
        //println!("{} {}", specimens[0].cost, specimens[specimens.len()-1].cost);
        if solver.converged() {
            break;
        }
        let new_specimens = solver.recombined_specimens(POPULATION, 0.0);
        solver.replace_worst_specimens(new_specimens);
    }
    let specimen = solver.into_specimen();
    dbg!(&specimen);

    //assert_eq!(specimen.cost, 30.0); 
}
