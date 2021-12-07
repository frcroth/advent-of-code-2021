fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        part_1(
            input
                .split('\n')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
    println!(
        "{}",
        part_2(
            input
                .split('\n')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
}

fn part_1(input: Vec<i32>) -> i32 {
    input
        .windows(2)
        .fold(0, |acc, item| acc + if item[1] > item[0] { 1 } else { 0 })
}

fn part_2(input: Vec<i32>) -> i32 {
    let mut prev = None;
    let mut sum = 0;
    for i in input.windows(3) {
        if let Some(value) = prev {
            if i[0] + i[1] + i[2] > value {
                sum += 1;
            }
        }
        prev = Some(i[0] + i[1] + i[2]);
    }
    sum
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(
        7,
        part_1(
            input
                .split('\n')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
    assert_eq!(
        5,
        part_2(
            input
                .split('\n')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
}
