#[cfg(test)]
mod tests {
    #[test]
    fn digits() {
        use crate::digit_to_text;
        assert_eq!(digit_to_text(9, false).unwrap(), "nine");
        assert_eq!(digit_to_text(3, false).unwrap(), "three");
        assert_eq!(digit_to_text(7, false).unwrap(), "seven");
        assert_eq!(digit_to_text(5, false).unwrap(), "five");
        assert_eq!(digit_to_text(5, true).unwrap(), "Five");
    }
    #[test]
    fn numbers() {
        use crate::to_text_fmt as to_text;
        use crate::Formatting;
        let fmt = &Formatting::none();
        assert_eq!(to_text(1, fmt), "one");
        assert_eq!(to_text(10, fmt), "ten");
        assert_eq!(to_text(100, fmt), "onehundred");
        assert_eq!(to_text(1000, fmt), "onethousand");
        assert_eq!(to_text(12345, fmt), "twelvethousandthreehundredfortyfive");
        assert_eq!(
            to_text(81123, fmt),
            "eightyonethousandonehundredtwentythree"
        );
        assert_eq!(to_text(12, fmt), "twelve");
    }
    #[test]
    fn numbers_formatting() {
        use crate::to_text_fmt as to_text;
        use crate::Formatting;
        let fmt = &Formatting::default();
        assert_eq!(to_text(1, fmt), "One");
        assert_eq!(to_text(10, fmt), "Ten");
        assert_eq!(to_text(100, fmt), "One Hundred");
        assert_eq!(to_text(1000, fmt), "One Thousand");
        assert_eq!(to_text(12315, fmt), "Twelve Thousand,Three Hundred,Fifteen");
        assert_eq!(
            to_text(1235245, fmt),
            "Twelve Million,Thirty-Five Thousand,Two Hundred,Forty-Five"
        );
    }
}
