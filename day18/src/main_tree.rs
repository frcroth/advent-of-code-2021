use crate::Element::Pair;
use crate::Element::Regular;

/*
    This was the first iteration, using a tree data structure, it did not work out

 */

fn main() {
    let input = include_str!("../example.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

struct SFNumber {
    parent: Option<Box<SFNumber>>,
    left: Box<Element>,
    right: Box<Element>,
}

impl PartialEq for SFNumber {
    fn eq(&self, other: &Self) -> bool {
        *self.left == *other.left && *self.right == *other.right
    }
}

impl SFNumber {
    fn reduce(&mut self) {
        loop {
            // Find leftmost pair nested in 4 pairs
            let nest_count = 0;
            let left_most_pair = get_pair_nested_in_4_pairs(self, 0);
            if let Some(number_to_explode) = left_most_pair {
                number_to_explode.explode();
                continue;
            }
            // Find regular numbers that are too big
            let number_too_big = get_leftmost_number(self, true);
            if let Some((node_to_split, left)) = number_too_big {
                node_to_split.split(left);
            }
        }
    }

    fn explode(&mut self) {
        // Exploding pairs always consist of two regular numbers

        let mut left_found = false;
        let mut right_found = false;

        let mut current_node = self.get_parent_mut();
        let mut prev_node = self;

        loop {
            let came_from_left = (if let Pair(sfn) = *current_node.left {
                sfn == *current_node
            } else {
                false
            });

            if came_from_left && !right_found {
                let current_right = &current_node.right;
                if let Regular(current_right_value) = **current_right {
                    current_node.right = Box::new(Regular(
                        current_right_value + self.right.get_regular_value(),
                    ));
                    right_found = true;
                } else if let Pair(current_node_right_node) = **current_right {
                    if let Some((sfn, direction)) =
                        get_leftmost_number(&mut current_node_right_node, false)
                    {
                        if !direction {
                            println!("This should not happen!");
                            panic!();
                        }
                        sfn.left = Box::new(Regular(
                            sfn.left.get_regular_value() + self.right.get_regular_value(),
                        ));
                        right_found = true;
                    }
                }
            } else if !came_from_left && !left_found {
                let current_left = &current_node.left;
                if let Regular(current_left_value) = **current_left {
                    current_node.left = Box::new(Regular(
                        current_left_value + self.left.get_regular_value(),
                    ));
                    left_found = true;
                } else if let Pair(current_node_left_node) = **current_left {
                    if let Some((sfn, direction)) = get_rightmost_number(&mut current_node_left_node) {
                        if direction {
                            println!("This should not happen!");
                            panic!();
                        }
                        sfn.right = Box::new(Regular(
                            sfn.right.get_regular_value() + self.left.get_regular_value(),
                        ));
                        left_found = true;
                    }
                }
            }

            if current_node.is_root() {
                break;
            }
            if left_found && right_found {
                break;
            }
            prev_node = current_node;
            current_node = current_node.get_parent_mut();
        }
    }

    fn split(&mut self, left: bool) {
        if left {
            let current_value = self.left.get_regular_value();
            let new_left = current_value / 2;
            let new_right = current_value / 2 + if current_value % 2 == 1 { 1 } else { 0 };
            let new_element = Pair(SFNumber {
                parent: Some(Box::new(*self)),
                left: Box::new(Regular(new_left)),
                right: Box::new(Regular(new_right)),
            });
            self.left = Box::new(new_element);
        } else {
            let current_value = self.right.get_regular_value();
            let new_left = current_value / 2;
            let new_right = current_value / 2 + if current_value % 2 == 1 { 1 } else { 0 };
            let new_element = Pair(SFNumber {
                parent: Some(Box::new(*self)),
                left: Box::new(Regular(new_left)),
                right: Box::new(Regular(new_right)),
            });
            self.right = Box::new(new_element);
        }
    }

    // called during second step of parsing
    fn assign_parent(&mut self, parent: &SFNumber) {
        self.parent = Some(Box::new(*parent));
        if let Pair(sfn) = *self.left {
            sfn.assign_parent(self);
        }
        if let Pair(sfn) = *self.right {
            sfn.assign_parent(self);
        }
    }

    fn is_root(&self) -> bool {
        if let Some(parent) = self.parent {
            return *parent == *self;
        }
        return false;
    }

    fn get_parent(&self) -> &SFNumber {
        if let Some(parent) = self.parent {
            return &parent;
        }
        panic!();
    }

    fn get_parent_mut(&mut self) -> &mut SFNumber {
        if let Some(parent) = self.parent {
            return &mut parent;
        }
        panic!();
    }
}

fn get_leftmost_number(node: &mut SFNumber, filter_geq_10: bool) -> Option<(&mut SFNumber, bool)> {
    match *node.left {
        Regular(num) => {
            if filter_geq_10 {
                if num >= 10 {
                    return Some((node, true));
                }
            } else {
                return Some((node, true));
            }
        }
        Pair(sfn) => {
            let left_results = get_leftmost_number(&mut sfn, filter_geq_10);
            if left_results.is_some() {
                return left_results;
            }
        }
    }
    match *node.right {
        Regular(num) => {
            if filter_geq_10 {
                if num >= 10 {
                    return Some((node, false));
                }
            } else {
                return Some((node, false));
            }
        }
        Pair(sfn) => {
            let right_results = get_leftmost_number(&mut sfn, filter_geq_10);
            if right_results.is_some() {
                return right_results;
            }
        }
    }
    return None;
}

fn get_rightmost_number(node: &mut SFNumber) -> Option<(&mut SFNumber, bool)> {
    match *node.right {
        Regular(_) => {
            return Some((node, false));
        }
        Pair(sfn) => {
            let right_results = get_rightmost_number(&mut sfn);
            if right_results.is_some() {
                return right_results;
            }
        }
    }
    match *node.left {
        Regular(_) => {
            return Some((node, true));
        }
        Pair(sfn) => {
            let left_results = get_rightmost_number(&mut sfn);
            if left_results.is_some() {
                return left_results;
            }
        }
    }

    return None;
}

fn get_pair_nested_in_4_pairs(node: &mut SFNumber, current_nesting_level: u32) -> Option<&mut SFNumber> {
    if current_nesting_level == 4 {
        return Some(node);
    }
    if let Pair(left_child) = *node.left {
        let left_found = get_pair_nested_in_4_pairs(&mut left_child, current_nesting_level + 1);
        if left_found.is_some() {
            return left_found;
        }
    }
    if let Pair(right_child) = *node.right {
        let right_found = get_pair_nested_in_4_pairs(&mut right_child, current_nesting_level + 1);
        if right_found.is_some() {
            return right_found;
        }
    }
    return None;
}

#[derive(PartialEq)]
enum Element {
    Regular(u32),
    Pair(SFNumber),
}

impl Element {
    fn is_regular(&self) -> bool {
        matches!(*self, Regular(_))
    }
    fn get_regular_value(&self) -> u32 {
        match self {
            Regular(num) => *num,
            Pair(sfn) => {
                panic!()
            }
        }
    }
}

fn parse_snailfish_number(input: &str, is_root: bool) -> SFNumber {
    let mut index: usize = 1;
    let mut depth = 0;
    let mut middle_index = 0;
    let input_chars = input.chars().collect::<Vec<char>>();
    while index < input.len() - 1 {
        match input_chars[index] {
            '[' => {
                depth += 1;
            }
            ']' => {
                depth -= 1;
            }
            ',' => {
                if depth == 0 {
                    middle_index = index;
                    break;
                }
            }
            _ => {}
        };
        index += 1;
    }
    let left = if let Ok(num) = input[1..middle_index].parse::<u32>() {
        Regular(num)
    } else {
        Pair(parse_snailfish_number(&input[1..middle_index], false))
    };
    let right = if let Ok(num) = input[middle_index + 1..input.len() - 1].parse::<u32>() {
        Regular(num)
    } else {
        Pair(parse_snailfish_number(
            &input[middle_index + 1..input.len() - 1],
            false,
        ))
    };
    let mut new_node = SFNumber {
        parent: None,
        left: Box::new(left),
        right: Box::new(right),
    };
    if is_root {
        new_node.assign_parent(&new_node);
    }
    return new_node;
}

fn part_1(input: &str) -> i32 {
    let sf_num = parse_snailfish_number(input, true);
    -1
}

fn part_2(input: &str) -> usize {
    0
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(4140, part_1(input));
    assert_eq!(112, part_2(input));
}
