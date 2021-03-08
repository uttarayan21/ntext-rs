use crate::{digit_to_text, place_value};
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

/// Convert u32 to words in a string.
pub fn to_text_no_seperator(number: u32) -> String {
    let mut numtext: String = String::new();
    let mut last: Option<u8> = None;
    let tens_place_holders: [u8; 2] = [2, 5];
    let digits: Vec<u8> = number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

    let mut place: u8 = digits.len() as u8;

    for (_index, digit) in digits.iter().enumerate() {
        if tens_place_holders.contains(&place) {
            last = Some(*digit);
            place -= 1;
            continue;
        }
        if let Some(last_digit) = last {
            numtext.push_str(tens_place(last_digit, *digit).unwrap().as_str());
            if place > 2 {
                numtext.push_str(place_value(0, place, None).unwrap().as_str());
            }
            last = None;
        } else if *digit != 0 {
            numtext.push_str(place_value(*digit, place, None).unwrap().as_str())
        }
        place -= 1;
    }
    numtext
}
