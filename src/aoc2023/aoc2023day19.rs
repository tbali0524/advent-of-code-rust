//! [aoc](https://adventofcode.com/2023/day/19)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use regex::Regex;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 19,
        title: "Aplenty",
        solution: ("432434", "132557544578569"),
        example_solutions: vec![("19114", "167409079868000")],
    }
}

type ItemType = i64;

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";
const MIN_PROPERTY: ItemType = 1;
const MAX_PROPERTY: ItemType = 4000;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut i = 0;
    while i < input.len() && !input[i].is_empty() {
        let w = Workflow::from_string(input[i])?;
        workflows.insert(w.name.clone(), w);
        i += 1;
    }
    i += 1;
    while i < input.len() {
        parts.push(Part::from_string(input[i])?);
        i += 1;
    }
    if workflows.is_empty() || parts.is_empty() {
        Err("invalid input")?;
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for part in &parts {
        let mut wf_name = String::from("in");
        loop {
            let wf = workflows.get(&wf_name).ok_or("nonexistent workflow name")?;
            wf_name = wf.next_workflow(part);
            if wf_name == ACCEPTED {
                ans1 += part.rating();
                break;
            }
            if wf_name == REJECTED {
                break;
            }
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    let mut q = Vec::new();
    q.push((String::from("in"), PartRange::new()));
    let mut idx_read = 0;
    while idx_read < q.len() {
        let (wf_name, range) = &q[idx_read];
        idx_read += 1;
        let wf = workflows.get(wf_name).unwrap();
        let next_list = wf.next_workflow_ranges(range);
        for (next_wf_name, next_range) in next_list.iter() {
            if next_wf_name == ACCEPTED {
                ans2 += next_range.count_parts();
            } else if next_wf_name != REJECTED {
                q.push((next_wf_name.clone(), next_range.clone()));
            }
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Default)]
struct Workflow {
    name: String,
    properties: Vec<usize>,
    conditions: Vec<char>,
    values: Vec<ItemType>,
    next_workflows: Vec<String>,
    default_workflow: String,
}

impl Workflow {
    #[allow(clippy::field_reassign_with_default)]
    fn from_string(line: &str) -> Result<Self, PuzzleError> {
        let mut a_iter = line.split('{');
        let mut w = Self::default();
        w.name = a_iter.next().unwrap().to_owned();
        let wf_def = a_iter
            .next()
            .ok_or("workflow definition lines must contain `{`")?;
        if !wf_def.ends_with('}') {
            Err("workflow definition lines must end with `}`")?
        }
        let rules = &wf_def[..wf_def.len() - 1].split(',').collect::<Vec<_>>();
        for (idx, &rule) in rules.iter().enumerate() {
            if idx == rules.len() - 1 {
                w.default_workflow = rule.to_owned();
                break;
            }
            let mut b_iter = rule.split(':');
            let c = b_iter.next().unwrap();
            if c.len() < 3 {
                Err("invalid input")?
            }
            let property = "xmas"
                .find(c.chars().next().unwrap())
                .ok_or("property must be one of `xmas`")?;
            let condition = c.chars().nth(1).ok_or("invalid input")?;
            let value = c[2..]
                .parse::<ItemType>()
                .map_err(|_| "condition value must be integer")?;
            let next_workflow = b_iter
                .next()
                .ok_or("conditions except the last one must contain a `:`")?;
            if condition != '<' && condition != '>' {
                Err("condition must contain one of `<>`")?
            }
            w.properties.push(property);
            w.conditions.push(condition);
            w.values.push(value);
            w.next_workflows.push(next_workflow.to_owned());
        }
        Ok(w)
    }

    fn next_workflow(&self, p: &Part) -> String {
        for (i, &property) in self.properties.iter().enumerate() {
            if (self.conditions[i] == '<' && p.properties[property] < self.values[i])
                || (self.conditions[i] == '>' && p.properties[property] > self.values[i])
            {
                return self.next_workflows[i].to_owned();
            }
        }
        self.default_workflow.to_owned()
    }

    fn next_workflow_ranges(&self, range: &PartRange) -> Vec<(String, PartRange)> {
        let mut ans = Vec::new();
        let mut r = range.clone();
        for (i, &property) in self.properties.iter().enumerate() {
            let min = r.min[property];
            let max = r.max[property];
            let value = self.values[i];
            let next = &self.next_workflows[i];
            match self.conditions[i] {
                '<' => {
                    if max < value {
                        ans.push((next.clone(), r.clone()));
                        return ans;
                    }
                    if value <= min {
                        continue;
                    }
                    let mut low = r.clone();
                    low.max[property] = value - 1;
                    let mut high = r.clone();
                    high.min[property] = value;
                    ans.push((next.clone(), low));
                    r = high;
                }
                '>' => {
                    if value < min {
                        ans.push((next.clone(), r.clone()));
                        return ans;
                    }
                    if max <= value {
                        continue;
                    }
                    let mut low = r.clone();
                    low.max[property] = value;
                    let mut high = r.clone();
                    high.min[property] = value + 1;
                    ans.push((next.clone(), high));
                    r = low;
                }
                _ => (),
            }
        }
        ans.push((self.default_workflow.clone(), r.clone()));
        ans
    }
}

struct Part {
    properties: [ItemType; 4],
}

impl Part {
    #[allow(clippy::needless_range_loop)]
    fn from_string(line: &str) -> Result<Self, PuzzleError> {
        let r = Regex::new(r"\{x=(-?\d+),m=(-?\d+),a=(-?\d+),s=(-?\d+)\}").unwrap();
        let caps = r.captures(line).ok_or("invalid part")?;
        let mut properties = [0; 4];
        for i in 0..4 {
            properties[i] = caps
                .get(i + 1)
                .ok_or("missing property")?
                .as_str()
                .trim()
                .parse::<ItemType>()
                .map_err(|_| "property value must be integer")?;
        }
        Ok(Part { properties })
    }

    fn rating(&self) -> ItemType {
        self.properties.iter().sum()
    }
}

#[derive(Clone)]
struct PartRange {
    min: [ItemType; 4],
    max: [ItemType; 4],
}

impl PartRange {
    fn new() -> Self {
        Self {
            min: [MIN_PROPERTY; 4],
            max: [MAX_PROPERTY; 4],
        }
    }

    fn count_parts(&self) -> ItemType {
        let mut ans = 1;
        for i in 0..4 {
            ans *= self.max[i] - self.min[i] + 1;
        }
        ans
    }
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
    fn invalid_missing_workflow() {
        test_invalid_msg(&[&"", &"{x=1,m=2,a=3,s=4}"], solve, "invalid input");
    }

    #[test]
    fn invalid_missing_part() {
        test_invalid_msg(&[&"p{a>1:R,A}"], solve, "invalid input");
    }

    #[test]
    fn invalid_wf_missing_open_par() {
        test_invalid_msg(
            &[&"p a>1:R,A}", &"", &"{x=1,m=2,a=3,s=4}"],
            solve,
            "workflow definition lines must contain `{`",
        );
    }

    #[test]
    fn invalid_wf_missing_close_par() {
        test_invalid_msg(
            &[&"p{a>1:R,A", &"", &"{x=1,m=2,a=3,s=4}"],
            solve,
            "workflow definition lines must end with `}`",
        );
    }

    #[test]
    fn invalid_wf_missing_colon() {
        test_invalid_msg(
            &[&"p{a>1,A}", &"", &"{x=1,m=2,a=3,s=4}"],
            solve,
            "conditions except the last one must contain a `:`",
        );
    }

    #[test]
    fn invalid_wf_condition_too_short() {
        test_invalid_msg(
            &[&"p{a>:R,A}", &"", &"{x=1,m=2,a=3,s=4}"],
            solve,
            "invalid input",
        );
    }

    #[test]
    fn invalid_wf_property_must_be_xmas() {
        test_invalid_msg(
            &[&"p{Z>1:R,A}", &"", &"{x=1,m=2,a=3,s=4}"],
            solve,
            "property must be one of `xmas`",
        );
    }

    #[test]
    fn invalid_wf_condition_must_be_comparison() {
        test_invalid_msg(
            &[&"p{a=1:R,A}", &"", &"{x=1,m=2,a=3,s=4}"],
            solve,
            "condition must contain one of `<>`",
        );
    }

    #[test]
    fn invalid_wf_condition_must_be_value() {
        test_invalid_msg(
            &[&"p{a<A:R,A}", &"", &"{x=1,m=2,a=3,s=4}"],
            solve,
            "condition value must be integer",
        );
    }

    #[test]
    fn invalid_part() {
        test_invalid_msg(
            &[&"p{a>1:R,A}", &"", &"{x=1,m=2,a=3,Z=4}"],
            solve,
            "invalid part",
        );
    }

    #[test]
    fn invalid_property_must_be_integer() {
        test_invalid_msg(
            &[&"p{a>1:R,A}", &"", &"{x=A,m=2,a=3,s=4}"],
            solve,
            "invalid part",
        );
    }

    #[test]
    fn invalid_nonexistent_workflow_name() {
        test_invalid_msg(
            &[&"in{a>1:w1,A}", &"", &"{x=1,m=2,a=3,s=4}"],
            solve,
            "nonexistent workflow name",
        );
    }
}
