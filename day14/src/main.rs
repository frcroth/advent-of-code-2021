use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

struct Rule {
    source: (char, char),
    result: char,
}

impl Rule {
    fn from_str(string: &str) -> Rule {
        let content = string
            .split(" -> ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Rule {
            source: (
                content[0].chars().nth(0).unwrap(),
                content[0].chars().nth(1).unwrap(),
            ),
            result: content[1].chars().nth(0).unwrap(),
        }
    }
}

fn read_input(input: &str) -> (String, Vec<Rule>) {
    let content = input
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let initial = (&content[0]).to_string();
    let rules = content[1]
        .split('\n')
        .map(|line| Rule::from_str(&line.to_string()))
        .collect::<Vec<Rule>>();
    (initial, rules)
}

fn apply_rules(polymer: String, rules: &[Rule]) -> String {
    let mut result = vec![];
    for (char1, char2) in polymer.chars().tuple_windows() {
        result.push(char1);
        if let Some(rule) = rules.iter().find(|elem| elem.source == (char1, char2)) {
            result.push(rule.result);
        }
    }
    result.push(polymer.chars().last().unwrap());
    result.into_iter().collect::<String>()
}

fn part_1(input: &str) -> usize {
    let (initial, rules) = read_input(input);
    let mut polymer = initial;
    for _ in 0..10 {
        polymer = apply_rules(polymer, &rules);
    }

    let mut frequencies = HashMap::new();
    for char in polymer.chars() {
        if let Some(count) = frequencies.get(&char) {
            frequencies.insert(char, count + 1);
        } else {
            frequencies.insert(char, 1);
        }
    }
    let mut occurrences = frequencies.into_iter().collect::<Vec<(char, usize)>>();
    occurrences.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let most_common = occurrences[occurrences.len() - 1].1;
    let least_common = occurrences[0].1;
    most_common - least_common
}

fn part_2(input: &str) -> u64 {
    let (initial, rules) = read_input(input);
    let mut pairs = HashMap::new();
    for (char1, char2) in initial.chars().tuple_windows() {
        if let Some(count) = pairs.get(&(char1, char2)) {
            pairs.insert((char1, char2), count + 1);
        } else {
            pairs.insert((char1, char2), 1_u64);
        }
    }

    let mut letter_count = HashMap::new();
    for char in initial.chars() {
        if let Some(prev_count) = letter_count.get(&char) {
            letter_count.insert(char, prev_count + 1);
        } else {
            letter_count.insert(char, 1);
        }
    }

    for _ in 0..40 {
        let mut new_pairs = pairs.clone();
        for ((char1, char2), count) in pairs.iter() {
            if let Some(rule) = rules.iter().find(|elem| elem.source == (*char1, *char2)) {
                if let Some(count_of_new_pair) = new_pairs.get(&(*char1, rule.result)) {
                    new_pairs.insert((*char1, rule.result), count_of_new_pair + count);
                } else {
                    new_pairs.insert((*char1, rule.result), *count);
                }
                if let Some(count_of_new_pair) = new_pairs.get(&(rule.result, *char2)) {
                    new_pairs.insert((rule.result, *char2), count_of_new_pair + count);
                } else {
                    new_pairs.insert((rule.result, *char2), *count);
                }

                // Add newly inserted letter to letter count
                if let Some(prev_count) = letter_count.get(&rule.result) {
                    letter_count.insert(rule.result, prev_count + *count);
                } else {
                    letter_count.insert(rule.result, *count);
                }
                new_pairs.insert(
                    (*char1, *char2),
                    new_pairs.get(&(*char1, *char2)).unwrap() - count,
                );
            }
        }
        pairs = new_pairs;
    }

    let mut occurrences = letter_count
        .values()
        .map(|n| *n as u64)
        .collect::<Vec<u64>>();

    occurrences.sort_unstable();
    let most_common = occurrences[occurrences.len() - 1];
    let least_common = occurrences[0];
    most_common - least_common
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(1588, part_1(input));
    assert_eq!(2188189693529, part_2(input));
}
