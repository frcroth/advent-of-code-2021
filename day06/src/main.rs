fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        part_1(
            input
                .split(",")
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );

    println!(
        "{}",
        part_2(
            input
                .split(",")
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
}

fn part_1(input: Vec<i32>) -> usize {
    iterate_n_times_array(80, input)
}

fn iterate_n_times_list(n: i32, initial: Vec<i32>) -> usize {
    let mut list = initial;
    for _ in 0..n {
        list = iterate_list(list);
    }
    list.len()
}

// Iterate through fish using a list, respecting order
fn iterate_list(initial: Vec<i32>) -> Vec<i32> {
    let mut new_list: Vec<i32> = vec![];
    let mut new_fish_count = 0;
    for fish in initial {
        if fish == 0 {
            new_list.push(6);
            new_fish_count += 1;
        } else {
            new_list.push(fish - 1);
        }
    }
    new_list.append(&mut vec![8i32; new_fish_count]);
    new_list
}

// Iterate over fish using an array, only measuring counts, ignoring order
fn iterate_n_times_array(n: i32, initial: Vec<i32>) -> usize {
    let mut fish_count = [0usize; 9];
    for fish in initial {
        fish_count[fish as usize] += 1;
    }
    for _ in 0..n {
        let mut new_fish_count = [0; 9];
        for i in 0..9 {
            if i == 0 {
                new_fish_count[8] = fish_count[i];
                new_fish_count[6] = fish_count[i];
            } else {
                new_fish_count[i - 1] += fish_count[i];
            }
        }
        fish_count = new_fish_count;
    }
    fish_count.iter().sum::<usize>()
}

fn part_2(input: Vec<i32>) -> usize {
    iterate_n_times_array(256, input)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(
        5934,
        part_1(
            input
                .split(",")
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
    assert_eq!(
        26984457539,
        part_2(
            input
                .split(",")
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect()
        )
    );
}
