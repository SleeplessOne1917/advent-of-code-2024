use crate::utils::read_lines;

pub fn solution1() {
    let lines = read_lines("src/day3/input.txt");
    let sum = lines
        .map(|line| {
            let mut sum = 0;
            let mut command_bytes = Vec::new();
            for byte in line.bytes() {
                match (byte, command_bytes.as_slice()) {
                    (b')', [.., b'0'..=b'9']) => {
                        handle_mul(&mut command_bytes, &mut sum);
                    }
                    _ if matches_mul(byte, &command_bytes) => {
                        command_bytes.push(byte);
                    }
                    _ => {
                        command_bytes.clear();
                    }
                }
            }

            sum
        })
        .sum::<usize>();

    println!("Sum of multiplication results = {sum}");
}

pub fn solution2() {
    let lines = read_lines("src/day3/input.txt");

    let mut can_mul = true;
    let sum = lines
        .map(|line| {
            let mut sum = 0;
            let mut command_bytes = Vec::new();
            for byte in line.bytes() {
                match (byte, command_bytes.as_slice()) {
                    (b')', [.., b'0'..=b'9']) if can_mul => {
                        handle_mul(&mut command_bytes, &mut sum);
                    }
                    (b')', [b'd', b'o', b'(']) => {
                        can_mul = true;
                        command_bytes.clear();
                    }
                    (b')', [.., b't', b'(']) => {
                        can_mul = false;
                        command_bytes.clear();
                    }
                    _ if matches_do_or_dont(byte, &command_bytes)
                        || matches_mul(byte, &command_bytes) =>
                    {
                        command_bytes.push(byte);
                    }
                    _ => {
                        command_bytes.clear();
                    }
                }
            }

            sum
        })
        .sum::<usize>();

    println!("Sum of enabled multiplication results = {sum}");
}

fn matches_mul(byte: u8, command_bytes: &[u8]) -> bool {
    matches!(
        (byte, command_bytes),
        (b'm', [])
            | (b'u', [.., b'm'])
            | (b'l', [.., b'u'])
            | (b'(', [.., b'l'])
            | (b'0'..=b'9', [.., b'(' | b'0'..=b'9' | b','])
            | (b',', [.., b'0'..=b'9'])
    )
}

fn matches_do_or_dont(byte: u8, command_bytes: &[u8]) -> bool {
    matches!(
        (byte, command_bytes),
        (b'd', [])
            | (b'o', [.., b'd'])
            | (b'n', [.., b'o'])
            | (b'\'', [.., b'n'])
            | (b'(', [.., b'o' | b't'])
            | (b't', [.., b'\''])
    )
}

fn handle_mul(command_bytes: &mut Vec<u8>, sum: &mut usize) {
    let first_num_index = command_bytes
        .iter()
        .position(u8::is_ascii_digit)
        .expect("Guarunteed to be there");
    let comma_index = command_bytes
        .iter()
        .position(|&c| c == b',')
        .expect("Guarunteed to be there.");

    let num1 = bytes_to_num(&command_bytes[first_num_index..comma_index]);
    let num2 = bytes_to_num(&command_bytes[comma_index + 1..]);

    *sum += num1 * num2;
    command_bytes.clear();
}

fn bytes_to_num(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| (*digit - b'0') as usize * 10usize.pow(i as u32))
        .sum::<usize>()
}
