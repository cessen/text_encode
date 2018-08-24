//! Encoding/decoding functions for the WHATWG variant of BIG5.

use core;
use {DecodeError, DecodeResult, EncodeError, EncodeResult};

// Generated by build.rs  Contains ENCODE_TABLE and DECODE_TABLE.
include!(concat!(env!("OUT_DIR"), "/big5_whatwg_tables.rs"));

pub fn encode_from_str<'a>(input: &str, output: &'a mut [u8]) -> EncodeResult<'a> {
    // Do the encode.
    let mut input_i = 0;
    let mut output_i = 0;
    for (offset, c) in input.char_indices() {
        let mut code = c as u32;
        if output_i >= output.len() {
            break;
        } else if code <= 127 {
            // Ascii
            output[output_i] = code as u8;
            output_i += 1;
            input_i = offset + 1;
        } else if let Ok(ptr_i) = ENCODE_TABLE.binary_search_by_key(&c, |x| x.0) {
            if (output_i + 1) < output.len() {
                let big5_bytes = ENCODE_TABLE[ptr_i].1;
                output[output_i] = big5_bytes[0];
                output[output_i + 1] = big5_bytes[1];
                output_i += 2;
                input_i = offset + 1;
            } else {
                break;
            }
        } else {
            return Err(EncodeError {
                character: c,
                error_range: (offset, offset + c.len_utf8()),
                output_bytes_written: output_i,
            });
        }
    }

    // Calculate how much of the input was consumed.
    if input_i > input.len() {
        input_i = input.len();
    } else {
        while !input.is_char_boundary(input_i) {
            input_i += 1;
        }
    }

    Ok((input_i, &output[..output_i]))
}

pub fn decode_to_str<'a>(input: &[u8], output: &'a mut [u8]) -> DecodeResult<'a> {
    let mut input_i = 0;
    let mut output_i = 0;
    let mut buf = [0u8; 4]; // For encoding utf8 codepoints.

    // Loop through the input, getting 2 bytes at a time.
    let mut itr = input.iter();
    while let Some(&byte_1) = itr.next() {
        if byte_1 <= 127 {
            // Ascii
            if output_i < output.len() {
                output[output_i] = byte_1;
                output_i += 1;
                input_i += 1;
            } else {
                break;
            }
        } else {
            // Decode to scalar value.
            let string = if let Some(&byte_2) = itr.next() {
                if byte_1 < 0x81 || byte_1 > 0xFE {
                    // Error: invalid leading byte.
                    return Err(DecodeError {
                        error_range: (input_i, input_i + 2),
                        output_bytes_written: output_i,
                    });
                }
                if byte_2 < 0x40 || byte_2 > 0xFE || (byte_2 > 0x7E && byte_2 < 0xA1) {
                    // Error: invalid trailing byte.
                    return Err(DecodeError {
                        error_range: (input_i, input_i + if byte_2 <= 127 { 1 } else { 2 }),
                        output_bytes_written: output_i,
                    });
                }
                let big5_ptr = {
                    let lead = (byte_1 as usize - 0x81) * 157;
                    let trail = if byte_2 < 0x7f {
                        byte_2 as usize - 0x40
                    } else {
                        byte_2 as usize - 0x62
                    };
                    lead + trail
                };

                // Get our string, either from the table or by special handling.
                if big5_ptr >= DECODE_TABLE.len() || DECODE_TABLE[big5_ptr] == '�' {
                    match big5_ptr {
                        // Special handling for codes that map to graphemes.
                        1133 => "\u{00CA}\u{0304}",
                        1135 => "\u{00CA}\u{030C}",
                        1164 => "\u{00EA}\u{0304}",
                        1166 => "\u{00EA}\u{030C}",
                        _ => {
                            // Error: undefined code.
                            return Err(DecodeError {
                                error_range: (input_i, input_i + 2),
                                output_bytes_written: output_i,
                            });
                        }
                    }
                } else {
                    // Encode codepoint to utf8.
                    DECODE_TABLE[big5_ptr].encode_utf8(&mut buf)
                }
            } else {
                break;
            };

            // Copy decoded data to output.
            if (output_i + string.len()) > output.len() {
                break;
            }
            output[output_i..(output_i + string.len())].copy_from_slice(string.as_bytes());

            // Update our counters.
            input_i += 2;
            output_i += string.len();
        }
    }

    Ok((input_i, unsafe {
        core::str::from_utf8_unchecked(&output[..output_i])
    }))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn encode_01() {
//         let text = "こんにちは！";
//         let mut buf = [0u8; 1];
//         let (consumed_count, encoded) = encode_from_str(text, &mut buf).unwrap();
//         assert_eq!(consumed_count, 0);
//         assert_eq!(encoded, &[]);
//     }

//     #[test]
//     fn encode_02() {
//         let text = "こんにちは！";
//         let mut buf = [0u8; 2];
//         let (consumed_count, encoded) = encode_from_str(text, &mut buf).unwrap();
//         assert_eq!(consumed_count, 3);
//         assert_eq!(encoded, &[0x30, 0x53]);
//     }

//     #[test]
//     fn encode_03() {
//         let text = "こんにちは！";
//         let mut buf = [0u8; 3];
//         let (consumed_count, encoded) = encode_from_str(text, &mut buf).unwrap();
//         assert_eq!(consumed_count, 3);
//         assert_eq!(encoded, &[0x30, 0x53]);
//     }

//     #[test]
//     fn encode_04() {
//         let text = "😺😼";
//         let mut buf = [0u8; 3];
//         let (consumed_count, encoded) = encode_from_str(text, &mut buf).unwrap();
//         assert_eq!(consumed_count, 0);
//         assert_eq!(encoded, &[]);
//     }

//     #[test]
//     fn encode_05() {
//         let text = "😺😼";
//         let mut buf = [0u8; 4];
//         let (consumed_count, encoded) = encode_from_str(text, &mut buf).unwrap();
//         assert_eq!(consumed_count, 4);
//         assert_eq!(encoded, &[0xD8, 0x3D, 0xDE, 0x3A]);
//     }

//     #[test]
//     fn encode_06() {
//         let text = "😺😼";
//         let mut buf = [0u8; 7];
//         let (consumed_count, encoded) = encode_from_str(text, &mut buf).unwrap();
//         assert_eq!(consumed_count, 4);
//         assert_eq!(encoded, &[0xD8, 0x3D, 0xDE, 0x3A]);
//     }

//     #[test]
//     fn decode_01() {
//         let data = [
//             0x30, 0x53, 0x30, 0x93, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！"
//         let mut buf = [0u8; 2];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 0);
//         assert_eq!(decoded, "");
//     }

//     #[test]
//     fn decode_02() {
//         let data = [
//             0x30, 0x53, 0x30, 0x93, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！"
//         let mut buf = [0u8; 3];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 2);
//         assert_eq!(decoded, "こ");
//     }

//     #[test]
//     fn decode_03() {
//         let data = [
//             0x30, 0x53, 0x30, 0x93, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！"
//         let mut buf = [0u8; 5];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 2);
//         assert_eq!(decoded, "こ");
//     }

//     #[test]
//     fn decode_04() {
//         let data = [0xD8, 0x3D, 0xDE, 0x3A, 0xD8, 0x3D, 0xDE, 0x3C]; // "😺😼"
//         let mut buf = [0u8; 3];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 0);
//         assert_eq!(decoded, "");
//     }

//     #[test]
//     fn decode_05() {
//         let data = [0xD8, 0x3D, 0xDE, 0x3A, 0xD8, 0x3D, 0xDE, 0x3C]; // "😺😼"
//         let mut buf = [0u8; 4];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 4);
//         assert_eq!(decoded, "😺");
//     }

//     #[test]
//     fn decode_06() {
//         let data = [0xD8, 0x3D, 0xDE, 0x3A, 0xD8, 0x3D, 0xDE, 0x3C]; // "😺😼"
//         let mut buf = [0u8; 7];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 4);
//         assert_eq!(decoded, "😺");
//     }

//     #[test]
//     fn decode_07() {
//         let data = [0xD8, 0x3D, 0xDE, 0x3A, 0xD8, 0x3D]; // "😺😼" with last codepoint chopped off.
//         let mut buf = [0u8; 64];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 4);
//         assert_eq!(decoded, "😺");
//     }

//     #[test]
//     fn decode_08() {
//         let data = [0xD8, 0x3D, 0xDE, 0x3A, 0xD8, 0x3D, 0xDE]; // "😺😼" with last byte chopped off.
//         let mut buf = [0u8; 64];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 4);
//         assert_eq!(decoded, "😺");
//     }

//     #[test]
//     fn decode_09() {
//         let data = [0xD8, 0x3D, 0xDE, 0x3A, 0xD8]; // "😺😼" with last 3 bytes chopped off.
//         let mut buf = [0u8; 64];
//         let (consumed_count, decoded) = decode_to_str(&data, &mut buf).unwrap();
//         assert_eq!(consumed_count, 4);
//         assert_eq!(decoded, "😺");
//     }

//     #[test]
//     fn decode_error_01() {
//         let data = [
//             0xDE, 0x3A, 0x30, 0x93, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！" with an error on the first char (end surrogate)
//         let mut buf = [0u8; 2];
//         let error = decode_to_str(&data, &mut buf);
//         assert_eq!(
//             error,
//             Err(DecodeError {
//                 error_range: (0, 2),
//                 output_bytes_written: 0,
//             })
//         );
//     }

//     #[test]
//     fn decode_error_02() {
//         let data = [
//             0x30, 0x53, 0xDE, 0x3A, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！" with an error on the second char (end surrogate)
//         let mut buf = [0u8; 3];
//         let error = decode_to_str(&data, &mut buf);
//         assert_eq!(
//             error,
//             Err(DecodeError {
//                 error_range: (2, 4),
//                 output_bytes_written: 3,
//             })
//         );
//     }

//     #[test]
//     fn decode_error_03() {
//         let data = [
//             0x30, 0x53, 0x30, 0x93, 0x30, 0x6B, 0xDE, 0x3A, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！" with an error on the fourth char (end surrogate)
//         let mut buf = [0u8; 64];
//         let error = decode_to_str(&data, &mut buf);
//         assert_eq!(
//             error,
//             Err(DecodeError {
//                 error_range: (6, 8),
//                 output_bytes_written: 9,
//             })
//         );
//     }

//     #[test]
//     fn decode_error_04() {
//         let data = [
//             0xD8, 0x3D, 0x30, 0x93, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！" with an error on the first char (start surrogate)
//         let mut buf = [0u8; 2];
//         let error = decode_to_str(&data, &mut buf);
//         assert_eq!(
//             error,
//             Err(DecodeError {
//                 error_range: (0, 2),
//                 output_bytes_written: 0,
//             })
//         );
//     }

//     #[test]
//     fn decode_error_05() {
//         let data = [
//             0x30, 0x53, 0xD8, 0x3D, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！" with an error on the second char (start surrogate)
//         let mut buf = [0u8; 3];
//         let error = decode_to_str(&data, &mut buf);
//         assert_eq!(
//             error,
//             Err(DecodeError {
//                 error_range: (2, 4),
//                 output_bytes_written: 3,
//             })
//         );
//     }

//     #[test]
//     fn decode_error_06() {
//         let data = [
//             0x30, 0x53, 0x30, 0x93, 0x30, 0x6B, 0xD8, 0x3D, 0x30, 0x6F, 0xFF, 0x01,
//         ]; // "こんにちは！" with an error on the fourth char (start surrogate)
//         let mut buf = [0u8; 64];
//         let error = decode_to_str(&data, &mut buf);
//         assert_eq!(
//             error,
//             Err(DecodeError {
//                 error_range: (6, 8),
//                 output_bytes_written: 9,
//             })
//         );
//     }
// }
