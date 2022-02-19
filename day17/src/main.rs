fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn get_target_coordinates(input: &str) -> ((i32, i32), (i32, i32)) {
    let input = String::from(input).replace(",", "");
    // This is stupid and should be done with regular expressions
    let x_part = &input
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[2];
    let x_numbers = x_part
        .split('=')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1]
        .to_string()
        .split("..")
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let y_part = &input
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[3];
    let y_numbers = y_part
        .split('=')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1]
        .to_string()
        .split("..")
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    ((x_numbers[0], x_numbers[1]), (y_numbers[0], y_numbers[1]))
}

fn part_1(input: &str) -> i32 {
    let target_y_min = get_target_coordinates(input).1 .0;
    (-target_y_min - 1) * (-target_y_min) / 2
}

fn is_velocity_valid(starting_velocity: (i32, i32), target: &((i32, i32), (i32, i32))) -> bool {
    let mut position = (0, 0);
    let mut velocity = starting_velocity;
    while position.0 <= target.0 .1 {
        if position.0 >= target.0 .0
            && position.0 <= target.0 .1
            && position.1 >= target.1 .0
            && position.1 <= target.1 .1
        {
            return true;
        }
        if velocity.1 < 0 && position.1 < target.1 .0 {
            return false;
        }
        position.0 += velocity.0;
        position.1 += velocity.1;
        velocity.0 += match velocity.0 {
            0 => 0,
            i32::MIN..=-1 => 1,
            1..=i32::MAX => -1,
        };
        velocity.1 -= 1;
    }
    false
}

fn part_2(input: &str) -> usize {
    let target = get_target_coordinates(input);
    let min_starting_y_velocity = -i32::abs(target.1 .0);
    let max_starting_y_velocity = i32::abs(target.1 .0);

    let mut possible_velocities = vec![];
    for y in min_starting_y_velocity..=max_starting_y_velocity {
        for x in 1..=target.0 .1 {
            if is_velocity_valid((x, y), &target) {
                possible_velocities.push((x, y));
            }
        }
    }
    possible_velocities.len()
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(45, part_1(input));
    assert_eq!(112, part_2(input));
}
