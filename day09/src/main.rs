use array2d::Array2D;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn create_array(input: String) -> Array2D<u32> {
    let rows = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| String::from(c).parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    Array2D::from_rows(&rows)
}

fn get_sinks(array: &Array2D<u32>) -> Vec<(usize, usize)> {
    let rows = array.num_rows();
    let cols = array.num_columns();
    let mut positions = vec![];
    for y in 0..rows {
        for x in 0..cols {
            let cell = array.get(y, x).unwrap();

            let neighbors = get_neighbors((y, x))
                .into_iter()
                .map(|n| array.get(n.0, n.1))
                .filter(|n| n.is_some())
                .map(|n| *n.unwrap())
                .collect::<Vec<u32>>();
            if neighbors.iter().all(|n| n > cell) {
                positions.push((y, x))
            }
        }
    }
    positions
}

fn get_neighbors(position: (usize, usize)) -> Vec<(usize, usize)> {
    let (y, x) = position;
    let mut neighbors = vec![(y, x + 1), (y + 1, x)];
    if x > 0 {
        neighbors.push((y, x - 1))
    };
    if y > 0 {
        neighbors.push((y - 1, x))
    };
    neighbors
}

fn part_1(input: &str) -> u32 {
    let array = create_array(input.to_string());
    get_sinks(&array)
        .into_iter()
        .map(|position| array.get(position.0, position.1).unwrap() + 1)
        .sum()
}

fn part_2(input: &str) -> u32 {
    let array = create_array(input.to_string());
    let sinks = get_sinks(&array);
    let mut basin_sizes = vec![];
    for sink in sinks {
        let mut queue = vec![sink];
        let mut size = 0;
        let mut visited = vec![];
        while !queue.is_empty() {
            size += 1;
            let current_cell = queue.pop().unwrap();
            visited.push(current_cell);
            let neighbors = get_neighbors(current_cell);
            for neighbor in neighbors {
                if let Some(value) = array.get(neighbor.0, neighbor.1) {
                    if *value < 9 && !visited.contains(&neighbor) && !queue.contains(&neighbor) {
                        queue.push(neighbor);
                    }
                }
            }
        }
        basin_sizes.push(size);
    }
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    basin_sizes
        .into_iter()
        .take(3)
        .product()
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(15, part_1(input));
    assert_eq!(1134, part_2(input));
}
