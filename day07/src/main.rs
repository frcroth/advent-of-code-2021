fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        part_1(
            input
                .split(',')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
    println!(
        "{}",
        part_2(
            input
                .split(',')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
}

fn median_on_sorted(list: &[i32]) -> i32 {
    if list.len() % 2 == 0 {
        list[list.len() / 2]
    } else {
        (list[list.len() / 2] + list[list.len() / 2 + 1]) / 2
    }
}

fn part_1(mut input: Vec<i32>) -> i32 {
    input.sort_unstable();
    let median = median_on_sorted(&input);
    let mut sum = 0;
    for n in input {
        sum += (median - n).abs()
    }
    sum
}
// This could all be very much improved by the use of *reduce*, but I can't be bothered right now
fn part_2(input: Vec<i32>) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    let mut current_min = i32::MAX;
    for x in min..=max {
        let mut sum = 0;
        for n in input.iter() {
            let diff = (*n - x).abs();
            sum += diff * (diff + 1) / 2;
        }
        current_min = if sum < current_min { sum } else { current_min };
    }
    current_min
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(
        37,
        part_1(
            input
                .split(',')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
    assert_eq!(
        168,
        part_2(
            input
                .split(',')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
}
