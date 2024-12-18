use advent_of_code::NUM_RE;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut program = Program::from(input);

    let result = program
        .run()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let program = Program::from(input);

    let len = program.instructions.len();

    let mut shift = 3 * (len);
    let mut starts = vec![1 << shift - 3];

    for i in (0..len).rev() {
        shift -= 3;
        starts = find_block(&program, program.instructions[i], starts, shift, i);
    }

    starts.into_iter().min()
}

fn find_block(
    program: &Program,
    target_num: usize,
    starts: Vec<usize>,
    shift: usize,
    instruction_idx: usize,
) -> Vec<usize> {
    let mut block_starts = vec![];

    for start in starts {
        for i in 0..8 {
            let block_start = start + (1 << shift) * i;
            let mut program = Program {
                register_a: block_start,
                ..program.clone()
            };
            let outputs = program.run();

            if outputs[instruction_idx] == target_num {
                block_starts.push(block_start);
            }
        }
    }

    block_starts
}

#[derive(Clone)]
struct Program {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    stack_pointer: usize,
    instructions: Vec<usize>,
}

impl Program {
    fn run(&mut self) -> Vec<usize> {
        let mut outputs = Vec::new();

        loop {
            if self.stack_pointer > self.instructions.len() - 1 {
                break outputs;
            }

            let op_code = self.instructions[self.stack_pointer];
            let operand = self.instructions[self.stack_pointer + 1];

            let output = match op_code {
                0 | 6 | 7 => {
                    let numerator = self.register_a;
                    let denominator = 1 << self.combo_operand(operand);
                    let result = numerator / denominator;

                    let register = match op_code {
                        0 => &mut self.register_a,
                        6 => &mut self.register_b,
                        _ => &mut self.register_c,
                    };
                    *register = result;
                    self.stack_pointer += 2;
                    None
                }
                1 => {
                    self.register_b = self.register_b ^ operand;
                    self.stack_pointer += 2;
                    None
                }
                2 => {
                    self.register_b = self.combo_operand(operand) % 8;
                    self.stack_pointer += 2;
                    None
                }
                3 => {
                    self.stack_pointer = if self.register_a == 0 {
                        self.stack_pointer + 2
                    } else {
                        operand
                    };
                    None
                }
                4 => {
                    self.register_b = self.register_b ^ self.register_c;
                    self.stack_pointer += 2;
                    None
                }
                5 => {
                    let out = self.combo_operand(operand) % 8;
                    self.stack_pointer += 2;
                    Some(out)
                }
                _ => panic!(),
            };

            if let Some(output) = output {
                outputs.push(output);
            }
        }
    }

    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            _ if operand <= 3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            _ => self.register_c,
        }
    }
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let mut nums = NUM_RE.find_iter(value).map(|x| x.as_str().parse().unwrap());

        Self {
            register_a: nums.next().unwrap(),
            register_b: nums.next().unwrap(),
            register_c: nums.next().unwrap(),
            stack_pointer: 0,
            instructions: nums.collect(),
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
