use array2d::Array2D;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

#[derive(Clone)]
struct Octopus {
    val: i32,
    has_flashed: bool,
}

fn get_grid(input: &str) -> Array2D<Octopus> {
    let rows = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| Octopus {
                    val: String::from(c).parse::<i32>().unwrap(),
                    has_flashed: false,
                })
                .collect::<Vec<Octopus>>()
        })
        .collect::<Vec<Vec<Octopus>>>();
    Array2D::from_rows(&rows)
}

fn perform_flash(array: &mut Array2D<Octopus>, target: (i32, i32)) -> Vec<(i32, i32)> {
    let (y, x) = target;
    let target_cell = array.get_mut(y as usize, x as usize).unwrap();
    if target_cell.has_flashed {
        return vec![];
    }
    target_cell.has_flashed = true;

    let neighboring_positions = vec![
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ]
    .into_iter()
    .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < 10 && pos.1 < 10)
    .collect::<Vec<(i32, i32)>>();

    let mut interesting_locations: Vec<(i32, i32)> = vec![];
    for neighboring_position in neighboring_positions {
        let (n_y, n_x) = neighboring_position;
        if let Some(cell) = array.get_mut(n_y as usize, n_x as usize) {
            cell.val += 1;
            if cell.val > 9 && !cell.has_flashed {
                interesting_locations.push((n_y, n_x));
            }
        }
    }
    interesting_locations
}

fn iterate_array(array: &mut Array2D<Octopus>) -> i32 {
    let rows = 10;
    let cols = 10;
    for y in 0..rows {
        for x in 0..cols {
            let prev_value = array.get(y, x).unwrap().val;
            array.set(
                y,
                x,
                Octopus {
                    val: prev_value + 1,
                    has_flashed: false,
                },
            );
        }
    }

    let mut flash_targets: Vec<(i32, i32)> = vec![];
    for y in 0..rows {
        for x in 0..cols {
            let cell = array.get(y, x).unwrap();
            if cell.val > 9 {
                // flash!
                flash_targets.append(&mut perform_flash(array, (y as i32, x as i32)));
            }
        }
    }

    while !flash_targets.is_empty() {
        let location = flash_targets.pop().unwrap();
        flash_targets.append(&mut perform_flash(array, location));
    }

    let mut flash_count = 0;
    for y in 0..rows {
        for x in 0..cols {
            let cell = array.get_mut(y, x).unwrap();
            if cell.has_flashed {
                flash_count += 1;
                cell.val = 0;
                cell.has_flashed = false;
            }
        }
    }
    flash_count
}

fn print_array(array: &Array2D<Octopus>) {
    for row in array.rows_iter() {
        let string = row
            .map(|oct| oct.val)
            .fold(String::from(""), |acc, elem| format!("{}{}", acc, elem));
        println!("{}", string);
    }
    println!("\n");
}

fn part_1(input: &str) -> i32 {
    let mut flash_count = 0;
    let mut array = get_grid(input);
    let iterations = 100;
    for _ in 0..iterations {
        flash_count += iterate_array(&mut array);
    }
    flash_count
}

fn has_everyone_flashed(array: &Array2D<Octopus>) -> bool {
    let rows = 10;
    let cols = 10;
    for y in 0..rows {
        for x in 0..cols {
            let cell = array.get(y, x).unwrap();
            if cell.val != 0 {
                return false;
            }
        }
    }
    true
}

fn part_2(input: &str) -> i32 {
    let mut array = get_grid(input);
    let max_iterations = 1000;
    for i in 0..max_iterations {
        iterate_array(&mut array);
        if has_everyone_flashed(&array) {
            return i + 1;
        }
    }
    -1
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(1656, part_1(input));
    assert_eq!(195, part_2(input));
}
