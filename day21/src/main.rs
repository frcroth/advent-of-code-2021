use memoize::memoize;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn parse_input(input: &str) -> (usize, usize) {
    let lines = input
        .split('\n')
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let player_1_start = lines[0][28].to_string().parse::<usize>().unwrap();
    let player_2_start = lines[1][28].to_string().parse::<usize>().unwrap();
    (player_1_start, player_2_start)
}

fn get_next_dice_throw(current_dice: usize) -> usize {
    if current_dice == 100 {
        return 1;
    }
    current_dice + 1
}

fn move_player(start_pos: usize, offset: usize) -> usize {
    (start_pos - 1 + offset) % 10 + 1
}

fn play_turn(start_pos: usize, dice_start: usize) -> (usize, usize) {
    let mut dice_value = dice_start;
    let mut player_position = start_pos;
    for _ in 0..3 {
        // Throw dice
        dice_value = get_next_dice_throw(dice_value);
        player_position = move_player(player_position, dice_value);
    }
    (player_position, dice_value)
}

fn part_1(input: &str) -> usize {
    let (mut player_1_pos, mut player_2_pos) = parse_input(input);
    let mut player_1_score = 0;
    let mut player_2_score = 0;
    let mut die = 0;
    let mut total_die_rolls = 0;
    while !usize::max(player_1_score, player_2_score) >= 1000 {
        // Player 1 plays
        let result = play_turn(player_1_pos, die);
        total_die_rolls += 3;
        player_1_pos = result.0;
        die = result.1;
        player_1_score += player_1_pos;
        if player_1_score >= 1000 {
            break;
        }
        // Player 2 plays
        let result = play_turn(player_2_pos, die);
        total_die_rolls += 3;
        player_2_pos = result.0;
        die = result.1;
        player_2_score += player_2_pos;
    }
    if player_1_score > player_2_score {
        player_2_score * total_die_rolls
    } else {
        player_1_score * total_die_rolls
    }
}

#[memoize]
fn play_round_part_2(
    player_pos: usize, // The player who's turn it is
    other_player_pos: usize,
    player_score: u64,
    other_player_score: u64,
) -> (u64, u64) {
    if player_score >= 21 {
        return (1, 0);
    }
    if other_player_score >= 21 {
        return (0, 1);
    }

    let die_sum_distribution = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    die_sum_distribution.iter().fold(
        (0, 0),
        |(total_player_wins, total_other_player_wins), (die_sum, count)| {
            let next_pos = (player_pos - 1 + die_sum) % 10 + 1;
            let new_score = player_score + (next_pos as u64);

            let (other_wins, player_wins) =
                play_round_part_2(other_player_pos, next_pos, other_player_score, new_score);
            (
                total_player_wins + player_wins * count,
                total_other_player_wins + other_wins * count,
            )
        },
    )
}

fn part_2(input: &str) -> u64 {
    let (player_1_pos, player_2_pos) = parse_input(input);
    let (winner_wins, loser_wins) = play_round_part_2(player_1_pos, player_2_pos, 0, 0);
    u64::max(winner_wins, loser_wins)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(739785, part_1(input));
    assert_eq!(444356092776315, part_2(input));
}
