use array2d::Array2D;
use std::cmp::max;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    part_2(input);
}

struct Fold {
    x: bool,
    index: u32,
}

fn parse_input(input: &str) -> (HashSet<(u32, u32)>, Vec<Fold>) {
    let parts = input
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let paper = HashSet::from_iter(
        parts[0]
            .split('\n')
            .map(|s| {
                let pair = s
                    .to_string()
                    .split(',')
                    .map(|s2| s2.to_string().parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                (pair[0], pair[1])
            })
            .collect::<Vec<(u32, u32)>>(),
    );

    let folds = parts[1]
        .split('\n')
        .map(|s| {
            let fold_spec = s
                .to_string()
                .split(' ')
                .map(|s2| s2.to_string())
                .collect::<Vec<String>>()[2]
                .split('=')
                .map(|s2| s2.to_string())
                .collect::<Vec<String>>();
            Fold {
                x: fold_spec[0] == "x",
                index: fold_spec[1].parse::<u32>().unwrap(),
            }
        })
        .collect::<Vec<Fold>>();
    (paper, folds)
}

fn part_1(input: &str) -> usize {
    let (mut paper, folds) = parse_input(input);
    let first_fold = &folds[0];
    let fold_index = first_fold.index;

    let mut points_to_remove = vec![];
    if first_fold.x {
        for point in paper.iter() {
            if point.0 > fold_index {
                points_to_remove.push(*point);
            }
        }
        for point in points_to_remove.into_iter() {
            paper.remove(&point);
            let diff = point.0 - fold_index;
            let new_point = (fold_index - diff, point.1);
            paper.insert(new_point);
        }
    } else {
        let mut points_to_remove = vec![];
        for point in paper.iter() {
            if point.1 > fold_index {
                points_to_remove.push(*point);
            }
        }
        for point in points_to_remove.into_iter() {
            paper.remove(&point);
            let diff = point.1 - fold_index;
            let new_point = (point.0, fold_index - diff);
            paper.insert(new_point);
        }
    }
    paper.len()
}

fn part_2(input: &str) {
    let (mut paper, folds) = parse_input(input);

    for fold in folds {
        let fold_index = fold.index;
        let mut points_to_remove = vec![];
        if fold.x {
            for point in paper.iter() {
                if point.0 > fold_index {
                    points_to_remove.push(*point);
                }
            }
            for point in points_to_remove.into_iter() {
                paper.remove(&point);
                let diff = point.0 - fold_index;
                let new_point = (fold_index - diff, point.1);
                paper.insert(new_point);
            }
        } else {
            for point in paper.iter() {
                if point.1 > fold_index {
                    points_to_remove.push(*point);
                }
            }
            for point in points_to_remove.into_iter() {
                paper.remove(&point);
                let diff = point.1 - fold_index;
                let new_point = (point.0, fold_index - diff);
                paper.insert(new_point);
            }
        }
    }

    let mut max_x = 0;
    let mut max_y = 0;
    for point in paper.iter() {
        max_x = max(point.0, max_x);
        max_y = max(point.1, max_y);
    }
    let mut matrix = Array2D::filled_with('-', (max_y + 1) as usize, (max_x + 1) as usize);
    for point in paper {
        matrix.set(point.1 as usize, point.0 as usize, '#').unwrap();
    }

    for row in matrix.as_rows() {
        println!("{}", row.iter().collect::<String>());
    }

    // You have to read the letters yourself ;)
}

#[test]
fn test_example() {
    assert_eq!(17, part_1(include_str!("../example.txt")));
}
