use array2d::Array2D;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Field {
    East,
    South,
    Empty,
}

impl Field {
    fn from_char(c: char) -> Field {
        match c {
            '>' => Field::East,
            'v' => Field::South,
            _ => Field::Empty,
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Field::Empty)
    }

    fn is_east(&self) -> bool {
        matches!(self, Field::East)
    }

    fn is_south(&self) -> bool {
        matches!(self, Field::South)
    }
}

fn parse_input(input: &str) -> Array2D<Field> {
    Array2D::from_rows(
        &input
            .split('\n')
            .map(|line| line.chars().map(Field::from_char).collect::<Vec<Field>>())
            .collect::<Vec<Vec<Field>>>(),
    )
}

fn perform_movement(floor: &Array2D<Field>) -> Array2D<Field> {
    let width = floor.row_len();
    let height = floor.column_len();
    // Moving east
    let mut emptied_positions = vec![];
    let mut positions_moved_in = vec![];
    for y in 0..height {
        for x in 0..width {
            let field = floor.get(y, x).unwrap();
            if field.is_east() {
                let next_pos = (if x + 1 >= width { 0 } else { x + 1 }, y);
                if floor.get(next_pos.1, next_pos.0).unwrap().is_empty() {
                    emptied_positions.push((x, y));
                    positions_moved_in.push(next_pos);
                }
            }
        }
    }
    let mut floor = floor.clone();
    for position in emptied_positions {
        floor.set(position.1, position.0, Field::Empty).unwrap();
    }
    for position in positions_moved_in {
        floor.set(position.1, position.0, Field::East).unwrap();
    }

    emptied_positions = vec![];
    positions_moved_in = vec![];
    for y in 0..height {
        for x in 0..width {
            let field = floor.get(y, x).unwrap();
            if field.is_south() {
                let next_pos = (x, if y + 1 >= height { 0 } else { y + 1 });
                if floor.get(next_pos.1, next_pos.0).unwrap().is_empty() {
                    emptied_positions.push((x, y));
                    positions_moved_in.push(next_pos);
                }
            }
        }
    }
    let mut floor = floor.clone();
    for position in emptied_positions {
        floor.set(position.1, position.0, Field::Empty).unwrap();
    }
    for position in positions_moved_in {
        floor.set(position.1, position.0, Field::South).unwrap();
    }
    floor
}

fn print_floor(floor: &Array2D<Field>) {
    for row in floor.rows_iter() {
        println!(
            "{}",
            row.map(|field| match field {
                Field::East => '>',
                Field::South => 'v',
                Field::Empty => '.',
            })
            .fold("".to_string(), |acc, s| format!("{}{}", acc, s))
        );
    }
}

fn part_1(input: &str) -> usize {
    let mut floor = parse_input(input);
    let max_iterations = 1000;
    for i in 1..max_iterations {
        let new_floor = perform_movement(&floor);
        if new_floor.eq(&floor) {
            print_floor(&floor);
            return i;
        }
        floor = new_floor;
    }
    0
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(58, part_1(input));
}
