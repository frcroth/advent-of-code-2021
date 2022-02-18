fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn input_to_binary_vec(input: &str) -> Vec<char> {
    let mut binary_vec = vec![];
    for c in input.chars() {
        let binary_string = format!("{:b}", i64::from_str_radix(&c.to_string(), 16).unwrap());
        let mut binary_conversion = binary_string.chars().collect::<Vec<char>>();
        while binary_conversion.len() < 4 {
            binary_conversion.insert(0, '0');
        }
        binary_vec.append(&mut binary_conversion);
    }
    binary_vec
}

struct Packet {
    version: i32,
    type_id: i32,
    length: usize,
    literal_content: Option<i64>,
    sub_packet_length: Option<SubPacketLength>,
    sub_packets: Option<Vec<Packet>>,
}

enum SubPacketLength {
    Total(usize),
    Number(i32),
}

fn convert_binary_char_slice_to_i64(input: &[char]) -> i64 {
    i64::from_str_radix(
        &input
            .iter()
            .fold("".to_string(), |acc, c| format!("{}{}", acc, c.to_string())),
        2,
    )
    .unwrap()
}

fn read_packet(input: &[char]) -> Packet {
    // read version number
    let version_number_slice = &input[0..3];
    let version_number = convert_binary_char_slice_to_i64(version_number_slice) as i32;

    // read type
    let type_slice = &input[3..6];
    let type_id = convert_binary_char_slice_to_i64(type_slice) as i32;

    let mut new_packet = Packet {
        version: version_number,
        type_id,
        length: usize::MAX, // not yet determined
        literal_content: None,
        sub_packet_length: None,
        sub_packets: None,
    };

    match type_id {
        4 => {
            // Literal
            let mut value_vec = vec![];
            let mut group_index = 0;
            let reading_start = 6;
            let mut continue_reading = true;
            while continue_reading {
                value_vec.append(
                    &mut (input
                        [reading_start + 5 * group_index + 1..reading_start + 5 * group_index + 5]
                        .iter()
                        .copied()
                        .collect::<Vec<char>>())
                    .clone(),
                );

                let continue_bit = input[reading_start + 5 * group_index];
                if continue_bit == '0' {
                    continue_reading = false;
                } else {
                    group_index += 1;
                }
            }
            let value = convert_binary_char_slice_to_i64(&value_vec[..]);
            new_packet.literal_content = Some(value);
            let length_read = reading_start + 5 * group_index + 5;
            new_packet.length = length_read;
        }
        _ => {
            // Operator
            let length_id = input[6];
            let mut sub_packets = vec![];
            match length_id {
                '0' => {
                    let sub_packet_length =
                        convert_binary_char_slice_to_i64(&input[7..22]) as usize;

                    new_packet.sub_packet_length = Some(SubPacketLength::Total(sub_packet_length));
                    new_packet.length = 22 + sub_packet_length;
                    let mut current_packet_index = 22;
                    while current_packet_index < 22 + sub_packet_length {
                        let sub_packet = read_packet(&input[current_packet_index..]);
                        current_packet_index += sub_packet.length;
                        sub_packets.push(sub_packet);
                    }
                }
                '1' => {
                    let sub_packet_number = convert_binary_char_slice_to_i64(&input[7..18]);
                    new_packet.sub_packet_length =
                        Some(SubPacketLength::Number(sub_packet_number as i32));
                    let mut current_packet_index = 18;
                    for _ in 0..sub_packet_number {
                        let sub_packet = read_packet(&input[current_packet_index..]);
                        current_packet_index += sub_packet.length;
                        sub_packets.push(sub_packet);
                    }
                    new_packet.length = 18 + sub_packets.iter().fold(0, |acc, p| acc + p.length);
                }
                _ => {}
            }
            new_packet.sub_packets = Some(sub_packets);
        }
    }
    new_packet
}

fn get_version_number_sum(packet: &Packet) -> i32 {
    let mut sum = packet.version;
    if let Some(sub_packets) = &packet.sub_packets {
        sum += sub_packets
            .iter()
            .fold(0, |acc, p| acc + get_version_number_sum(p));
    }
    sum
}

fn get_value(packet: &Packet) -> i64 {
    if let Some(num) = packet.literal_content {
        num
    } else {
        let sub_packets = (&packet.sub_packets).as_ref().unwrap();
        match packet.type_id {
            0 => sub_packets.iter().fold(0, |acc, p| acc + get_value(p)),
            1 => sub_packets.iter().fold(1, |acc, p| acc * get_value(p)),
            2 => sub_packets
                .iter()
                .fold(i64::MAX, |acc, p| i64::min(acc, get_value(p))),
            3 => sub_packets
                .iter()
                .fold(i64::MIN, |acc, p| i64::max(acc, get_value(p))),
            5 => {
                if get_value(&sub_packets[0]) > get_value(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                if get_value(&sub_packets[0]) < get_value(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                if get_value(&sub_packets[0]) == get_value(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            _ => -1,
        }
    }
}

fn part_1(input: &str) -> i32 {
    let binary_input = input_to_binary_vec(input);
    let root_packet = read_packet(&binary_input[..]);
    get_version_number_sum(&root_packet)
}

fn part_2(input: &str) -> i64 {
    let binary_input = input_to_binary_vec(input);
    let root_packet = read_packet(&binary_input[..]);
    get_value(&root_packet)
}

#[test]
fn test_part_1() {
    assert_eq!(16, part_1(include_str!("../example1.txt")));
    assert_eq!(12, part_1(include_str!("../example2.txt")));
    assert_eq!(23, part_1(include_str!("../example3.txt")));
    assert_eq!(31, part_1(include_str!("../example4.txt")));
}

#[test]
fn test_part_2() {
    assert_eq!(3, part_2(include_str!("../example_p2_1.txt")));
    assert_eq!(54, part_2(include_str!("../example_p2_2.txt")));
    assert_eq!(7, part_2(include_str!("../example_p2_3.txt")));
    assert_eq!(9, part_2(include_str!("../example_p2_4.txt")));
    assert_eq!(1, part_2(include_str!("../example_p2_5.txt")));
    assert_eq!(0, part_2(include_str!("../example_p2_6.txt")));
    assert_eq!(0, part_2(include_str!("../example_p2_7.txt")));
    assert_eq!(1, part_2(include_str!("../example_p2_8.txt")));
}
