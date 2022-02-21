use core::fmt::Formatter;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

#[derive(Clone, Copy)]
enum Token {
    Number(i32),
    Open,
    Close,
    Separator,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Token::Number(val) => write!(f, "{}", format!("{}", val)),
            Token::Open => write!(f, "{}", String::from('[')),
            Token::Close => write!(f, "{}", String::from(']')),
            Token::Separator => write!(f, "{}", String::from(',')),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Token>> {
    input
        .split('\n')
        .map(|line| parse_line(&line.to_string()))
        .collect::<Vec<Vec<Token>>>()
}

fn parse_line(input: &str) -> Vec<Token> {
    // In the input, all numbers are single digits!
    input
        .chars()
        .map(|c| match c {
            '[' => Token::Open,
            ']' => Token::Close,
            ',' => Token::Separator,
            _ => Token::Number(c.to_string().parse::<i32>().unwrap()),
        })
        .collect::<Vec<Token>>()
}

fn reduce(tokens: Vec<Token>) -> Vec<Token> {
    let mut tokens = tokens;
    loop {
        // print_tokens(&tokens);
        // Find leftmost pair nested in 4 pairs
        let left_most_pair = get_left_most_pair_nested_in_4_index(&tokens);
        if let Some(pair_to_explode) = left_most_pair {
            // println!("Exploding at {}", pair_to_explode);
            tokens = explode(&tokens, pair_to_explode);
            continue;
        }
        // Find regular numbers that are too big
        let number_too_big = get_first_regular_right_at_least_10(&tokens);
        if let Some(big_num_index) = number_too_big {
            // println!("Splitting at {}", big_num_index);
            tokens = split(&tokens, big_num_index);
            continue;
        }

        break;
    }
    tokens
}

fn get_left_most_pair_nested_in_4_index(tokens: &[Token]) -> Option<usize> {
    let mut nesting_level = 0;
    for (i, current_token) in tokens.iter().enumerate() {
        match current_token {
            Token::Open => {
                if nesting_level == 4 {
                    return Some(i);
                }
                nesting_level += 1;
            }
            Token::Close => {
                nesting_level -= 1;
            }
            _ => {}
        }
    }
    None
}

fn get_first_regular_right_of(tokens: &[Token], index: usize) -> Option<(i32, usize)> {
    for (i, token) in tokens.iter().enumerate().skip(index) {
        if let Token::Number(val) = token {
            return Some((*val, i));
        }
    }
    None
}

fn get_first_regular_right_at_least_10(tokens: &Vec<Token>) -> Option<usize> {
    for (i, token) in tokens.iter().enumerate() {
        if let Token::Number(val) = token {
            if *val >= 10 {
                return Some(i);
            }
        }
    }
    None
}

fn get_first_regular_left_of(tokens: &[Token], index: usize) -> Option<(i32, usize)> {
    for i in (0..=index).rev() {
        if let Token::Number(val) = tokens[i] {
            return Some((val, i));
        }
    }
    None
}

fn explode(tokens: &Vec<Token>, pair_index: usize) -> Vec<Token> {
    let mut new_tokens = (*tokens).clone();
    let left_value = get_first_regular_right_of(&new_tokens, pair_index)
        .unwrap()
        .0;
    let right_value = get_first_regular_right_of(&new_tokens, pair_index + 2)
        .unwrap()
        .0;

    if let Some((value, index)) = get_first_regular_left_of(&new_tokens, pair_index) {
        new_tokens[index] = Token::Number(value + left_value);
    }

    if let Some((value, index)) = get_first_regular_right_of(&new_tokens, pair_index + 5) {
        new_tokens[index] = Token::Number(value + right_value);
    }

    // Remove old pair
    for _ in 0..5 {
        new_tokens.remove(pair_index);
    }
    new_tokens.insert(pair_index, Token::Number(0));

    new_tokens
}

fn split(tokens: &Vec<Token>, big_number_index: usize) -> Vec<Token> {
    let mut new_tokens = (*tokens).clone();

    if let Token::Number(current_value) = new_tokens[big_number_index] {
        let new_left = current_value / 2;
        let new_right = current_value / 2 + if current_value % 2 == 1 { 1 } else { 0 };

        new_tokens.remove(big_number_index);
        // Inserting in reverse order
        new_tokens.insert(big_number_index, Token::Close);
        new_tokens.insert(big_number_index, Token::Number(new_right));
        new_tokens.insert(big_number_index, Token::Separator);
        new_tokens.insert(big_number_index, Token::Number(new_left));
        new_tokens.insert(big_number_index, Token::Open);
    }

    new_tokens
}

fn add(first: &mut Vec<Token>, second: &mut Vec<Token>) -> Vec<Token> {
    let mut new_token = vec![Token::Open];
    new_token.append(first);
    new_token.push(Token::Separator);
    new_token.append(second);
    new_token.push(Token::Close);
    reduce(new_token)
}

fn get_magnitude(tokens: &[Token], start: usize, end: usize) -> i32 {
    if end == start + 1 {
        if let Token::Number(value) = tokens[start] {
            return value;
        }
    } else {
        // At start there is a [
        let mut nesting_level = 0;
        for i in start + 1..end - 1 {
            match tokens[i] {
                Token::Open => {
                    nesting_level += 1;
                }
                Token::Close => {
                    nesting_level -= 1;
                }
                Token::Separator => {
                    if nesting_level == 0 {
                        let left_magnitude = 3 * get_magnitude(tokens, start + 1, i);
                        let right_magnitude = 2 * get_magnitude(tokens, i + 1, end - 1);
                        return left_magnitude + right_magnitude;
                    }
                }
                _ => {}
            }
        }
    }
    panic!();
}

fn print_tokens(tokens: &[Token]) {
    println!(
        "{}",
        tokens
            .iter()
            .fold("".to_string(), |acc, s| format!("{}{}", acc, s))
    );
}

fn part_1(input: &str) -> i32 {
    let mut terms = parse_input(input);
    let mut result = terms[0].clone();
    for term in terms.iter_mut().skip(1) {
        result = add(&mut result,  term);
    }
    print_tokens(&result);
    get_magnitude(&result, 0, result.len())
}

fn part_2(input: &str) -> i32 {
    let terms = parse_input(input);
    let mut max = i32::MIN;
    for x in 0..terms.len() {
        for y in 0..terms.len() {
            if x == y {
                continue;
            }
            let result = add(&mut terms[x].clone(), &mut terms[y].clone());
            max = i32::max(max, get_magnitude(&result, 0, result.len()));
        }
    }
    max
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(4140, part_1(input));
    assert_eq!(3993, part_2(input));
}
