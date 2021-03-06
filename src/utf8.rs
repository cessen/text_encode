//! UTF-8.
//!
//! These functions are essentially redundant, since they're supposedly
//! encoding/decoding between utf8 and... utf8.  However, `decode_to_str()`
//! is still useful for validating unknown input.  And they provide a uniform
//! API for all encodings.

use core;
use {DecodeError, DecodeErrorCause, DecodeResult, EncodeResult};

pub fn encode_from_str<'a>(input: &str, out_buffer: &'a mut [u8]) -> EncodeResult<'a> {
    let cl = copy_len(input.as_bytes(), out_buffer.len());
    out_buffer[..cl].copy_from_slice(input[..cl].as_bytes());
    Ok((&out_buffer[..cl], cl))
}

pub fn decode_to_str<'a>(input: &[u8], out_buffer: &'a mut [u8], is_end: bool) -> DecodeResult<'a> {
    // Find how much of the data is valid utf8.
    let valid_up_to = match core::str::from_utf8(input) {
        Ok(text) => text.len(),
        Err(e) => e.valid_up_to(),
    };

    // Copy over what we can.
    let bytes_copied = copy_len(&input[..valid_up_to], out_buffer.len());
    out_buffer[..bytes_copied].copy_from_slice(&input[..bytes_copied]);

    // Determine if there's an error.
    if bytes_copied < out_buffer.len() && bytes_copied == valid_up_to && valid_up_to < input.len() {
        let trailing_bytes = input.len() - valid_up_to;
        let byte = input[valid_up_to];
        // First we check if we're truncated.  If not, then it's an error.
        let is_truncated = ((byte & 0b1110_0000) == 0b1100_0000 && trailing_bytes < 2)
            || ((byte & 0b1111_0000) == 0b1110_0000 && trailing_bytes < 3)
            || ((byte & 0b1111_1000) == 0b1111_0000 && trailing_bytes < 4);
        if !is_truncated {
            // Find the byte range of the error by finding the next valid
            // starting byte (or reaching end of input).
            let mut i = valid_up_to + 1;
            while i < input.len()
                && ((input[i] & 0b1100_0000) == 0b1000_0000
                    || (input[i] & 0b1111_1000) == 0b1111_1000)
            {
                i += 1;
            }
            // Return the error.
            return Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (valid_up_to, i),
                output_bytes_written: bytes_copied,
            });
        } else if is_end {
            // If we're truncated _and_ at end-of-input, that's also an error.
            return Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (valid_up_to, input.len()),
                output_bytes_written: bytes_copied,
            });
        }
    }

    // No error, return success.
    Ok((
        unsafe { core::str::from_utf8_unchecked(&out_buffer[..bytes_copied]) },
        bytes_copied,
    ))
}

/// Calculates how many bytes should be copied from input to output given
/// their lengths and the content of input.  Specifically, it calculates
/// the maximum amount that can be copied without incompletely copying
/// any multi-byte codepoints.
///
/// Input is assumed to be valid and complete utf8 (i.e. could be turned
/// directly into a &str).
#[inline(always)]
fn copy_len(input: &[u8], output_len: usize) -> usize {
    if output_len >= input.len() {
        input.len()
    } else {
        let mut i = output_len;
        while i > 0 && (input[i] & 0b1100_0000) == 0b1000_0000 {
            i -= 1;
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_01() {
        let text = "こんにちは！";
        let mut buf = [0u8; 2];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(encoded, &[]);
    }

    #[test]
    fn encode_02() {
        let text = "こんにちは！";
        let mut buf = [0u8; 3];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 3);
        assert_eq!(encoded, &[0xE3, 0x81, 0x93]);
    }

    #[test]
    fn encode_03() {
        let text = "こんにちは！";
        let mut buf = [0u8; 5];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 3);
        assert_eq!(encoded, &[0xE3, 0x81, 0x93]);
    }

    #[test]
    fn decode_01() {
        let data = [
            0xE3, 0x81, 0x93, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！"
        let mut buf = [0u8; 2];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(decoded, "");
    }

    #[test]
    fn decode_02() {
        let data = [
            0xE3, 0x81, 0x93, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！"
        let mut buf = [0u8; 3];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 3);
        assert_eq!(decoded, "こ");
    }

    #[test]
    fn decode_03() {
        let data = [
            0xE3, 0x81, 0x93, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！"
        let mut buf = [0u8; 5];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 3);
        assert_eq!(decoded, "こ");
    }

    #[test]
    fn decode_04() {
        let data = [
            0xE3, 0x81, 0x93, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF, 0xBC,
        ]; // "こんにちは！" with last byte chopped off.
        let mut buf = [0u8; 64];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, false).unwrap();
        assert_eq!(consumed_count, 15);
        assert_eq!(decoded, "こんにちは");
    }

    #[test]
    fn decode_05() {
        let data = [
            0xE3, 0x81, 0x93, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF,
        ]; // "こんにちは！" with last 2 bytes chopped off.
        let mut buf = [0u8; 64];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, false).unwrap();
        assert_eq!(consumed_count, 15);
        assert_eq!(decoded, "こんにちは");
    }

    #[test]
    fn decode_error_01() {
        let data = [
            0b10000000, 0x81, 0x93, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3,
            0x81, 0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the first char (continuing code unit).
        let mut buf = [0u8; 2];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (0, 3),
                output_bytes_written: 0,
            })
        );
    }

    #[test]
    fn decode_error_02() {
        let data = [
            0xE3, 0x81, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81, 0xAF,
            0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the first code point (too few continuing code units).
        let mut buf = [0u8; 2];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (0, 2),
                output_bytes_written: 0,
            })
        );
    }

    #[test]
    fn decode_error_03() {
        let data = [
            0xE3, 0x81, 0x93, 0b10000000, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3,
            0x81, 0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the second code point (continuing code unit).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (3, 6),
                output_bytes_written: 3,
            })
        );
    }

    #[test]
    fn decode_error_04() {
        let data = [
            0xE3, 0x81, 0x93, 0b10000000, 0x82, 0x93, 0b10000000, 0x81, 0xAB, 0b10000000, 0x81,
            0xA1, 0xE3, 0x81, 0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the second code point (lots of continuing code units).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (3, 12),
                output_bytes_written: 3,
            })
        );
    }

    #[test]
    fn decode_error_05() {
        let data = [
            0xE3, 0x81, 0x93, 0b11111000, 0x82, 0x93, 0x93, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81,
            0xA1, 0xE3, 0x81, 0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the second code point (invalid bit pattern).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (3, 8),
                output_bytes_written: 3,
            })
        );
    }

    #[test]
    fn decode_error_06() {
        let data = [
            0xE3, 0x81, 0x93, 0xED, 0xA0, 0x80, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the second code point (beginning of surrogate range).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (3, 6),
                output_bytes_written: 3,
            })
        );
    }

    #[test]
    fn decode_error_07() {
        let data = [
            0xE3, 0x81, 0x93, 0xED, 0xBF, 0xBF, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the second code point (end of surrogate range).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (3, 6),
                output_bytes_written: 3,
            })
        );
    }

    #[test]
    fn decode_error_08() {
        let data = [
            0xE3, 0x81, 0x93, 0xF4, 0x90, 0x80, 0x80, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3,
            0x81, 0xAF, 0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the second code point (out of unicode range).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (3, 7),
                output_bytes_written: 3,
            })
        );
    }

    #[test]
    fn decode_error_09() {
        let data = [
            0xE3, 0x81, 0x93, 0xC0, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81, 0xAF,
            0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the second code point (byte == 0xC0).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (3, 5),
                output_bytes_written: 3,
            })
        );
    }

    #[test]
    fn decode_error_10() {
        let data = [
            0xE3, 0x81, 0x93, 0xC1, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81, 0xAF,
            0xEF, 0xBC, 0x81,
        ]; // "こんにちは！" with an error on the second code point (byte == 0xC1).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (3, 5),
                output_bytes_written: 3,
            })
        );
    }

    #[test]
    fn decode_error_11() {
        let data = [
            0xE3, 0x81, 0x93, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF, 0xBC,
        ]; // "こんにちは！" with last byte chopped off.
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (15, 17),
                output_bytes_written: 15,
            })
        );
    }

    #[test]
    fn decode_error_12() {
        let data = [
            0xE3, 0x81, 0x93, 0xE3, 0x82, 0x93, 0xE3, 0x81, 0xAB, 0xE3, 0x81, 0xA1, 0xE3, 0x81,
            0xAF, 0xEF,
        ]; // "こんにちは！" with last 2 bytes chopped off.
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (15, 16),
                output_bytes_written: 15,
            })
        );
    }
}
