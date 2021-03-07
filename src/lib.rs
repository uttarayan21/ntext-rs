//!```rust
//!extern crate ntext;
//!use ntext::digit_to_text;
//!fn main() {
//!    println!("{}",digit_to_text(1).unwrap());
//!    assert_eq!("two",digit_to_text(2).unwrap());
//!    assert_ne!("five",digit_to_text(8).unwrap());
//!}
//!```
//! However giving the program a zero will return an empty string.
//!
//!```rust
//!extern crate ntext;
//!use ntext::to_text;
//!fn main() {
//!     println!("{}",to_text(1312));
//!     assert_eq!(to_text(1312),"onethousandthreehundredtwelve");
//!}
//!```
//! This fucntion will also return an empty string on input zero

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
    let mut value = digit_to_text(number).unwrap();
    match place {
        1 => (),
        2 => (), //This should never happen as 2 is included in the tens_place_holders
        3 => value.push_str("hundred"),
        4 => value.push_str("thousand"),
        5 => (), // Souldn't happen
        6 => value.push_str("million"),
        _ => (),
    };
    Some(value)
}

/// Convert u32 to words in a string.
pub fn to_text(number: u32) -> String {
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
            println!("hello index {}", place);
            numtext.push_str(tens_place(last_digit, *digit).unwrap().as_str());
            if place > 2 {
                numtext.push_str(place_value(0, place).unwrap().as_str());
            }
            last = None;
        } else if *digit != 0 {
            numtext.push_str(place_value(*digit, place).unwrap().as_str())
        }
        place -= 1;
    }
    numtext
}
