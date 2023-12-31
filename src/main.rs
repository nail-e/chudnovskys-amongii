// Among Us Pattern:
// 0111
// 1100
// 1111
// 0101
// 0111110011110101
// or
// 1110
// 0011
// 1111
// 1010
// 1110001111111010

mod lib;

use lib::pi;
use std::env;

// Edit this for precision
const PRECISION: u32 = 1000000;

// Default number of iterations
const ITERATIONS: u32 = 1;

fn main() {
    // Command line arguments
    let args: Vec<_> = env::args().collect();

    // Make decisions based on the number of arguments
    match args.len() {
        1 => {
            println!("Approximation: {:?}", pi(PRECISION, ITERATIONS));
            println!("Precision:     {:?}", PRECISION);
            println!("Iterations:    {:?}", ITERATIONS);
        }
        2 => {
            let precision = args[1]
                .parse::<u32>()
                .expect("Precision must be a positive integer!");
            if precision <= 0 {
                panic!("Precision must be a positive integer!");
            } else {
                println!("Approximation: {:?}", pi(precision as u32, ITERATIONS));
                println!("Precision:  {:?}", precision);
                println!("Iterations: {:?}", ITERATIONS);
            }
        }
        3 => {
            let precision = args[1]
                .parse::<u32>()
                .expect("Precision must be a positive integer!");
            let iterations = args[2]
                .parse::<u32>()
                .expect("Number of iterations must be a positive integer!");
            if precision <= 0 {
                panic!("Precision must be a positive integer!");
            } else if iterations <= 0 {
                panic!("Number of iterations must be a positive integer!");
            } else {
                println!(
                    "Approximation: {:?}",
                    pi(precision as u32, iterations as u32)
                );
                println!("Precision:  {:?}", precision);
                println!("Iterations: {:?}", iterations);
            }
        }

        _ => panic!("Redundant argument."),
    }
}
