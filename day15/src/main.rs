use array2d::Array2D;
use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn create_cave(input: &str) -> Array2D<i32> {
    let rows = input
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    Array2D::from_rows(&rows)
}

fn create_part_2_cave(input: &str) -> Array2D<i32> {
    let mut rows = vec![];
    let rows_once = input
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|row| {
            let mut new_row = vec![];
            new_row.append(&mut (row.clone()));
            for i in 0..4 {
                for n in row.iter() {
                    new_row.push((n + i) % 9 + 1);
                }
            }
            new_row
        })
        .collect::<Vec<Vec<i32>>>();
    rows.append(&mut rows_once.clone());
    for i in 0..4 {
        rows.append(
            &mut rows_once
                .clone()
                .iter()
                .map(|row| {
                    (*row)
                        .iter()
                        .map(|n| (*n + i) % 9 + 1)
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>(),
        );
    }
    Array2D::from_rows(&rows)
}

fn calculate_path(input: &str, part_2: bool) -> usize {
    let cave = if part_2 {
        create_part_2_cave(input)
    } else {
        create_cave(input)
    };

    let extent = cave.column_len();
    let mut pq = PriorityQueue::new();
    for a in 0..extent {
        for b in 0..extent {
            pq.push((a, b), i32::MIN);
        }
    }
    pq.change_priority(&(0, 0), 0);
    let destination = (extent - 1, extent - 1);
    let mut destination_distance = 0;
    let mut current: (usize, usize);
    while pq.get(&(destination.0, destination.1)).is_some() {
        let current_distance;
        if let Some(next_cell) = pq.pop() {
            current = next_cell.0;
            current_distance = next_cell.1;
            if current == destination {
                destination_distance = current_distance;
                break;
            }
        } else {
            break;
        }

        let mut neighbors = vec![(current.0 + 1, current.1), (current.0, current.1 + 1)];
        if current.0.checked_sub(1).is_some() {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.1.checked_sub(1).is_some() {
            neighbors.push((current.0, current.1 - 1));
        }

        for (neighbor_x, neighbor_y) in neighbors.into_iter() {
            if let Some(cost) = cave.get(neighbor_x, neighbor_y) {
                if pq.get(&(neighbor_x, neighbor_y)).is_none() {
                    // Not in Priority Queue, therefore already visited
                    continue;
                }
                let neighbor_distance = *(pq.get(&(neighbor_x, neighbor_y)).unwrap().1);
                if current_distance - cost > neighbor_distance {
                    pq.change_priority(&(neighbor_x, neighbor_y), current_distance - cost);
                }
            }
        }

        destination_distance = *pq.get(&destination).unwrap().1;
    }

    (-destination_distance) as usize
}

fn part_1(input: &str) -> usize {
    calculate_path(input, false)
}

fn part_2(input: &str) -> usize {
    calculate_path(input, true)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(40, part_1(input));
    assert_eq!(315, part_2(input));
}
