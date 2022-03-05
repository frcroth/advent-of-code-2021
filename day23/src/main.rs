use memoize::memoize;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .map(|line| line.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn extend_for_part_2(setup: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut setup = setup;
    setup.insert(3, "  #D#C#B#A#".chars().collect::<Vec<char>>());
    setup.insert(4, "  #D#B#A#C#".chars().collect::<Vec<char>>());
    setup
}

/*
fn print_setup(setup: &Vec<Vec<char>>) {
    print!(
        "{}",
        setup
            .iter()
            .map(|line| (*line).iter().collect::<String>())
            .collect::<Vec<String>>()
            .iter()
            .fold("".to_string(), |acc, l| format!("{}{}\n", acc, l))
    );
}
*/

fn get_room_index(amphipod: char) -> usize {
    match amphipod {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!(),
    }
}

fn get_room_size(setup: &Vec<Vec<char>>) -> usize {
    setup.len() - 3
}

fn is_amphipod(setup: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let found_char = setup[y][x];
    ['A', 'B', 'C', 'D'].contains(&found_char)
}

fn is_in_room(y: usize) -> bool {
    y >= 2
}

fn is_in_correct_room(setup: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if !is_amphipod(setup, x, y) {
        return false;
    }
    x == get_room_index(setup[y][x]) && is_in_room(y)
}

fn is_room_complete(setup: &Vec<Vec<char>>, x: usize) -> bool {
    for y in 2..2 + get_room_size(setup) {
        if !is_in_correct_room(setup, x, y) {
            return false;
        }
    }
    true
}

fn is_room_empty(setup: &Vec<Vec<char>>, x: usize) -> bool {
    for y in 2..2 + get_room_size(setup) {
        if !field_is_empty(setup, x, y) {
            return false;
        }
    }
    true
}

fn is_done(setup: &Vec<Vec<char>>) -> bool {
    is_room_complete(setup, 3)
        && is_room_complete(setup, 5)
        && is_room_complete(setup, 7)
        && is_room_complete(setup, 9)
}

fn can_move(setup: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    (!is_in_correct_room(setup, x, y) || is_blocking_room(setup, x, y))
        && !is_blocked_in_room(setup, x, y)
}

fn is_blocking_room(setup: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    for i in (y + 1)..2 + get_room_size(setup) {
        if is_amphipod(setup, x, i) && !is_in_correct_room(setup, x, i) {
            return true;
        }
    }
    return false;
}

fn field_is_empty(setup: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    setup[y][x] == '.'
}

fn is_blocked_in_room(setup: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if !is_in_room(y) {
        return false;
    }
    !field_is_empty(setup, x, y - 1)
}

fn get_movement_cost_for_amphipod(amphipod: char) -> usize {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

fn get_move_cost(setup: &Vec<Vec<char>>, x: usize, y: usize, i: usize, j: usize) -> usize {
    ((y - 1) + i32::abs(x as i32 - i as i32) as usize + (j - 1))
        * get_movement_cost_for_amphipod(setup[y][x])
}

fn is_space_in_room_for(setup: &Vec<Vec<char>>, amphipod: char) -> bool {
    let target = get_room_index(amphipod);
    if is_room_empty(setup, target) {
        return true;
    }
    for y in 2..2 + get_room_size(setup) {
        if !field_is_empty(setup, target, y) && !is_in_correct_room(setup, target, y) {
            return false;
        }
    }
    true
}

fn is_path_empty(setup: &Vec<Vec<char>>, x: usize, dest_x: usize) -> bool {
    let mut x = x;
    while x != dest_x {
        if x > dest_x {
            x -= 1;
        }
        if x < dest_x {
            x += 1;
        }
        if !field_is_empty(setup, x, 1) {
            return false;
        }
    }
    true
}

fn next_y_in_room(setup: &Vec<Vec<char>>, room_index: usize) -> usize {
    for y in (2..2 + get_room_size(setup)).rev() {
        if field_is_empty(setup, room_index, y) {
            return y;
        }
    }
    panic!();
}

fn move_amphipod(
    setup: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dest_x: usize,
    dest_y: usize,
) -> Vec<Vec<char>> {
    let mut new_setup = setup.clone();
    new_setup[dest_y][dest_x] = setup[y][x];
    new_setup[y][x] = '.';
    new_setup
}

fn get_y_indices_not_in_front_of_rooms() -> [usize; 7] {
    [1, 2, 4, 6, 8, 10, 11]
}

#[memoize]
fn calculate_min_for_setup(setup: Vec<Vec<char>>) -> Option<usize> {
    if is_done(&setup) {
        return Some(0);
    }
    let mut costs = vec![];
    for y in 1..setup.len() {
        for x in 1..setup[y].len() {
            if !is_amphipod(&setup, x, y) {
                continue;
            }
            let amphipod = setup[y][x];
            if can_move(&setup, x, y) {
                let target_room_index = get_room_index(amphipod);
                if is_space_in_room_for(&setup, amphipod)
                    && is_path_empty(&setup, x, target_room_index)
                {
                    let target_y = next_y_in_room(&setup, target_room_index);
                    let cost_of_step = get_move_cost(&setup, x, y, target_room_index, target_y);
                    let new_setup = move_amphipod(&setup, x, y, target_room_index, target_y);
                    let further_cost = calculate_min_for_setup(new_setup);
                    if let Some(cost) = further_cost {
                        costs.push(cost + cost_of_step);
                    }
                } else if is_in_room(y) {
                    for i in get_y_indices_not_in_front_of_rooms() {
                        if !is_path_empty(&setup, x, i) {
                            continue;
                        }
                        let cost_of_step = get_move_cost(&setup, x, y, i, 1);
                        let new_setup = move_amphipod(&setup, x, y, i, 1);
                        let further_cost = calculate_min_for_setup(new_setup);
                        if let Some(cost) = further_cost {
                            costs.push(cost + cost_of_step);
                        }
                    }
                }
            }
        }
    }
    costs.into_iter().reduce(usize::min)
}

fn part_1(input: &str) -> usize {
    calculate_min_for_setup(parse_input(input)).unwrap()
}

fn part_2(input: &str) -> usize {
    calculate_min_for_setup(extend_for_part_2(parse_input(input))).unwrap()
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(12521, part_1(input));
    assert_eq!(44169, part_2(input));
}
