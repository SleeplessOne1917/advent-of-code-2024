use crate::utils::read_lines;

pub fn solution1() {
    let lines = read_lines("src/day3/input.txt");
    let sum = lines
        .map(|line| {
            let mut sum = 0;
            let mut mul_bytes = Vec::new();
            for byte in line.bytes() {
                match (byte, mul_bytes.last()) {
                    (b'm', None)
                    | (b'u', Some(b'm'))
                    | (b'l', Some(b'u'))
                    | (b'(', Some(b'l'))
                    | (b'0'..=b'9', Some(b'(' | b'0'..=b'9' | b','))
                    | (b',', Some(b'0'..=b'9')) => mul_bytes.push(byte),
                    (b')', Some(b'0'..=b'9')) => {
                        let first_num_index = mul_bytes
                            .iter()
                            .position(u8::is_ascii_digit)
                            .expect("Guarunteed to be there");
                        let comma_index = mul_bytes
                            .iter()
                            .position(|&c| c == b',')
                            .expect("Guarunteed to be there.");

                        let num1 = bytes_to_num(&mul_bytes[first_num_index..comma_index]);
                        let num2 = bytes_to_num(&mul_bytes[comma_index + 1..]);

                        sum += num1 * num2;
                        mul_bytes.clear();
                    }
                    _ => mul_bytes.clear(),
                }
            }

            sum
        })
        .sum::<usize>();

    println!("Sum of multiplication results = {sum}");
}

fn bytes_to_num(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| (*digit - b'0') as usize * 10usize.pow(i as u32))
        .sum::<usize>()
}
