//! [aoc](https://adventofcode.com/2023/day20)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 20,
        title: "Pulse Propagation",
        solution: ("919383692", "247702167614647"),
        example_solutions: vec![("32000000", "0"), ("11687500", "0")],
    }
}

const MAX_STEPS_PART1: usize = 1000;
const BUTTON: &str = "button";
const BROADCASTER: &str = "broadcaster";
const RECEIVER: &str = "rx";

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut circuit = Circuit::new(input)?;
    // ---------- Part 1
    circuit.sim_steps(MAX_STEPS_PART1);
    let ans1 = circuit.count_low * circuit.count_high;
    // ---------- Part 2
    circuit.init();
    let ans2 = circuit.min_buttons_to_rx();
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum Pulse {
    Off,
    Low,
    High,
}

fn parse_module_line(line: &str) -> Result<(String, Vec<String>), PuzzleError> {
    let mut a_iter = line.split(" -> ");
    let name = a_iter.next().unwrap().to_owned();
    let outputs = a_iter
        .next()
        .ok_or("module name must be followed `->`")?
        .split(", ")
        .map(str::to_string)
        .collect::<Vec<String>>();
    if a_iter.next().is_some() {
        Err("line must contain only a single `->`")?;
    }
    Ok((name, outputs))
}

trait Module {
    fn name(&self) -> &str;

    fn outputs(&self) -> &[String];

    fn inputs(&self) -> Vec<String>;

    fn add_input(&mut self, input: &str);

    fn init(&mut self);

    fn hash(&self) -> String;

    fn receive(&mut self, input: &str, pulse: Pulse) -> Pulse;
}

#[derive(Default)]
struct FlipFlop {
    name: String,
    inputs: HashSet<String>,
    outputs: Vec<String>,
    state: bool,
}

impl FlipFlop {
    fn new(line: &str) -> Result<Self, PuzzleError> {
        let (name, outputs) = parse_module_line(line)?;
        Ok(Self {
            name,
            outputs,
            ..Default::default()
        })
    }
}

impl Module for FlipFlop {
    fn name(&self) -> &str {
        &self.name
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }

    fn inputs(&self) -> Vec<String> {
        self.inputs.iter().cloned().collect::<Vec<_>>()
    }

    fn add_input(&mut self, input: &str) {
        self.inputs.insert(input.to_owned());
    }

    fn init(&mut self) {
        self.state = false;
    }

    fn hash(&self) -> String {
        if self.state {
            String::from("1")
        } else {
            String::from("0")
        }
    }

    fn receive(&mut self, _input: &str, pulse: Pulse) -> Pulse {
        if pulse != Pulse::Low {
            return Pulse::Off;
        }
        self.state = !self.state;
        if self.state { Pulse::High } else { Pulse::Low }
    }
}

#[derive(Default)]
struct Conjunction {
    name: String,
    inputs: HashSet<String>,
    outputs: Vec<String>,
    states: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new(line: &str) -> Result<Self, PuzzleError> {
        let (name, outputs) = parse_module_line(line)?;
        Ok(Self {
            name,
            outputs,
            ..Default::default()
        })
    }
}

impl Module for Conjunction {
    fn name(&self) -> &str {
        &self.name
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }

    fn inputs(&self) -> Vec<String> {
        self.inputs.iter().cloned().collect::<Vec<_>>()
    }

    fn add_input(&mut self, input: &str) {
        self.inputs.insert(input.to_owned());
    }

    fn init(&mut self) {
        self.states.clear();
        for input in self.inputs.iter() {
            self.states.insert(input.to_owned(), Pulse::Low);
        }
    }

    fn hash(&self) -> String {
        // we need a stable, fixed order of values in the hashmap
        let mut pulses = self.states.iter().collect::<Vec<_>>();
        pulses.sort_by_key(|&x| x.0);
        let mut s = String::new();
        for (_, pulse) in pulses {
            s.push(match pulse {
                Pulse::High => '1',
                _ => '0',
            });
        }
        s
    }

    fn receive(&mut self, input: &str, pulse: Pulse) -> Pulse {
        self.states.insert(input.to_owned(), pulse);
        let count_high = self.states.values().filter(|&x| *x == Pulse::High).count();
        if count_high == self.inputs.len() {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

#[derive(Default)]
struct Broadcast {
    name: String,
    inputs: HashSet<String>,
    outputs: Vec<String>,
}

impl Broadcast {
    fn new(line: &str) -> Result<Self, PuzzleError> {
        let (name, outputs) = parse_module_line(line)?;
        Ok(Self {
            name,
            outputs,
            ..Default::default()
        })
    }
}

impl Module for Broadcast {
    fn name(&self) -> &str {
        &self.name
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }

    fn inputs(&self) -> Vec<String> {
        self.inputs.iter().cloned().collect::<Vec<_>>()
    }

    fn add_input(&mut self, input: &str) {
        self.inputs.insert(input.to_owned());
    }

    fn init(&mut self) {}

    fn hash(&self) -> String {
        String::from("0")
    }

    fn receive(&mut self, _input: &str, pulse: Pulse) -> Pulse {
        pulse
    }
}

#[derive(Default)]
struct Circuit {
    modules: HashMap<String, Box<dyn Module>>,
    count_low: usize,
    count_high: usize,
    final_module_part2: String,
}

impl Circuit {
    fn new(input: &[&str]) -> Result<Self, PuzzleError> {
        let mut circuit = Circuit::default();
        for &line in input {
            match line.chars().next().unwrap_or(' ') {
                '%' => {
                    let m = FlipFlop::new(&line[1..])?;
                    circuit.modules.insert(m.name().to_owned(), Box::new(m));
                }
                '&' => {
                    let m = Conjunction::new(&line[1..])?;
                    circuit.modules.insert(m.name().to_owned(), Box::new(m));
                }
                _ => {
                    if line.starts_with(BROADCASTER) {
                        let m = Broadcast::new(line)?;
                        circuit.modules.insert(m.name().to_owned(), Box::new(m));
                    } else {
                        return Err("invalid module type")?;
                    }
                }
            }
        }
        let names = circuit.modules.keys().cloned().collect::<Vec<_>>();
        for name in &names {
            let outputs = circuit.modules.get(name).unwrap().outputs().to_vec();
            for output in &outputs {
                if output == RECEIVER {
                    circuit.final_module_part2 = name.to_owned();
                }
                if circuit.modules.contains_key(output) {
                    circuit.modules.get_mut(output).unwrap().add_input(name);
                }
            }
        }
        Ok(circuit)
    }

    fn init(&mut self) {
        for (_, module) in self.modules.iter_mut() {
            module.init();
        }
        self.count_low = 0;
        self.count_high = 0;
    }

    fn hash(&self) -> String {
        let mut mods = self.modules.iter().collect::<Vec<_>>();
        mods.sort_by_key(|&x| x.0);
        let mut s = String::new();
        for (name, module) in mods {
            s.push_str(name);
            s.push(':');
            s.push_str(&module.hash());
            s.push('|');
        }
        s
    }

    #[expect(clippy::map_entry)]
    fn sim_steps(&mut self, max_steps: usize) {
        let mut item_id = 0;
        let mut visited = HashMap::new();
        let mut step = 1;
        while step <= max_steps {
            let turn = 0usize;
            let mut pq = PriorityQueue::new();
            let item = (
                BUTTON.to_owned(),
                BROADCASTER.to_owned(),
                Pulse::Low,
                item_id,
            );
            item_id += 1;
            pq.push(item, Reverse(turn));
            while !pq.is_empty() {
                let ((from, to, pulse, _), priority) = pq.pop().unwrap();
                let turn = priority.0;
                match pulse {
                    Pulse::Low => self.count_low += 1,
                    Pulse::High => self.count_high += 1,
                    _ => (),
                }
                if !self.modules.contains_key(&to) {
                    continue;
                }
                let next_pulse = self.modules.get_mut(&to).unwrap().receive(&from, pulse);
                if next_pulse == Pulse::Off {
                    continue;
                }
                for output in self.modules.get(&to).unwrap().outputs() {
                    pq.push(
                        (
                            to.to_owned(),
                            output.to_owned(),
                            next_pulse.clone(),
                            item_id,
                        ),
                        Reverse(turn + 1),
                    );
                    item_id += 1;
                }
            }
            let hash = self.hash();
            if !visited.contains_key(&hash) {
                visited.insert(hash, (step, self.count_low, self.count_high));
                step += 1;
                continue;
            }
            let (prev_step, prev_count_low, prev_count_high) = visited.get(&hash).unwrap();
            let cycle_len = step - prev_step;
            let cycle_count = (max_steps - step) / cycle_len;
            step += cycle_count * cycle_len;
            self.count_low += cycle_count * (self.count_low - *prev_count_low);
            self.count_high += cycle_count * (self.count_high - *prev_count_high);
            step += 1;
        }
    }

    fn min_buttons_to_rx(&mut self) -> usize {
        if self.final_module_part2.is_empty() {
            return 0;
        }
        let mut counters = HashMap::new();
        for name in self.modules.get(&self.final_module_part2).unwrap().inputs() {
            counters.insert(name.to_owned(), 0usize);
        }
        let mut step = 0;
        loop {
            step += 1;
            let turn = 0usize;
            let mut pq = PriorityQueue::new();
            let item = (BUTTON.to_owned(), BROADCASTER.to_owned(), Pulse::Low);
            pq.push(item, Reverse(turn));
            while !pq.is_empty() {
                let ((from, to, pulse), priority) = pq.pop().unwrap();
                let turn = priority.0;
                if to == RECEIVER && pulse == Pulse::Low {
                    return step;
                }
                if !self.modules.contains_key(&to) {
                    continue;
                }
                let next_pulse = self.modules.get_mut(&to).unwrap().receive(&from, pulse);
                if next_pulse == Pulse::Off {
                    continue;
                }
                if next_pulse == Pulse::High {
                    for (name, value) in counters.iter_mut() {
                        if *value == 0 && to == *name {
                            *value = step;
                        }
                    }
                }
                for output in self.modules.get(&to).unwrap().outputs() {
                    pq.push(
                        (to.to_owned(), output.to_owned(), next_pulse.clone()),
                        Reverse(turn + 1),
                    );
                }
            }
            if !counters.iter().any(|(_, x)| *x == 0) {
                break;
            }
        }
        let mut ans = 1;
        for cycle_len in counters.values() {
            ans = lcm(ans, *cycle_len);
        }
        ans
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a1 = max(a, b);
    let mut b1 = min(a, b);
    while b1 != 0 {
        (a1, b1) = (b1, a1 % b1);
    }
    a1
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_missing_arrow() {
        test_invalid_msg(
            &[&"broadcaster"],
            solve,
            "module name must be followed `->`",
        );
    }

    #[test]
    fn invalid_must_have_one_arrow() {
        test_invalid_msg(
            &[&"broadcaster -> a, b, c -> d"],
            solve,
            "line must contain only a single `->`",
        );
    }

    #[test]
    fn invalid_module_type() {
        test_invalid_msg(&[&"mod -> a, b, c"], solve, "invalid module type");
    }
}
