use core::str::Split;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input.split("\n")));
    println!("{}", part_2(input.split("\n")));
}

fn part_1(input: Split<&str>) -> i32 {
    let numbers: Vec<String> = input.map(|s| s.to_string()).collect();
    let input_length = numbers.len() as i32;
    let number_length = numbers[0].chars().count();
    let mut frequencies: Vec<i32> = vec![0; number_length];
    for number in numbers {
        let mut pos = 0;
        for c in number.chars() {
            match c {
                '1' => {
                    frequencies[pos] += 1;
                }
                _ => { //Do nothing
                }
            }
            pos += 1;
        }
    }
    let mut gamma_string: String = String::from("");
    let mut epsilon_string: String = String::from("");
    for i in 0..number_length {
        let gamma_char = if frequencies[i] > input_length / 2 {
            1
        } else {
            0
        };
        let epsilon_char = if frequencies[i] <= input_length / 2 {
            1
        } else {
            0
        };
        gamma_string = format!("{}{}", gamma_string, gamma_char);
        epsilon_string = format!("{}{}", epsilon_string, epsilon_char);
    }
    let gamma_value = i32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon_value = i32::from_str_radix(&epsilon_string, 2).unwrap();
    gamma_value * epsilon_value
}

fn part_2(input: Split<&str>) -> i32 {
    let numbers: Vec<_> = input
        .map(|s| s.to_string())
        .map(|number| number.trim().chars().collect::<Vec<_>>())
        .collect();
    let number_length = numbers[0].len();
    let mut possible_oxygen_values = numbers.clone();
    let mut possible_co2_values = numbers.clone();
    let mut oxygen_found = false;
    let mut co2_found = false;
    for i in 0..number_length {
        if possible_oxygen_values.len() == 1 {
            oxygen_found = true;
        }
        if possible_co2_values.len() == 1 {
            co2_found = true;
        }
        if !oxygen_found {
            let mut count = 0;
            for j in 0..possible_oxygen_values.len() {
                if possible_oxygen_values[j][i] == '1' {
                    count += 1;
                }
            }
            if count * 2 >= possible_oxygen_values.len() {
                possible_oxygen_values = possible_oxygen_values
                    .into_iter()
                    .filter(|row| row[i] == '1')
                    .collect();
            } else {
                possible_oxygen_values = possible_oxygen_values
                    .into_iter()
                    .filter(|row| row[i] == '0')
                    .collect();
            }
        }
        if !co2_found {
            let mut count = 0;
            for j in 0..possible_co2_values.len() {
                if possible_co2_values[j][i] == '1' {
                    count += 1;
                }
            }
            if count * 2 < possible_co2_values.len() {
                possible_co2_values = possible_co2_values
                    .into_iter()
                    .filter(|row| row[i] == '1')
                    .collect();
            } else {
                possible_co2_values = possible_co2_values
                    .into_iter()
                    .filter(|row| row[i] == '0')
                    .collect();
            }
        }
    }
    let oxygen_value =
        i32::from_str_radix(&possible_oxygen_values[0].iter().collect::<String>(), 2).unwrap();
    let co2_value =
        i32::from_str_radix(&possible_co2_values[0].iter().collect::<String>(), 2).unwrap();
    oxygen_value * co2_value
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(198, part_1(input.split("\n")));
    assert_eq!(230, part_2(input.split("\n")));
}
