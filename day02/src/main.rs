use core::str::Split;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input.split("\n")));
    println!("{}", part_2(input.split("\n")));
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn from_str(string: &str) -> Command {
        let string_split: Vec<&str> = string.split(" ").collect();
        let digit = string_split[1];
        match string_split[0] {
            "forward" => Command::Forward(digit.parse::<i32>().unwrap()),
            "down" => Command::Down(digit.parse::<i32>().unwrap()),
            "up" => Command::Up(digit.parse::<i32>().unwrap()),
            _ => panic!(),
        }
    }
}

fn part_1(input: Split<&str>) -> i32 {
    let commands: Vec<Command> = input.map(Command::from_str).collect();
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands.iter() {
        match command {
            Command::Forward(n) => horizontal += n,
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n,
        };
    }
    horizontal * depth
}

fn part_2(input: Split<&str>) -> i32 {
    let commands: Vec<Command> = input.map(Command::from_str).collect();
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(n) => {
                horizontal += n;
                depth += aim * n;
            }
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n,
        };
    }
    horizontal * depth
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(150, part_1(input.split("\n")));
    assert_eq!(900, part_2(input.split("\n")));
}
