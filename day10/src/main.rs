use core::str::Split;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input.split("\n")));
    println!("{}", part_2(input.split("\n")));
}

fn get_opening_bracket(closing_bracket: char) -> char {
    match closing_bracket {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => {
            panic!()
        }
    }
}

fn get_error_score(closing_bracket: char) -> u64 {
    match closing_bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => {
            panic!()
        }
    }
}

fn get_incomplete_score(opening_bracket: char) -> u64 {
    match opening_bracket {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => {
            panic!()
        }
    }
}

fn part_1(input: Split<&str>) -> u64 {
    calculate_score(input, false)
}

fn part_2(input: Split<&str>) -> u64 {
    calculate_score(input, true)
}

fn calculate_score(input: Split<&str>, score_incomplete: bool) -> u64 {
    let lines = input
        .map(|s| s.to_string().chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut scores = vec![];
    let mut corruption_score = 0;
    for line in lines {
        let mut chunk_stack = vec![];
        let mut corrupt = false;
        for char in line {
            if char == '(' || char == '[' || char == '{' || char == '<' {
                chunk_stack.push(char);
            }
            if char == ')' || char == ']' || char == '}' || char == '>' {
                if let Some(val) = chunk_stack.last() {
                    let correct_opening = get_opening_bracket(char);
                    if *val == correct_opening {
                        // everything correct!
                        chunk_stack.pop();
                    } else {
                        // corrupt
                        corrupt = true;
                        if !score_incomplete {
                            corruption_score += get_error_score(char);
                        }
                        break;
                    }
                }
            }
        }
        if corrupt || !score_incomplete {
            continue;
        }
        // line is incomplete!
        let mut line_score = 0u64;
        chunk_stack.reverse();
        for char in chunk_stack {
            line_score *= 5;
            line_score += get_incomplete_score(char);
        }
        scores.push(line_score);
    }
    if !score_incomplete {
        return corruption_score;
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(26397, part_1(input.split("\n")));
    assert_eq!(288957, part_2(input.split("\n")));
}
