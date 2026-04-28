use chardetng::EncodingDetector;

pub fn decode_to_utf8(bytes: &[u8]) -> (String, &'static str) {
    let mut detector = EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);
    let (decoded, _, _) = encoding.decode(bytes);
    (decoded.into_owned(), encoding.name())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_sjis() {
        let sjis_bytes = vec![0x82, 0xB1, 0x82, 0xF1, 0x82, 0xC9, 0x82, 0xBF, 0x82, 0xCD];
        let (decoded, encoding) = decode_to_utf8(&sjis_bytes);
        assert_eq!(decoded, "こんにちは");
        assert!(encoding.to_lowercase().contains("shift"));
    }
}
