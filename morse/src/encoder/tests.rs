#[cfg(test)]
mod tests {
    use crate::encoder;

    #[test]
    fn test_encode_string() {
        assert_eq!(encoder::encode("bruno").unwrap(), "_... ._. .._ _. ___");
        assert_eq!(encoder::encode("sos").unwrap(), "... ___ ...");
        assert_eq!(encoder::encode("a").unwrap(), "._");
        assert_eq!(encoder::encode("b").unwrap(), "_...");
        assert_eq!(encoder::encode("c").unwrap(), "_._.");
        assert_eq!(encoder::encode("d").unwrap(), "_..");
    }
}
