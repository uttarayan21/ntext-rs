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
//!     println!("{}",to_text!(1312));
//!     assert_eq!(to_text!(1312),"onethousandthreehundredtwelve");
//!     println!("{}",to_text!(7123));
//!     assert_eq!(to_text!(7123," "),"seven thousand one hundred twenty three");
//!}
//!```
//! This macro will also return an empty string on input zero

mod no_seperator;
mod seperator;
mod test;
pub use no_seperator::to_text_no_seperator;
pub use seperator::to_text_with_seperator;

/// Convert digit to words in a string.
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
fn place_value(number: u8, place: u8, seperator: Option<&str>) -> Option<String> {
    let mut buffer = digit_to_text(number).unwrap();
    if let Some(sep) = seperator {
        if number != 0 {
            buffer.push_str(sep)
        }
    }
    match place {
        1 => (),
        2 => (), //This should never happen as 2 is included in the tens_place_holders
        3 => buffer.push_str("hundred"),
        4 => buffer.push_str("thousand"),
        5 => (), // Souldn't happen
        6 => buffer.push_str("million"),
        _ => (),
    };
    Some(buffer)
}

/// Macro which supports both seperator and without it
#[macro_export]
macro_rules! to_text {
    ($number:expr) => {
        ntext::to_text_no_seperator($number);
    };
    ($number:expr, $seperator:expr) => {
        ntext::to_text_with_seperator($number, $seperator);
    };
}
