#[cfg(test)]
mod tests {
    #[test]
    fn digits() {
        use crate::digit_to_text;
        assert_eq!(digit_to_text(9).unwrap(), "nine");
        assert_eq!(digit_to_text(3).unwrap(), "three");
        assert_eq!(digit_to_text(7).unwrap(), "seven");
        assert_eq!(digit_to_text(5).unwrap(), "five");
    }
    #[test]
    fn numbers() {
        use crate::to_text_no_seperator as to_text;
        assert_eq!(to_text(1), "one");
        assert_eq!(to_text(10), "ten");
        assert_eq!(to_text(100), "onehundred");
        assert_eq!(to_text(1000), "onethousand");
        assert_eq!(to_text(12345), "twelvethousandthreehundredfortyfive");
        assert_eq!(to_text(81123), "eightyonethousandonehundredtwentythree");
        assert_eq!(to_text(12), "twelve");
    }
    #[test]
    fn numbers_seperator() {
        use crate::to_text_with_seperator as to_text;
        assert_eq!(to_text(103, "/"), "one/hundred/three");
        assert_eq!(to_text(1000, "/"), "one/thousand");
        assert_eq!(
            to_text(12345, "/"),
            "twelve/thousand/three/hundred/forty/five"
        );
        assert_eq!(
            to_text(651243, "/"),
            "six/million/fifty/one/thousand/two/hundred/forty/three"
        );
    }
}
