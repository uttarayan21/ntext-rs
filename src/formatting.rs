/// Set the formatting of the output  
#[derive(Clone, Copy)]
pub struct Formatting<'format> {
    /// capitalize the start of the word.
    pub capitalize: bool,
    /// Set the seperator in between digits like "threehundred,twentytwo".
    pub digit_seperator: Option<&'format str>,
    /// Set the seperator in between words like "three/hundred".
    pub place_seperator: Option<&'format str>,
    /// Set the seperator between tens place digits like "twenty-two".
    pub tens_seperator: Option<&'format str>,
}

impl<'format> Formatting<'format> {
    /// Get the default formatting.
    pub fn default() -> Self {
        Self {
            capitalize: true,
            digit_seperator: Some(","),
            place_seperator: Some(" "),
            tens_seperator: Some("-"),
        }
    }
    /// No formatting at all
    pub fn none() -> Self {
        Self {
            capitalize: false,
            digit_seperator: None,
            place_seperator: None,
            tens_seperator: None,
        }
    }
    /// With same formatting for all
    pub fn with_seperator(seperator: &'format str) -> Self {
        Self {
            capitalize: false,
            digit_seperator: Some(seperator),
            place_seperator: Some(seperator),
            tens_seperator: Some(seperator),
        }
    }
    /// Capitalize the formatting
    pub fn capitalize(&mut self) -> Self {
        self.capitalize = true;
        Self {
            capitalize: self.capitalize,
            digit_seperator: self.digit_seperator,
            place_seperator: self.place_seperator,
            tens_seperator: self.tens_seperator,
        }
    }
    /// Lower case the formatting struct
    pub fn decapitalize(&mut self) -> Self {
        self.capitalize = false;
        Self {
            capitalize: self.capitalize,
            digit_seperator: self.digit_seperator,
            place_seperator: self.place_seperator,
            tens_seperator: self.tens_seperator,
        }
    }
}
