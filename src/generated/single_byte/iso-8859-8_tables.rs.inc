// This file is auto-generated.  Please see `encoding_tables/single_byte/`
// from the root directory for the files that generate this.

use {DecodeResult, EncodeResult};

pub fn decode_to_str<'a>(
    input: &[u8],
    out_buffer: &'a mut [u8],
    is_end: bool
) -> DecodeResult<'a> {
    super::ascii_ext_decode_to_str(&DECODE_TABLE, input, out_buffer, is_end)
}

pub fn encode_from_str<'a>(
    input: &str,
    out_buffer: &'a mut [u8],
    is_end: bool
) -> EncodeResult<'a> {
    super::ascii_ext_encode_from_str(&ENCODE_TABLE, input, out_buffer, is_end)
}

const DECODE_TABLE: [char; 128] = [
    '\u{0080}','\u{0081}','\u{0082}','\u{0083}','\u{0084}','\u{0085}','\u{0086}','\u{0087}',
    '\u{0088}','\u{0089}','\u{008A}','\u{008B}','\u{008C}','\u{008D}','\u{008E}','\u{008F}',
    '\u{0090}','\u{0091}','\u{0092}','\u{0093}','\u{0094}','\u{0095}','\u{0096}','\u{0097}',
    '\u{0098}','\u{0099}','\u{009A}','\u{009B}','\u{009C}','\u{009D}','\u{009E}','\u{009F}',
    '\u{00A0}','�','\u{00A2}','\u{00A3}','\u{00A4}','\u{00A5}','\u{00A6}','\u{00A7}',
    '\u{00A8}','\u{00A9}','\u{00D7}','\u{00AB}','\u{00AC}','\u{00AD}','\u{00AE}','\u{00AF}',
    '\u{00B0}','\u{00B1}','\u{00B2}','\u{00B3}','\u{00B4}','\u{00B5}','\u{00B6}','\u{00B7}',
    '\u{00B8}','\u{00B9}','\u{00F7}','\u{00BB}','\u{00BC}','\u{00BD}','\u{00BE}','�',
    '�','�','�','�','�','�','�','�',
    '�','�','�','�','�','�','�','�',
    '�','�','�','�','�','�','�','�',
    '�','�','�','�','�','�','�','\u{2017}',
    '\u{05D0}','\u{05D1}','\u{05D2}','\u{05D3}','\u{05D4}','\u{05D5}','\u{05D6}','\u{05D7}',
    '\u{05D8}','\u{05D9}','\u{05DA}','\u{05DB}','\u{05DC}','\u{05DD}','\u{05DE}','\u{05DF}',
    '\u{05E0}','\u{05E1}','\u{05E2}','\u{05E3}','\u{05E4}','\u{05E5}','\u{05E6}','\u{05E7}',
    '\u{05E8}','\u{05E9}','\u{05EA}','�','�','\u{200E}','\u{200F}','�',
];

const ENCODE_TABLE: [(char, u8); 92] = [
    ('\u{0080}', 0x80), ('\u{0081}', 0x81), ('\u{0082}', 0x82), ('\u{0083}', 0x83), 
    ('\u{0084}', 0x84), ('\u{0085}', 0x85), ('\u{0086}', 0x86), ('\u{0087}', 0x87), 
    ('\u{0088}', 0x88), ('\u{0089}', 0x89), ('\u{008A}', 0x8A), ('\u{008B}', 0x8B), 
    ('\u{008C}', 0x8C), ('\u{008D}', 0x8D), ('\u{008E}', 0x8E), ('\u{008F}', 0x8F), 
    ('\u{0090}', 0x90), ('\u{0091}', 0x91), ('\u{0092}', 0x92), ('\u{0093}', 0x93), 
    ('\u{0094}', 0x94), ('\u{0095}', 0x95), ('\u{0096}', 0x96), ('\u{0097}', 0x97), 
    ('\u{0098}', 0x98), ('\u{0099}', 0x99), ('\u{009A}', 0x9A), ('\u{009B}', 0x9B), 
    ('\u{009C}', 0x9C), ('\u{009D}', 0x9D), ('\u{009E}', 0x9E), ('\u{009F}', 0x9F), 
    ('\u{00A0}', 0xA0), ('\u{00A2}', 0xA2), ('\u{00A3}', 0xA3), ('\u{00A4}', 0xA4), 
    ('\u{00A5}', 0xA5), ('\u{00A6}', 0xA6), ('\u{00A7}', 0xA7), ('\u{00A8}', 0xA8), 
    ('\u{00A9}', 0xA9), ('\u{00AB}', 0xAB), ('\u{00AC}', 0xAC), ('\u{00AD}', 0xAD), 
    ('\u{00AE}', 0xAE), ('\u{00AF}', 0xAF), ('\u{00B0}', 0xB0), ('\u{00B1}', 0xB1), 
    ('\u{00B2}', 0xB2), ('\u{00B3}', 0xB3), ('\u{00B4}', 0xB4), ('\u{00B5}', 0xB5), 
    ('\u{00B6}', 0xB6), ('\u{00B7}', 0xB7), ('\u{00B8}', 0xB8), ('\u{00B9}', 0xB9), 
    ('\u{00BB}', 0xBB), ('\u{00BC}', 0xBC), ('\u{00BD}', 0xBD), ('\u{00BE}', 0xBE), 
    ('\u{00D7}', 0xAA), ('\u{00F7}', 0xBA), ('\u{05D0}', 0xE0), ('\u{05D1}', 0xE1), 
    ('\u{05D2}', 0xE2), ('\u{05D3}', 0xE3), ('\u{05D4}', 0xE4), ('\u{05D5}', 0xE5), 
    ('\u{05D6}', 0xE6), ('\u{05D7}', 0xE7), ('\u{05D8}', 0xE8), ('\u{05D9}', 0xE9), 
    ('\u{05DA}', 0xEA), ('\u{05DB}', 0xEB), ('\u{05DC}', 0xEC), ('\u{05DD}', 0xED), 
    ('\u{05DE}', 0xEE), ('\u{05DF}', 0xEF), ('\u{05E0}', 0xF0), ('\u{05E1}', 0xF1), 
    ('\u{05E2}', 0xF2), ('\u{05E3}', 0xF3), ('\u{05E4}', 0xF4), ('\u{05E5}', 0xF5), 
    ('\u{05E6}', 0xF6), ('\u{05E7}', 0xF7), ('\u{05E8}', 0xF8), ('\u{05E9}', 0xF9), 
    ('\u{05EA}', 0xFA), ('\u{200E}', 0xFD), ('\u{200F}', 0xFE), ('\u{2017}', 0xDF), 
];