use std::iter::repeat;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, compact_one))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, compact_two))
}

fn solve(input: &str, compact: fn(&mut [Block])) -> usize {
    let mut blocks: Vec<Block> = input
        .chars()
        .filter(|c| c != &'\n')
        .enumerate()
        .flat_map(|(i, ch)| {
            let amount = ch.to_digit(10).unwrap().try_into().unwrap();
            let el = if i % 2 == 0 {
                Block::File(i / 2)
            } else {
                Block::Free
            };
            repeat(el).take(amount)
        })
        .collect();

    compact(&mut blocks);

    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, block)| match block {
            Block::Free => None,
            Block::File(id) => Some(id * i),
        })
        .sum()
}

fn compact_one(blocks: &mut [Block]) {
    let mut left = 0;
    let mut right = blocks.len() - 1;

    while left < right {
        let (left_block, right_block) = (&blocks[left], &blocks[right]);
        match (left_block, right_block) {
            (_, Block::Free) => {
                right -= 1;
            }
            (Block::File(_), _) => {
                left += 1;
            }
            (_, _) => {
                blocks.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
    }
}

fn compact_two(blocks: &mut [Block]) {
    let mut left = 0;
    let mut right = blocks.len() - 1;

    while left < right {
        let (left_block, right_block) = (&blocks[left], &blocks[right]);
        match (left_block, right_block) {
            (_, Block::Free) => {
                right -= 1;
            }
            (Block::File(_), _) => {
                left += 1;
            }
            (_, Block::File(id)) => {
                let mut file_size = 1;
                let mut tmp_file = right - 1;
                while left < tmp_file && blocks[tmp_file] == Block::File(*id) {
                    file_size += 1;
                    tmp_file -= 1;
                }

                let mut tmp_free = left;
                let mut free_space_start_idx = None;
                while tmp_free < tmp_file {
                    let free_start = tmp_free;
                    let mut free_size = 0;

                    while blocks[tmp_free] == Block::Free {
                        free_size += 1;
                        tmp_free += 1;
                    }

                    if free_size >= file_size {
                        free_space_start_idx = Some(free_start);
                        break;
                    }

                    tmp_free += 1;
                }

                if let Some(mut start) = free_space_start_idx {
                    for _ in 0..file_size {
                        blocks.swap(start, right);
                        start += 1;
                        right -= 1;
                    }
                } else {
                    right -= file_size;
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum Block {
    Free,
    File(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
