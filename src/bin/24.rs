use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut registers, gates) = parse(input);

    let mut queue = VecDeque::from_iter(gates);

    while let Some(gate) = queue.pop_front() {
        if let (Some(&in1), Some(&in2)) = (registers.get(gate.in1), registers.get(gate.in2)) {
            let val = gate.get_out(in1, in2);
            registers.insert(gate.out, val);
        } else {
            queue.push_back(gate);
        }
    }

    let mut z_registers: Vec<_> = registers
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect();

    z_registers.sort_unstable_by(|a, b| a.0.cmp(b.0));
    z_registers.reverse();

    let z_registers: Vec<_> = z_registers.into_iter().map(|(_, v)| v).collect();

    let mut num = 0u64;
    for &bit in &z_registers {
        num = (num << 1) | (bit as u64);
    }

    Some(num)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, gates) = parse(input);
    let gates: Vec<_> = gates.collect();

    let mut swapped: HashSet<&str> = HashSet::new();

    for i in 1..=44 {
        let x = format!("x{:02}", i);

        let xor_gate = gates
            .iter()
            .find(|gate| gate.has_input(x.as_str()) && gate.operation == Operation::XOR)
            .unwrap();

        let and_gate = gates
            .iter()
            .find(|gate| gate.has_input(x.as_str()) && gate.operation == Operation::AND)
            .unwrap();

        let and_gate_out_gates = gates
            .iter()
            .filter(|gate| gate.has_input(and_gate.out))
            .count();

        let xor_gate_out_gates = gates
            .iter()
            .filter(|gate| gate.has_input(xor_gate.out))
            .count();

        if xor_gate_out_gates == 1 && and_gate_out_gates == 2 {
            swapped.insert(xor_gate.out);
            swapped.insert(and_gate.out);
        }

        if and_gate.out.starts_with('z') {
            swapped.insert(and_gate.out);

            let output_gate = gates
                .iter()
                .find(|gate| gate.has_input(xor_gate.out) && gate.operation == Operation::XOR)
                .unwrap();

            swapped.insert(output_gate.out);
        }
    }

    let z_gates = gates
        .iter()
        .filter(|gate| gate.out.starts_with('z') && gate.out != "z45");

    for gate in z_gates {
        if gate.operation == Operation::OR {
            let initial_gate = gates
                .iter()
                .find(|g| (g.out == gate.in1 || g.out == gate.in2) && g.in1.starts_with('x'))
                .unwrap();

            let xor_gate = gates
                .iter()
                .find(|g| g.in1 == initial_gate.in1 && g.operation == Operation::XOR)
                .unwrap();

            let output_gate = gates
                .iter()
                .find(|g| g.has_input(xor_gate.out) && g.operation == Operation::XOR)
                .unwrap();

            swapped.insert(gate.out);
            swapped.insert(output_gate.out);
        }

        if gate.operation == Operation::AND && !gate.in1.starts_with('x') {
            let non_carry_input = &gates
                .iter()
                .find(|g| gate.has_input(g.out) && g.operation == Operation::XOR)
                .unwrap()
                .out;

            let output_gate = gates
                .iter()
                .find(|g| g.has_input(non_carry_input) && g.operation == Operation::XOR)
                .unwrap();

            swapped.insert(gate.out);
            swapped.insert(output_gate.out);
        }
    }

    let mut swapped = Vec::from_iter(swapped.into_iter().map(String::from));
    swapped.sort_unstable();

    Some(swapped.join(","))
}

fn parse<'a>(input: &'a str) -> (HashMap<&'a str, u8>, impl Iterator<Item = Gate<'a>>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let registers = parts[0]
        .lines()
        .map(|line| {
            let mut s = line.split(": ");
            let key = s.next().unwrap();
            let val = s.next().unwrap().parse::<u8>().unwrap();
            (key, val)
        })
        .collect();

    let gates = parts[1].lines().map(|line| Gate::from(line));

    (registers, gates)
}

struct Gate<'a> {
    in1: &'a str,
    in2: &'a str,
    out: &'a str,
    operation: Operation,
}

impl<'a> Gate<'a> {
    fn get_out(&self, in1: u8, in2: u8) -> u8 {
        match self.operation {
            Operation::XOR => in1 ^ in2,
            Operation::AND => in1 & in2,
            Operation::OR => in1 | in2,
        }
    }

    fn has_input(&self, input: &str) -> bool {
        self.in1 == input || self.in2 == input
    }
}

impl<'a> From<&'a str> for Gate<'a> {
    fn from(value: &'a str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let mut ins = [parts[0], parts[2]];
        ins.sort_unstable();

        Gate {
            in1: ins[0],
            in2: ins[1],
            out: parts[4],
            operation: Operation::from(parts[1]),
        }
    }
}

#[derive(PartialEq)]
enum Operation {
    XOR,
    AND,
    OR,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "XOR" => Operation::XOR,
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            _ => unreachable!(),
        }
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
