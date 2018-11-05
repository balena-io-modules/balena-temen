use balena_temen::{error::Result, parser::ast::*};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};

// TODO Come with better fuzzer strategy
//
//  - run on CI?
//  - disable locally?
//  - more iterations, longer strings, ...?
//  - add as feature?

#[test]
fn test_random() {
    let mut rng = thread_rng();

    for _ in 0..10 {
        let length = rng.gen::<u16>() as usize;
        let expression: String = rng
            .sample_iter::<char, Standard>(&Standard)
            .take(length)
            .into_iter()
            .collect();
        let _ = expression.parse() as Result<Expression>;
    }
}
