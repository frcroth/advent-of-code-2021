use std::iter::FromIterator;

fn main() {
    let input = include_str!("../input.txt");
    let split_input: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
    let numbers: Vec<i32> = split_input[0]
        .split(",")
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    let boards: Vec<String> = split_input[1..].to_vec();
    println!("{}", part_1(numbers.clone(), boards.clone()));
    println!("{}", part_2(numbers, boards));
}

#[derive(PartialEq)]
struct Board {
    content: [[i32; 5]; 5],
    numbers: Vec<i32>,
}

impl Board {
    fn from_string(s: String) -> Board {
        let mut board = Board {
            content: [[0; 5]; 5],
            numbers: vec![],
        };
        let board_rows: Vec<String> = s.split("\n").map(|s| s.to_string()).collect();
        for i in 0..5 {
            let numbers: Vec<i32> = Board::get_numbers_from_input_row(board_rows[i].clone());
            for j in 0..5 {
                board.content[i][j] = numbers[j];
                board.numbers.push(numbers[j]);
            }
        }
        board
    }

    fn get_numbers_from_input_row(s: String) -> Vec<i32> {
        let mut numbers: Vec<i32> = vec![];
        for i in 0..5 {
            let number = String::from_iter(
                s.as_bytes()[i * 3..i * 3 + 2]
                    .iter()
                    .map(|x| *x as char)
                    .collect::<Vec<char>>(),
            );
            numbers.push(number.trim().parse::<i32>().unwrap());
        }
        numbers
    }

    fn get_columns(&self) -> Vec<Vec<i32>> {
        let mut columns: Vec<Vec<i32>> = vec![];
        for i in 0..5 {
            let mut current_column: Vec<i32> = vec![];
            for j in 0..5 {
                current_column.push(self.content[j][i].clone());
            }
            columns.push(current_column);
        }
        columns
    }

    fn has_won(&self, numbers: &Vec<i32>) -> bool {
        let columns = self.get_columns();
        for i in 0..5 {
            // Check rows
            if self.content[i].iter().all(|x| numbers.contains(x)) {
                return true;
            }
            // Check columns
            if columns[i].iter().all(|x| numbers.contains(x)) {
                return true;
            }
        }
        false
    }

    fn get_unmarked_numbers(&self, marked_numbers: &Vec<i32>) -> Vec<i32> {
        self.numbers
            .clone()
            .into_iter()
            .filter(|x| !marked_numbers.contains(x))
            .collect()
    }

    fn sum_of_unmarked_numbers(&self, marked_numbers: &Vec<i32>) -> i32 {
        self.get_unmarked_numbers(marked_numbers)
            .into_iter()
            .reduce(|accum, x| accum + x)
            .unwrap()
    }
}

fn part_1(numbers: Vec<i32>, board_specs: Vec<String>) -> i32 {
    let boards: Vec<Board> = board_specs.into_iter().map(Board::from_string).collect();
    let mut marked_numbers: Vec<i32> = vec![];
    for number in numbers {
        marked_numbers.push(number);
        for board in boards.iter() {
            if board.has_won(&marked_numbers) {
                return board.sum_of_unmarked_numbers(&marked_numbers) * number;
            }
        }
    }
    -1
}

fn part_2(numbers: Vec<i32>, board_specs: Vec<String>) -> i32 {
    let boards: Vec<Board> = board_specs.into_iter().map(Board::from_string).collect();
    let mut marked_numbers: Vec<i32> = vec![];
    let mut eliminated_boards: Vec<&Board> = vec![];
    for number in numbers {
        marked_numbers.push(number);
        let possible_boards: Vec<&Board> = boards
            .iter()
            .filter(|b| !eliminated_boards.contains(b))
            .collect();
        for board in possible_boards.iter() {
            if board.has_won(&marked_numbers) {
                eliminated_boards.push(board);
                if possible_boards.len() == 1 {
                    return board.sum_of_unmarked_numbers(&marked_numbers) * number;
                }
            }
        }
    }
    -1
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");
    let split_input: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
    let numbers: Vec<i32> = split_input[0]
        .split(",")
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    let boards: Vec<String> = split_input[1..].to_vec();

    assert_eq!(4512, part_1(numbers.clone(), boards.clone()));
    assert_eq!(1924, part_2(numbers, boards));
}
