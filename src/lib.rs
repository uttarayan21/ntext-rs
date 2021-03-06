mod test;
/// Convert digit to text.
pub fn digit_to_text(digit: u8) -> Option<String> {
    match digit {
        0 => Some("".to_string()),
        1 => Some("one".to_string()),
        2 => Some("two".to_string()),
        3 => Some("three".to_string()),
        4 => Some("four".to_string()),
        5 => Some("five".to_string()),
        6 => Some("six".to_string()),
        7 => Some("seven".to_string()),
        8 => Some("eight".to_string()),
        9 => Some("nine".to_string()),
        _ => None,
    }
}

fn tens_place(tens: u8, ones: u8) -> Option<String> {
    match tens {
        0 => digit_to_text(ones),
        1 => match ones {
            0 => Some("ten".to_string()),
            1 => Some("eleven".to_string()),
            2 => Some("twelve".to_string()),
            3 => Some("thirteen".to_string()),
            4 => Some("fourteen".to_string()),
            5 => Some("fifteen".to_string()),
            6 => Some("sixteen".to_string()),
            7 => Some("seventeen".to_string()),
            8 => Some("eighteen".to_string()),
            9 => Some("nineteen".to_string()),
            _ => None,
        },
        d @ 2..=5 | d @ 8 => Some({
            let mut buffer = String::new();
            match d {
                2 => buffer.push_str("twenty"),
                3 => buffer.push_str("thirty"),
                4 => buffer.push_str("forty"),
                5 => buffer.push_str("fifty"),
                8 => buffer.push_str("eighty"),
                _ => (),
            }
            buffer.push_str(digit_to_text(ones).unwrap().as_str());
            buffer
        }),
        d @ 6..=9 => Some({
            let mut string = digit_to_text(d).unwrap() + "ty";
            string.push_str(digit_to_text(ones).unwrap().as_str());
            string
        }),
        _ => None,
    }
}
fn place_value(number: u8, place: u8) -> Option<String> {
    match place {
        p @ 3..=6 => Some({
            let mut value = digit_to_text(number).unwrap();
            match p {
                3 => value.push_str("hundred"),
                4 => value.push_str("thousand"),
                // 5 => value.push_str("placeholder"),
                6 => value.push_str("million"),
                _ => (),
            }
            value
        }),
        _ => None,
    }
}
/// Convert u32 to words in a string.
pub fn to_text(number: u32) -> String {
    let mut numtext: String = String::new();
    let digits: Vec<u8> = number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();
    let second_digit_array: [usize; 2] = [0, 3];
    let mut last: Option<u8> = None;
    if digits.len() == 1 {
        numtext.push_str(digit_to_text(*digits.last().unwrap()).unwrap().as_str());
    } else {
        for (index, digit) in digits.iter().rev().enumerate() {
            if second_digit_array.contains(&index) {
                last = Some(digit.clone());
                continue;
            }
            if let Some(last_digit) = last {
                let mut buffer = String::new();
                buffer.push_str(tens_place(digit.clone(), last_digit).unwrap().as_str());
                if index > 2 {
                    buffer.push_str(place_value(0, index as u8).unwrap().as_str());
                }
                numtext.insert_str(0, buffer.as_str());
                last = None;
            } else {
                numtext.insert_str(
                    0,
                    place_value(digit.clone(), index as u8 + 1)
                        .unwrap()
                        .as_str(),
                )
            }
        }
    }
    numtext
}
