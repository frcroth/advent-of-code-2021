fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn get_model_number(input: Vec<Vec<String>>, max: bool) -> i64 {
    let number_values = input
        .into_iter()
        .map(|command| {
            if command.len() == 3 {
                if let Ok(number) = command[2].parse::<i64>() {
                    Some(number)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<Option<i64>>>()
        .into_iter()
        .enumerate();

    let parameters = number_values
        .into_iter()
        .filter(|(index, _)| [4, 5, 15].iter().any(|n| *n == index % 18))
        .map(|(_, element)| element.unwrap());

    let interesting_values = parameters.into_iter().enumerate().fold(
        (0..14).map(|_| vec![]).collect::<Vec<Vec<i64>>>(),
        |mut groups, (i, v)| {
            groups[i / 3].push(v);
            groups
        },
    );

    interesting_values
        .into_iter()
        .enumerate()
        .fold(
            (vec![0; 14], vec![]),
            |(mut digits, mut stack), (i, values)| {
                let div_z = values[0];
                let c = values[1];
                let next_c = values[2];
                if div_z == 1 {
                    stack.push((next_c, i));
                } else {
                    let (prev_c, prev_i) = stack.pop().unwrap();
                    let diff = prev_c + c;
                    digits[prev_i] = if max {
                        9.min(9 - diff)
                    } else {
                        1.max(1 - diff)
                    };
                    digits[i] = digits[prev_i] + diff;
                }
                (digits, stack)
            },
        )
        .0
        .into_iter()
        .fold(0, |number, digit| number * 10 + digit)
}

fn part_1(input: &str) -> i64 {
    get_model_number(parse_input(input), true)
}

fn part_2(input: &str) -> i64 {
    get_model_number(parse_input(input), false)
}
