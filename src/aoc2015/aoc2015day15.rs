//! [aoc](https://adventofcode.com/2015/day/15)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 15,
        title: "Science for Hungry People",
        solution: ("21367368", "1766400"),
        example_solutions: vec![("62842880", "57600000")],
    }
}

type ItemType = i32;

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    const TOTAL_QUANTITY: ItemType = 101;
    let ingredients = parse_input(input)?;
    let mut ans1 = 0;
    let mut ans2 = 0;
    let powers = (0..=ingredients.len() as u32)
        .map(|x| TOTAL_QUANTITY.pow(x))
        .collect::<Vec<_>>();
    for code in 0..powers[ingredients.len() - 1] {
        let mut quantities = Vec::new();
        let mut remaining_quantity = TOTAL_QUANTITY - 1;
        for idx in 0..ingredients.len() - 1 {
            let qty = (code / powers[idx]) % TOTAL_QUANTITY;
            quantities.push(qty);
            remaining_quantity -= qty;
            if remaining_quantity < 0 {
                break;
            }
        }
        if remaining_quantity < 0 {
            continue;
        }
        quantities.push(remaining_quantity);
        let mut product = 1;
        for property in ["capacity", "durability", "flavor", "texture"] {
            let mut sum = 0;
            for (idx, ingredient) in ingredients.iter().enumerate() {
                sum += ingredient.get(property).unwrap_or(&0) * quantities[idx];
            }
            sum = cmp::max(0, sum);
            product *= sum;
        }
        let mut sum_calory = 0;
        for (idx, ingredient) in ingredients.iter().enumerate() {
            sum_calory += ingredient.get("calories").unwrap_or(&0) * quantities[idx];
        }
        ans1 = cmp::max(ans1, product);
        if sum_calory == 500 {
            ans2 = cmp::max(ans2, product);
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn parse_input(input: PuzzleInput) -> Result<Vec<HashMap<String, ItemType>>, PuzzleError> {
    let mut ingredients = Vec::with_capacity(input.len());
    for line in input {
        let a = line.split(' ').collect::<Vec<_>>();
        if a.len() != 11 {
            return Err(PuzzleError("invalid number of words in input line".into()));
        }
        let mut ingredient = HashMap::new();
        for i in 0..5 {
            let prop_name = a[1 + 2 * i];
            let prop_value = if i < 4 {
                a[2 + 2 * i][0..(a[2 + 2 * i].len() - 1)]
                    .parse::<ItemType>()
                    .map_err(|_| PuzzleError("property value must be an integer".into()))?
            } else {
                a[10]
                    .parse::<ItemType>()
                    .map_err(|_| PuzzleError("property value must be an integer".into()))?
            };
            ingredient.insert(prop_name.to_owned(), prop_value);
        }
        ingredients.push(ingredient);
    }
    Ok(ingredients)
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_input_line() {
        test_invalid(&[&"a"], solve);
    }

    #[test]
    fn invalid_property_value_must_be_integer() {
        test_invalid(
            &[&"Butterscotch: capacity -1, durability X, flavor 6, texture 3, calories 8"],
            solve,
        );
    }
}
