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
    '\u{00A0}','\u{0104}','\u{0138}','\u{0156}','\u{00A4}','\u{0128}','\u{013B}','\u{00A7}',
    '\u{00A8}','\u{0160}','\u{0112}','\u{0122}','\u{0166}','\u{00AD}','\u{017D}','\u{00AF}',
    '\u{00B0}','\u{0105}','\u{02DB}','\u{0157}','\u{00B4}','\u{0129}','\u{013C}','\u{02C7}',
    '\u{00B8}','\u{0161}','\u{0113}','\u{0123}','\u{0167}','\u{014A}','\u{017E}','\u{014B}',
    '\u{0100}','\u{00C1}','\u{00C2}','\u{00C3}','\u{00C4}','\u{00C5}','\u{00C6}','\u{012E}',
    '\u{010C}','\u{00C9}','\u{0118}','\u{00CB}','\u{0116}','\u{00CD}','\u{00CE}','\u{012A}',
    '\u{0110}','\u{0145}','\u{014C}','\u{0136}','\u{00D4}','\u{00D5}','\u{00D6}','\u{00D7}',
    '\u{00D8}','\u{0172}','\u{00DA}','\u{00DB}','\u{00DC}','\u{0168}','\u{016A}','\u{00DF}',
    '\u{0101}','\u{00E1}','\u{00E2}','\u{00E3}','\u{00E4}','\u{00E5}','\u{00E6}','\u{012F}',
    '\u{010D}','\u{00E9}','\u{0119}','\u{00EB}','\u{0117}','\u{00ED}','\u{00EE}','\u{012B}',
    '\u{0111}','\u{0146}','\u{014D}','\u{0137}','\u{00F4}','\u{00F5}','\u{00F6}','\u{00F7}',
    '\u{00F8}','\u{0173}','\u{00FA}','\u{00FB}','\u{00FC}','\u{0169}','\u{016B}','\u{02D9}',
];

const ENCODE_TABLE: [(char, u8); 128] = [
    ('\u{0080}', 0x80), ('\u{0081}', 0x81), ('\u{0082}', 0x82), ('\u{0083}', 0x83), 
    ('\u{0084}', 0x84), ('\u{0085}', 0x85), ('\u{0086}', 0x86), ('\u{0087}', 0x87), 
    ('\u{0088}', 0x88), ('\u{0089}', 0x89), ('\u{008A}', 0x8A), ('\u{008B}', 0x8B), 
    ('\u{008C}', 0x8C), ('\u{008D}', 0x8D), ('\u{008E}', 0x8E), ('\u{008F}', 0x8F), 
    ('\u{0090}', 0x90), ('\u{0091}', 0x91), ('\u{0092}', 0x92), ('\u{0093}', 0x93), 
    ('\u{0094}', 0x94), ('\u{0095}', 0x95), ('\u{0096}', 0x96), ('\u{0097}', 0x97), 
    ('\u{0098}', 0x98), ('\u{0099}', 0x99), ('\u{009A}', 0x9A), ('\u{009B}', 0x9B), 
    ('\u{009C}', 0x9C), ('\u{009D}', 0x9D), ('\u{009E}', 0x9E), ('\u{009F}', 0x9F), 
    ('\u{00A0}', 0xA0), ('\u{00A4}', 0xA4), ('\u{00A7}', 0xA7), ('\u{00A8}', 0xA8), 
    ('\u{00AD}', 0xAD), ('\u{00AF}', 0xAF), ('\u{00B0}', 0xB0), ('\u{00B4}', 0xB4), 
    ('\u{00B8}', 0xB8), ('\u{00C1}', 0xC1), ('\u{00C2}', 0xC2), ('\u{00C3}', 0xC3), 
    ('\u{00C4}', 0xC4), ('\u{00C5}', 0xC5), ('\u{00C6}', 0xC6), ('\u{00C9}', 0xC9), 
    ('\u{00CB}', 0xCB), ('\u{00CD}', 0xCD), ('\u{00CE}', 0xCE), ('\u{00D4}', 0xD4), 
    ('\u{00D5}', 0xD5), ('\u{00D6}', 0xD6), ('\u{00D7}', 0xD7), ('\u{00D8}', 0xD8), 
    ('\u{00DA}', 0xDA), ('\u{00DB}', 0xDB), ('\u{00DC}', 0xDC), ('\u{00DF}', 0xDF), 
    ('\u{00E1}', 0xE1), ('\u{00E2}', 0xE2), ('\u{00E3}', 0xE3), ('\u{00E4}', 0xE4), 
    ('\u{00E5}', 0xE5), ('\u{00E6}', 0xE6), ('\u{00E9}', 0xE9), ('\u{00EB}', 0xEB), 
    ('\u{00ED}', 0xED), ('\u{00EE}', 0xEE), ('\u{00F4}', 0xF4), ('\u{00F5}', 0xF5), 
    ('\u{00F6}', 0xF6), ('\u{00F7}', 0xF7), ('\u{00F8}', 0xF8), ('\u{00FA}', 0xFA), 
    ('\u{00FB}', 0xFB), ('\u{00FC}', 0xFC), ('\u{0100}', 0xC0), ('\u{0101}', 0xE0), 
    ('\u{0104}', 0xA1), ('\u{0105}', 0xB1), ('\u{010C}', 0xC8), ('\u{010D}', 0xE8), 
    ('\u{0110}', 0xD0), ('\u{0111}', 0xF0), ('\u{0112}', 0xAA), ('\u{0113}', 0xBA), 
    ('\u{0116}', 0xCC), ('\u{0117}', 0xEC), ('\u{0118}', 0xCA), ('\u{0119}', 0xEA), 
    ('\u{0122}', 0xAB), ('\u{0123}', 0xBB), ('\u{0128}', 0xA5), ('\u{0129}', 0xB5), 
    ('\u{012A}', 0xCF), ('\u{012B}', 0xEF), ('\u{012E}', 0xC7), ('\u{012F}', 0xE7), 
    ('\u{0136}', 0xD3), ('\u{0137}', 0xF3), ('\u{0138}', 0xA2), ('\u{013B}', 0xA6), 
    ('\u{013C}', 0xB6), ('\u{0145}', 0xD1), ('\u{0146}', 0xF1), ('\u{014A}', 0xBD), 
    ('\u{014B}', 0xBF), ('\u{014C}', 0xD2), ('\u{014D}', 0xF2), ('\u{0156}', 0xA3), 
    ('\u{0157}', 0xB3), ('\u{0160}', 0xA9), ('\u{0161}', 0xB9), ('\u{0166}', 0xAC), 
    ('\u{0167}', 0xBC), ('\u{0168}', 0xDD), ('\u{0169}', 0xFD), ('\u{016A}', 0xDE), 
    ('\u{016B}', 0xFE), ('\u{0172}', 0xD9), ('\u{0173}', 0xF9), ('\u{017D}', 0xAE), 
    ('\u{017E}', 0xBE), ('\u{02C7}', 0xB7), ('\u{02D9}', 0xFF), ('\u{02DB}', 0xB2), 
];