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
        use crate::to_text;
        assert_eq!(to_text(12345), "twelvethousandthreehundredfortyfive");
        assert_eq!(to_text(81123), "eightyonethousandonehundredtwentythree");
        assert_eq!(to_text(12), "twelve");
        assert_eq!(to_text(2), "two");
    }
    #[test]
    fn tens() {
        use crate::tens_place;
        assert_eq!(tens_place(1, 2).unwrap(), "twelve");
        assert_eq!(tens_place(7, 6).unwrap(), "seventysix");
        assert_eq!(tens_place(5, 7).unwrap(), "fiftyseven");
        assert_eq!(tens_place(2, 3).unwrap(), "twentythree");
        assert_eq!(tens_place(6, 9).unwrap(), "sixtynine");
        assert_eq!(tens_place(0, 9).unwrap(), "nine");
    }
}
