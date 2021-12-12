use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn parse_input(input: &str) -> Vec<(String, String)> {
    let rules = input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut connections = vec![];
    for rule in rules {
        let rule_split = rule
            .split('-')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let origin = rule_split[0].clone();
        let destination = rule_split[1].clone();
        connections.push((origin.clone(), destination.clone()));
        connections.push((destination.clone(), origin.clone()));
    }
    connections
}

fn cave_is_small(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn valid_in_part_1(cave: String, path: &[String]) -> bool {
    !cave_is_small(&cave) || !path.contains(&cave)
}

fn valid_in_part_2(cave: String, path: &[String], small_cave_visited_twice: bool) -> bool {
    (!cave_is_small(&cave) || (!path.contains(&cave) || !small_cave_visited_twice))
        && cave != "start"
}

fn traverse_cave_system(connections: Vec<(String, String)>, part_2_rules: bool) -> usize {
    let mut paths = vec![];
    let mut queue: VecDeque<(String, (Vec<String>, bool))> = VecDeque::new();
    queue.push_back(("start".to_string(), (vec![], false)));
    while !queue.is_empty() {
        let (current_cave, path_and_scvt) = queue.pop_front().unwrap();
        let (mut path, mut small_cave_visited_twice) = path_and_scvt;
        if path.contains(&current_cave) && cave_is_small(&current_cave) {
            small_cave_visited_twice = true;
        }
        path.push(current_cave.clone());
        if current_cave == "end" {
            paths.push(path);
            continue;
        }
        let targets = connections
            .iter()
            .filter(|connection| {
                connection.0 == current_cave
                    && if part_2_rules {
                        valid_in_part_2(connection.1.clone(), &path, small_cave_visited_twice)
                    } else {
                        valid_in_part_1(connection.1.clone(), &path)
                    }
            })
            .map(|connection| connection.1.clone())
            .collect::<Vec<String>>();
        for target in targets {
            queue.push_back((target, (path.clone(), small_cave_visited_twice)));
        }
    }
    paths.len()
}

fn part_1(input: &str) -> usize {
    let connections = parse_input(input);
    traverse_cave_system(connections, false)
}

fn part_2(input: &str) -> usize {
    let connections = parse_input(input);
    traverse_cave_system(connections, true)
}

#[test]
fn test_example_1() {
    assert_eq!(10, part_1(include_str!("../example-1.txt")));
    assert_eq!(36, part_2(include_str!("../example-1.txt")));
}
#[test]
fn test_example_2() {
    assert_eq!(19, part_1(include_str!("../example-2.txt")));
    assert_eq!(103, part_2(include_str!("../example-2.txt")));
}
#[test]
fn test_example_3() {
    assert_eq!(226, part_1(include_str!("../example-3.txt")));
    assert_eq!(3509, part_2(include_str!("../example-3.txt")));
}
