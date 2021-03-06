use std::error::Error;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/maps.rs"));

#[derive(Debug)]
pub struct TranslationError {
    pub why: String,
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl Error for TranslationError {}

pub fn encode_byte(value: u8) -> &'static str {
    &BYTE_TO_EMOJI[value as usize]
}

pub fn decode_byte(input: &dyn AsRef<str>) -> Result<u8, TranslationError> {
    let input_ref = input.as_ref();
    let result = EMOJI_TO_BYTE.get(input_ref).ok_or_else(|| TranslationError {
        why: format!("Cannot decode character {}", input_ref),
    })?;
    Ok(*result)
}

pub fn encode_string(input: &dyn AsRef<str>) -> String {
    input.as_ref().bytes().map(encode_byte).collect::<String>()
}

pub fn decode_string(input: &dyn AsRef<str>) -> Result<String, TranslationError> {
    let input = input.as_ref();
    let result = {
        // Older versions used a ZWSP as a character separator, instead of `ππ`.
        let split_char = input
            .chars()
            .find(|&c| c == '\u{200b}' || c == 'π');

        if let Some('\u{200b}') = split_char {
            input.trim_end_matches("\u{200B}").split("\u{200B}")
        } else {
            input.trim_end_matches("ππ").split("ππ")
        }
    }
    .map(|c| decode_byte(&c))
    .collect::<Result<Vec<u8>, _>>()?;

    Ok(String::from_utf8_lossy(&result).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_encode() {
        assert_eq!(
            encode_string(&"Test"),
            "πβ¨β¨β¨,,,,ππππ,ππππβ¨π₯Ίππππβ¨π₯Ί,ππ"
        );
    }

    #[test]
    fn test_byte_encode() {
        assert_eq!(encode_byte(b'h'), "ππ,,,,ππ",);
    }

    #[test]
    fn test_char_decode() {
        assert_eq!(decode_byte(&"ππ,,,,").unwrap(), b'h',);
    }

    #[test]
    fn test_string_decode() {
        // Test that we haven't killed backwards-compat
        assert_eq!(
            decode_string(&"πβ¨β¨β¨,,,,\u{200B}ππ,\u{200B}ππβ¨π₯Ί\u{200B}ππβ¨π₯Ί,\u{200B}")
                .unwrap(),
            "Test"
        );
        assert_eq!(
            decode_string(&"πβ¨β¨β¨,,,,ππππ,ππππβ¨π₯Ίππππβ¨π₯Ί,ππ").unwrap(),
            "Test"
        );
    }

    #[test]
    fn test_unicode_string_encode() {
        assert_eq!(
            encode_string(&"π₯Ί"),
            "π«β¨β¨β¨β¨ππππππ₯Ί,,,,πππππβ¨π₯Ίπππππβ¨β¨β¨π₯Ί,ππ"
        );
        assert_eq!(
            encode_string(&"γγγ°γ"),
            "π«β¨β¨π₯Ί,,ππππβ¨β¨π₯Ί,,,,ππππβ¨β¨β¨β¨πππ«β¨β¨π₯Ί,,ππ\
            ππβ¨β¨β¨ππππβ¨β¨β¨β¨π₯Ί,,πππ«β¨β¨π₯Ί,,ππππβ¨β¨π₯Ί,,,,ππ\
            πππβ¨β¨π₯Ί,πππ«β¨β¨π₯Ί,,ππππβ¨β¨β¨ππππβ¨β¨β¨β¨ππ"
        );
    }

    #[test]
    fn test_unicode_string_decode() {
        assert_eq!(
            decode_string(&"π«β¨β¨β¨β¨ππππππ₯Ί,,,,πππππβ¨π₯Ίπππππβ¨β¨β¨π₯Ί,ππ")
                .unwrap(),
            "π₯Ί",
        );
        assert_eq!(
            decode_string(
                &"π«β¨β¨π₯Ί,,ππππβ¨β¨π₯Ί,,,,ππππβ¨β¨β¨β¨πππ«β¨β¨π₯Ί,,ππ\
            ππβ¨β¨β¨ππππβ¨β¨β¨β¨π₯Ί,,πππ«β¨β¨π₯Ί,,ππππβ¨β¨π₯Ί,,,,ππ\
            πππβ¨β¨π₯Ί,πππ«β¨β¨π₯Ί,,ππππβ¨β¨β¨ππππβ¨β¨β¨β¨ππ"
            )
            .unwrap(),
            "γγγ°γ",
        );
    }

    #[test]
    fn test_embedded_null_byte() {
        assert_eq!(
            encode_string(&"\0"),
            "β€οΈππ",
        );
        assert_eq!(
            decode_string(&"β€οΈππ")
                .unwrap(),
            "\0",
        );
    }
}
