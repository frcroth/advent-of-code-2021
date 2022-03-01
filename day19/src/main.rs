use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn read_input(input: &str) -> Vec<Vec<(i32, i32, i32)>> {
    input
        .split("\n\n")
        .map(|scanner_block| {
            scanner_block
                .split('\n')
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .filter(|line| !line.contains("---"))
                .map(|line| -> (i32, i32, i32) {
                    let values = line
                        .split(',')
                        .map(|number| number.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    (values[0], values[1], values[2])
                })
                .collect::<Vec<(i32, i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32, i32)>>>()
}

fn calculate_distances(scanner: &Vec<(i32, i32, i32)>) -> HashMap<i32, (usize, usize)> {
    let mut distances = HashMap::new();
    for i in 0..scanner.len() {
        for j in 0..scanner.len() {
            if i != j {
                let beacon1 = scanner[i];
                let beacon2 = scanner[j];
                let distance = i32::pow(beacon1.0 - beacon2.0, 2)
                    + i32::pow(beacon1.1 - beacon2.1, 2)
                    + i32::pow(beacon1.2 - beacon2.2, 2);
                distances.insert(distance, (i, j));
            }
        }
    }
    distances
}

fn get_rotation_matrices() -> [[[i32; 3]; 3]; 24] {
    [
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
        [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
        [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
        [[0, 1, 0], [1, 0, 0], [0, -1, 0]],
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
        [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
        [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
        [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
        [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
        [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
        [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
        [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
        [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
        [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
        [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
        [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
        [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
        [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    ]
}

fn matrix_multiply_vec(matrix: &[[i32; 3]; 3], vec: (i32, i32, i32)) -> (i32, i32, i32) {
    let [first_row, second_row, third_row] = matrix;
    let [a, b, c] = first_row;
    let [d, e, f] = second_row;
    let [g, h, i] = third_row;
    let (x, y, z) = vec;
    (
        *a * x + *b * y + *c * z,
        *d * x + *e * y + *f * z,
        *g * x + *h * y + *i * z,
    )
}

fn vec_difference(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn vec_addition(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

/*
fn print_matrix(matrix: &[[i32; 3]; 3]) {
    println!(
        "[{},{},{}]\n[{},{},{}]\n[{},{},{}]",
        matrix[0][0],
        matrix[0][1],
        matrix[0][2],
        matrix[1][0],
        matrix[1][1],
        matrix[1][2],
        matrix[2][0],
        matrix[2][1],
        matrix[2][2],
    )
}*/

fn translate_scanner(
    scanner: Vec<(i32, i32, i32)>,
    rotation: [[i32; 3]; 3],
    translation_vec: (i32, i32, i32),
) -> Vec<(i32, i32, i32)> {
    scanner
        .iter()
        .map(|beacon| {
            vec_addition(
                matrix_multiply_vec(&rotation, *beacon),
                vec_difference((0, 0, 0), translation_vec),
            )
        })
        .collect::<Vec<(i32, i32, i32)>>()
}

fn manhattan_distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1) + i32::abs(a.2 - b.2)
}

fn try_anchoring(
    scanner: Vec<(i32, i32, i32)>,
    anchor: &Vec<(i32, i32, i32)>,
    distances: &Vec<HashMap<i32, (usize, usize)>>,
    scanner_index: usize,
    _anchor_index: usize,
) -> Option<(Vec<(i32, i32, i32)>, (i32, i32, i32))> {
    let anchor_distance_map = calculate_distances(anchor);

    let scanner_distances: HashSet<i32, RandomState> =
        HashSet::from_iter(distances[scanner_index].keys().cloned());
    let anchor_distances: HashSet<i32, RandomState> =
        HashSet::from_iter(anchor_distance_map.keys().cloned());
    let distance_intersection = scanner_distances
        .intersection(&anchor_distances)
        .clone()
        .map(|i| *i)
        .collect::<Vec<i32>>();
    if distance_intersection.len() < 12 {
        return None;
    }

    let mut translation_frequencies: HashMap<([[i32; 3]; 3], (i32, i32, i32)), u32> =
        HashMap::new();
    for distance in distance_intersection {
        let (scanner_beacon_1_index, scanner_beacon_2_index) = distances[scanner_index][&distance];
        let (anchor_beacon_1_index, anchor_beacon_2_index) = anchor_distance_map[&distance];
        let mut translation = None;
        'rotations: for rotation in get_rotation_matrices() {
            let scanner_beacon_1 = scanner[scanner_beacon_1_index];
            let scanner_beacon_2 = scanner[scanner_beacon_2_index];

            let anchor_beacon_1 = anchor[anchor_beacon_1_index];
            let anchor_beacon_2 = anchor[anchor_beacon_2_index];

            let rotated_scanner_beacon_1 = matrix_multiply_vec(&rotation, scanner_beacon_1);
            let rotated_scanner_beacon_2 = matrix_multiply_vec(&rotation, scanner_beacon_2);

            let translation_option = if vec_difference(rotated_scanner_beacon_1, anchor_beacon_1)
                == vec_difference(rotated_scanner_beacon_2, anchor_beacon_2)
            {
                Some((
                    rotation,
                    vec_difference(rotated_scanner_beacon_1, anchor_beacon_1),
                ))
            } else if vec_difference(rotated_scanner_beacon_2, anchor_beacon_1)
                == vec_difference(rotated_scanner_beacon_1, anchor_beacon_2)
            {
                Some((
                    rotation,
                    vec_difference(rotated_scanner_beacon_2, anchor_beacon_1),
                ))
            } else {
                None
            };
            if let Some(trans) = translation_option {
                translation = Some(trans);
                break 'rotations;
            }
        }
        if let Some(trans) = translation {
            if let Some(frequency) = translation_frequencies.get(&trans) {
                translation_frequencies.insert(trans, frequency + 1);
            } else {
                translation_frequencies.insert(trans, 1);
            }
        }
    }

    let (max_frequency, trans) = translation_frequencies.iter().fold(
        (0, None),
        |(current_max, current_trans), (trans, frequency)| {
            if *frequency > current_max {
                return (*frequency, Some(trans));
            }
            return (current_max, current_trans);
        },
    );
    if max_frequency < 12 {
        return None;
    }
    if let Some((rotation, vec_diff)) = trans {
        return Some((
            translate_scanner(scanner, *rotation, *vec_diff),
            vec_difference((0, 0, 0), *vec_diff),
        ));
    }
    None
}

fn solve(input: &str) -> Vec<(usize, Vec<(i32, i32, i32)>, (i32, i32, i32))> {
    let scanners = read_input(input);
    let distances = scanners
        .iter()
        .map(|scanner| calculate_distances(scanner))
        .collect::<Vec<HashMap<i32, (usize, usize)>>>();
    let mut normalized_scanners: Vec<(usize, Vec<(i32, i32, i32)>, (i32, i32, i32))> = vec![];
    let mut solved = HashSet::new();
    let mut unsolved = (1..scanners.len()).collect::<Vec<usize>>();
    solved.insert(0);
    normalized_scanners.push((0, scanners[0].clone(), (0, 0, 0)));

    let mut attempted_normalizations: HashMap<(usize, usize), bool> = HashMap::new();

    let mut prev_unsolved_length = usize::MAX;
    'outer: while !unsolved.is_empty() {
        if unsolved.len() == prev_unsolved_length {
            break;
        }
        prev_unsolved_length = unsolved.len();
        // iterate over scanners and try to anchor them
        for scanner_index in unsolved.clone().iter() {
            for normalized_scanner in normalized_scanners.iter() {
                if attempted_normalizations.contains_key(&(*scanner_index, normalized_scanner.0)) {
                    continue;
                }
                attempted_normalizations.insert((*scanner_index, normalized_scanner.0), true);

                let scanner = scanners[*scanner_index].clone();
                if let Some((anchored_scanner, position)) = try_anchoring(
                    scanner,
                    &normalized_scanner.1,
                    &distances,
                    *scanner_index,
                    normalized_scanner.0,
                ) {
                    solved.insert(*scanner_index);
                    unsolved.remove(unsolved.iter().position(|x| *x == *scanner_index).unwrap());
                    normalized_scanners.push((*scanner_index, anchored_scanner, position));
                    continue 'outer;
                }
            }
        }
    }
    normalized_scanners.sort_by(|a, b| a.0.cmp(&b.0));
    normalized_scanners
}

fn part_1(input: &str) -> usize {
    let normalized_scanners = solve(input);
    let mut beacons = HashSet::new();
    for scanner in normalized_scanners.iter() {
        for beacon in (*scanner).1.iter() {
            beacons.insert(beacon);
        }
    }
    beacons.len()
}

fn part_2(input: &str) -> i32 {
    let normalized_scanners = solve(input);
    let mut max = i32::MIN;
    for a in normalized_scanners.iter() {
        for b in normalized_scanners.iter() {
            max = i32::max(manhattan_distance(a.2, b.2), max)
        }
    }
    max
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(79, part_1(input));
    assert_eq!(3621, part_2(input));
}
