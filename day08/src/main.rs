use core::str::Split;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input.split("\n")));
    println!("{}", part_2(input.split("\n")));
}

fn part_1(input: Split<&str>) -> i32 {
    let mut two_segments = 0;
    let mut three_segments = 0;
    let mut four_segments = 0;
    let mut seven_segments = 0;
    for line in input {
        let digit_part = &line
            .split('|')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()[1];
        let digits = digit_part.trim().split(' ').map(|s| s.to_string());
        for digit in digits {
            match digit.len() {
                2 => {
                    two_segments += 1;
                }
                3 => {
                    three_segments += 1;
                }
                4 => {
                    four_segments += 1;
                }
                7 => {
                    seven_segments += 1;
                }
                _ => {}
            }
        }
    }
    two_segments + three_segments + four_segments + seven_segments
}

fn get_digit(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool) -> i32 {
    if a && b && c && !d && e && f && g {
        return 0;
    }
    if !a && !b && c && !d && !e && f && !g {
        return 1;
    }
    if a && !b && c && d && e && !f && g {
        return 2;
    }
    if a && !b && c && d && !e && f && g {
        return 3;
    }
    if !a && b && c && d && !e && f && !g {
        return 4;
    }
    if a && b && !c && d && !e && f && g {
        return 5;
    }
    if a && b && !c && d && e && f && g {
        return 6;
    }
    if a && !b && c && !d && !e && f && !g {
        return 7;
    }
    if a && b && c && d && e && f && g {
        return 8;
    }
    if a && b && c && d && !e && f && g {
        return 9;
    }
    -1
}

fn intersections(a: &str, b: &str) -> String {
    let mut chars = vec![];
    for c in a.chars() {
        if b.contains(c) {
            chars.push(c);
        }
    }
    chars.into_iter().collect::<String>()
}

fn intersection_count(a: &str, b: &str) -> usize {
    intersections(a, b).len()
}

fn part_2(input: Split<&str>) -> i32 {
    let mut decoded_number_sum = 0;
    for line in input {
        let line_split = &line
            .split('|')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let signal_part = line_split[0]
            .clone()
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let digit_part = line_split[1]
            .clone()
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let one = signal_part.iter().filter(|n| n.len() == 2).next().unwrap();
        let four = signal_part.iter().filter(|n| n.len() == 4).next().unwrap();
        let seven = signal_part.iter().filter(|n| n.len() == 3).next().unwrap();
        let eight = signal_part.iter().filter(|n| n.len() == 7).next().unwrap();
        let six = signal_part
            .iter()
            .filter(|n| n.len() == 6 && intersection_count(one, n) == 1)
            .next()
            .unwrap();
        let f = intersections(one, six);
        let c = String::from(
            one.chars()
                .filter(|x| String::from(*x) != f)
                .next()
                .unwrap(),
        );
        let three = signal_part
            .iter()
            .filter(|n| n.len() == 5 && n.contains(&f) && n.contains(&c))
            .next()
            .unwrap();
        let b = String::from(
            four.chars()
                .filter(|x| !three.contains(&String::from(*x)))
                .next()
                .unwrap(),
        );
        let five = signal_part
            .iter()
            .filter(|n| n.len() == 5 && n.contains(&b))
            .next()
            .unwrap();
        let d = four
            .chars()
            .map(|x| String::from(x))
            .filter(|x| !one.contains(x) && *x != b)
            .next()
            .unwrap();
        let nine = signal_part
            .iter()
            .filter(|n| n.len() == 6 && n.contains(&d) && n.contains(&c))
            .next()
            .unwrap();
        let e = eight
            .chars()
            .map(|x| String::from(x))
            .filter(|x| !nine.contains(x))
            .next()
            .unwrap();
        let g = five
            .chars()
            .map(|x| String::from(x))
            .filter(|x| !seven.contains(x) && !four.contains(x))
            .next()
            .unwrap();
        let a = seven
            .chars()
            .map(|x| String::from(x))
            .filter(|x| !four.contains(x))
            .next()
            .unwrap();
        let mut digits = vec![];
        for digit_string in digit_part {
            let digit = get_digit(
                digit_string.contains(&a),
                digit_string.contains(&b),
                digit_string.contains(&c),
                digit_string.contains(&d),
                digit_string.contains(&e),
                digit_string.contains(&f),
                digit_string.contains(&g),
            );
            digits.push(digit);
        }
        let decoded = digits.iter().fold(0, |acc, elem| acc * 10 + elem);
        decoded_number_sum += decoded;
    }
    decoded_number_sum
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(26, part_1(input.split("\n")));
    assert_eq!(61229, part_2(input.split("\n")));
}
