//! Example program with default formatting  
//!```rust
//!extern crate ntext;
//!use ntext::to_text;
//!fn main() {
//!     assert_eq!(to_text!(7123),"Seven Thousand,One Hundred,Twenty-Three");
//!     assert_eq!(to_text!(1000),"One Thousand");
//!}
//!```
//! Example program with custom formatting
//!
//!```rust
//!extern crate ntext;
//!use ntext::{Formatting,to_text};
//!fn main() {
//!     assert_eq!(to_text!(1000, &Formatting::none().capitalize()),"OneThousand");
//!     assert_eq!(to_text!(34123, &Formatting::with_seperator("#").capitalize()),"Thirty#Four#Thousand#One#Hundred#Twenty#Three");
//!}
//!```
//! This macro will also return an empty string on input zero  
//! You can also create the Formatting struct manually  
//! Source [ntext-rs](https://github.com/uttarayan21/ntext-rs)   

mod formatting;
mod numtext;
mod test;

pub use formatting::Formatting;
pub use numtext::{digit_to_text, to_text_fmt};

/// Macro which supports both seperator and without it
#[macro_export]
macro_rules! to_text {
    ($number:expr) => {
        ntext::to_text_fmt($number, &ntext::Formatting::default());
    };
    ($number:expr, $formatting:expr) => {
        ntext::to_text_fmt($number, $formatting);
    };
}
