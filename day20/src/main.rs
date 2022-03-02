fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn parse_input(input: &str) -> (Vec<char>, Vec<Vec<char>>) {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let enhancement = split[0].chars().collect::<Vec<char>>();
    let image = split[1]
        .split('\n')
        .map(|line| line.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    (enhancement, image)
}

fn read_image_at(x: i32, y: i32, image: &[Vec<char>], infinite_space_dark: bool) -> char {
    if y > (image.len() - 1) as i32 || y < 0 || x > (image[0].len() - 1) as i32 || x < 0 {
        return if infinite_space_dark { '.' } else { '#' };
    }
    image[y as usize][x as usize]
}

fn char_vec_to_usize(chars: &[char]) -> usize {
    let exponents = chars
        .iter()
        .map(|char| match char {
            '.' => 0,
            '#' => 1,
            _ => {
                panic!()
            }
        })
        .collect::<Vec<usize>>();
    let mut val = 0;
    for i in 0..exponents.len() {
        val += exponents[i] * usize::pow(2, (exponents.len() - 1 - i) as u32);
    }
    val
}

fn get_pixel_value(x: i32, y: i32, image: &[Vec<char>], infinite_space_dark: bool) -> usize {
    char_vec_to_usize(&[
        read_image_at(x - 1, y - 1, image, infinite_space_dark),
        read_image_at(x, y - 1, image, infinite_space_dark),
        read_image_at(x + 1, y - 1, image, infinite_space_dark),
        read_image_at(x - 1, y, image, infinite_space_dark),
        read_image_at(x, y, image, infinite_space_dark),
        read_image_at(x + 1, y, image, infinite_space_dark),
        read_image_at(x - 1, y + 1, image, infinite_space_dark),
        read_image_at(x, y + 1, image, infinite_space_dark),
        read_image_at(x + 1, y + 1, image, infinite_space_dark),
    ])
}

fn generate_new_image(
    image: &[Vec<char>],
    enhancement: &[char],
    infinite_space_dark: bool,
) -> (Vec<Vec<char>>, bool) {
    let image_height = image.len() as i32;
    let image_width = image[0].len() as i32;
    let mut new_image = vec![];
    let security_offset = 2; // To account for infinite space
    for y in -security_offset..image_height + security_offset {
        let mut row = vec![];
        for x in -security_offset..image_width + security_offset {
            row.push(enhancement[get_pixel_value(x, y, image, infinite_space_dark)]);
        }
        new_image.push(row);
    }
    let empty_space_switches_color = enhancement[0] == '#';
    (
        new_image,
        if empty_space_switches_color {
            !infinite_space_dark
        } else {
            infinite_space_dark
        },
    )
}

fn count_bright_pixels(image: &[Vec<char>]) -> usize {
    let mut sum = 0;
    for y in 0..image.len() {
        for x in 0..image[0].len() {
            if image[y][x] == '#' {
                sum += 1;
            }
        }
    }
    sum
}

fn part_1(input: &str) -> usize {
    let (enhancement, initial_image) = parse_input(input);
    let (mut image, infinite_space_dark) = generate_new_image(&initial_image, &enhancement, true);
    let image = generate_new_image(&image, &enhancement, infinite_space_dark).0;
    count_bright_pixels(&image)
}

fn part_2(input: &str) -> usize {
    let (enhancement, mut image) = parse_input(input);
    let mut infinite_space_dark = true;
    for _ in 0..50 {
        let result = generate_new_image(&image, &enhancement, infinite_space_dark);
        image = result.0;
        infinite_space_dark = result.1;
    }
    count_bright_pixels(&image)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(35, part_1(input));
    assert_eq!(3351, part_2(input));
}
