use crate::Formatting;

/// Convert digit to words in a string.
pub fn digit_to_text(digit: u8, capitalize: bool) -> Option<String> {
    if capitalize {
        match digit {
            0 => Some("".to_string()),
            1 => Some("One".to_string()),
            2 => Some("Two".to_string()),
            3 => Some("Three".to_string()),
            4 => Some("Four".to_string()),
            5 => Some("Five".to_string()),
            6 => Some("Six".to_string()),
            7 => Some("Seven".to_string()),
            8 => Some("Eight".to_string()),
            9 => Some("Nine".to_string()),
            _ => None,
        }
    } else {
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
}
fn place_value(number: u8, place: u8, fmt: &Formatting) -> Option<String> {
    let mut buffer = digit_to_text(number, fmt.capitalize).unwrap();
    if let Some(sep) = fmt.place_seperator {
        if number != 0 && place != 1 {
            buffer.push_str(sep)
        }
    }
    if fmt.capitalize {
        match place {
            1 => (), //Can happen but should return the same digit
            2 => (), //This should never happen as 2 is included in the tens_place_holders
            3 => buffer.push_str("Hundred"),
            4 => buffer.push_str("Thousand"),
            5 => (), // Souldn't happen
            6 => buffer.push_str("Million"),
            _ => (),
        };
    } else {
        match place {
            1 => (), //Can happen but should return the same digit
            2 => (), //This should never happen as 2 is included in the tens_place_holders
            3 => buffer.push_str("hundred"),
            4 => buffer.push_str("thousand"),
            5 => (), // Souldn't happen
            6 => buffer.push_str("million"),
            _ => (),
        }
    }
    Some(buffer)
}
fn tens_place(tens: u8, ones: u8, fmt: &Formatting) -> Option<String> {
    match tens {
        0 => digit_to_text(ones, fmt.capitalize),
        1 => {
            if fmt.capitalize {
                match ones {
                    0 => Some("Ten".to_string()),
                    1 => Some("Eleven".to_string()),
                    2 => Some("Twelve".to_string()),
                    3 => Some("Thirteen".to_string()),
                    4 => Some("Fourteen".to_string()),
                    5 => Some("Fifteen".to_string()),
                    6 => Some("Sixteen".to_string()),
                    7 => Some("Seventeen".to_string()),
                    8 => Some("Eighteen".to_string()),
                    9 => Some("Nineteen".to_string()),
                    _ => None,
                }
            } else {
                match ones {
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
                }
            }
        }
        d @ 2..=5 | d @ 8 => Some({
            let mut buffer = String::new();
            if fmt.capitalize {
                match d {
                    2 => buffer.push_str("Twenty"),
                    3 => buffer.push_str("Thirty"),
                    4 => buffer.push_str("Forty"),
                    5 => buffer.push_str("Fifty"),
                    8 => buffer.push_str("Eighty"),
                    _ => (),
                }
            } else {
                match d {
                    2 => buffer.push_str("twenty"),
                    3 => buffer.push_str("thirty"),
                    4 => buffer.push_str("forty"),
                    5 => buffer.push_str("fifty"),
                    8 => buffer.push_str("eighty"),
                    _ => (),
                }
            }
            if let Some(sep) = fmt.tens_seperator {
                buffer.push_str(sep);
            }
            buffer.push_str(digit_to_text(ones, fmt.capitalize).unwrap().as_str());
            buffer
        }),
        d @ 6..=9 => Some({
            let mut buffer = digit_to_text(d, fmt.capitalize).unwrap() + "ty";
            if let Some(sep) = fmt.tens_seperator {
                buffer.push_str(sep);
            }
            buffer.push_str(digit_to_text(ones, fmt.capitalize).unwrap().as_str());
            buffer
        }),
        _ => None,
    }
}

/// Convert usize to words in a string seperated by a seperator.
pub fn to_text_fmt(number: usize, fmt: &Formatting) -> String {
    let mut numtext: String = String::new();
    let mut last: Option<u8> = None;
    let tens_place_holders: [u8; 3] = [2, 5, 7];
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
        if *digit != 0 && place != digits.len() as u8 && numtext != "" {
            if let Some(sep) = fmt.digit_seperator {
                numtext.push_str(sep);
            }
        }
        if let Some(last_digit) = last {
            numtext.push_str(tens_place(last_digit, *digit, fmt).unwrap().as_str());
            if place > 2 {
                if let Some(sep) = fmt.digit_seperator {
                    numtext.push_str(sep);
                }
                numtext.push_str(place_value(0, place, fmt).unwrap().as_str());
            }
            last = None;
        } else if *digit != 0 {
            numtext.push_str(place_value(*digit, place, fmt).unwrap().as_str());
        }
        place -= 1;
    }
    numtext
}
