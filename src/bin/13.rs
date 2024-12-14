advent_of_code::solution!(13);

use regex::Regex;
use z3::{
    ast::{Ast, Int},
    Config, Context, Optimize, SatResult,
};

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, 0))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, 10_000_000_000_000))
}

fn solve(input: &str, pos_inc: i64) -> i64 {
    let re = Regex::new(r"[+-]?\d+").unwrap();
    let numbers: Vec<i64> = re
        .find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    numbers
        .chunks_exact(6)
        .map(|chunk| {
            let [a, b, c, d, e, f] = <[i64; 6]>::try_from(chunk).unwrap();
            ((a, b), (c, d), (e + pos_inc, f + pos_inc))
        })
        .filter_map(|(a, b, target)| get_min_tokens(a, b, target))
        .sum()
}

fn get_min_tokens(a: (i64, i64), b: (i64, i64), target: (i64, i64)) -> Option<i64> {
    let ctx = Context::new(&Config::new());
    let opt = Optimize::new(&ctx);

    let a_presses = Int::new_const(&ctx, "a");
    let b_presses = Int::new_const(&ctx, "b");

    opt.assert(&a_presses.ge(&Int::from_i64(&ctx, 0)));
    opt.assert(&b_presses.ge(&Int::from_i64(&ctx, 0)));

    let a_x_inc = Int::from_i64(&ctx, a.0);
    let a_y_inc = Int::from_i64(&ctx, a.1);
    let b_x_inc = Int::from_i64(&ctx, b.0);
    let b_y_inc = Int::from_i64(&ctx, b.1);

    let x_target = Int::from_i64(&ctx, target.0);
    let y_target = Int::from_i64(&ctx, target.1);

    let x_sum = Int::add(
        &ctx,
        &[
            &Int::mul(&ctx, &[&a_presses, &a_x_inc]),
            &Int::mul(&ctx, &[&b_presses, &b_x_inc]),
        ],
    );
    opt.assert(&x_sum._eq(&x_target));

    let y_sum = Int::add(
        &ctx,
        &[
            &Int::mul(&ctx, &[&a_presses, &a_y_inc]),
            &Int::mul(&ctx, &[&b_presses, &b_y_inc]),
        ],
    );
    opt.assert(&y_sum._eq(&y_target));

    let total_tokens = Int::add(
        &ctx,
        &[
            &Int::mul(&ctx, &[&a_presses, &Int::from_i64(&ctx, 3)]),
            &Int::mul(&ctx, &[&b_presses, &Int::from_i64(&ctx, 1)]),
        ],
    );

    opt.minimize(&total_tokens);

    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            Some(model.eval(&total_tokens, true).unwrap().as_i64().unwrap())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
