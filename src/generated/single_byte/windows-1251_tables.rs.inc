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
    '\u{0402}','\u{0403}','\u{201A}','\u{0453}','\u{201E}','\u{2026}','\u{2020}','\u{2021}',
    '\u{20AC}','\u{2030}','\u{0409}','\u{2039}','\u{040A}','\u{040C}','\u{040B}','\u{040F}',
    '\u{0452}','\u{2018}','\u{2019}','\u{201C}','\u{201D}','\u{2022}','\u{2013}','\u{2014}',
    '\u{0098}','\u{2122}','\u{0459}','\u{203A}','\u{045A}','\u{045C}','\u{045B}','\u{045F}',
    '\u{00A0}','\u{040E}','\u{045E}','\u{0408}','\u{00A4}','\u{0490}','\u{00A6}','\u{00A7}',
    '\u{0401}','\u{00A9}','\u{0404}','\u{00AB}','\u{00AC}','\u{00AD}','\u{00AE}','\u{0407}',
    '\u{00B0}','\u{00B1}','\u{0406}','\u{0456}','\u{0491}','\u{00B5}','\u{00B6}','\u{00B7}',
    '\u{0451}','\u{2116}','\u{0454}','\u{00BB}','\u{0458}','\u{0405}','\u{0455}','\u{0457}',
    '\u{0410}','\u{0411}','\u{0412}','\u{0413}','\u{0414}','\u{0415}','\u{0416}','\u{0417}',
    '\u{0418}','\u{0419}','\u{041A}','\u{041B}','\u{041C}','\u{041D}','\u{041E}','\u{041F}',
    '\u{0420}','\u{0421}','\u{0422}','\u{0423}','\u{0424}','\u{0425}','\u{0426}','\u{0427}',
    '\u{0428}','\u{0429}','\u{042A}','\u{042B}','\u{042C}','\u{042D}','\u{042E}','\u{042F}',
    '\u{0430}','\u{0431}','\u{0432}','\u{0433}','\u{0434}','\u{0435}','\u{0436}','\u{0437}',
    '\u{0438}','\u{0439}','\u{043A}','\u{043B}','\u{043C}','\u{043D}','\u{043E}','\u{043F}',
    '\u{0440}','\u{0441}','\u{0442}','\u{0443}','\u{0444}','\u{0445}','\u{0446}','\u{0447}',
    '\u{0448}','\u{0449}','\u{044A}','\u{044B}','\u{044C}','\u{044D}','\u{044E}','\u{044F}',
];

const ENCODE_TABLE: [(char, u8); 128] = [
    ('\u{0098}', 0x98), ('\u{00A0}', 0xA0), ('\u{00A4}', 0xA4), ('\u{00A6}', 0xA6), 
    ('\u{00A7}', 0xA7), ('\u{00A9}', 0xA9), ('\u{00AB}', 0xAB), ('\u{00AC}', 0xAC), 
    ('\u{00AD}', 0xAD), ('\u{00AE}', 0xAE), ('\u{00B0}', 0xB0), ('\u{00B1}', 0xB1), 
    ('\u{00B5}', 0xB5), ('\u{00B6}', 0xB6), ('\u{00B7}', 0xB7), ('\u{00BB}', 0xBB), 
    ('\u{0401}', 0xA8), ('\u{0402}', 0x80), ('\u{0403}', 0x81), ('\u{0404}', 0xAA), 
    ('\u{0405}', 0xBD), ('\u{0406}', 0xB2), ('\u{0407}', 0xAF), ('\u{0408}', 0xA3), 
    ('\u{0409}', 0x8A), ('\u{040A}', 0x8C), ('\u{040B}', 0x8E), ('\u{040C}', 0x8D), 
    ('\u{040E}', 0xA1), ('\u{040F}', 0x8F), ('\u{0410}', 0xC0), ('\u{0411}', 0xC1), 
    ('\u{0412}', 0xC2), ('\u{0413}', 0xC3), ('\u{0414}', 0xC4), ('\u{0415}', 0xC5), 
    ('\u{0416}', 0xC6), ('\u{0417}', 0xC7), ('\u{0418}', 0xC8), ('\u{0419}', 0xC9), 
    ('\u{041A}', 0xCA), ('\u{041B}', 0xCB), ('\u{041C}', 0xCC), ('\u{041D}', 0xCD), 
    ('\u{041E}', 0xCE), ('\u{041F}', 0xCF), ('\u{0420}', 0xD0), ('\u{0421}', 0xD1), 
    ('\u{0422}', 0xD2), ('\u{0423}', 0xD3), ('\u{0424}', 0xD4), ('\u{0425}', 0xD5), 
    ('\u{0426}', 0xD6), ('\u{0427}', 0xD7), ('\u{0428}', 0xD8), ('\u{0429}', 0xD9), 
    ('\u{042A}', 0xDA), ('\u{042B}', 0xDB), ('\u{042C}', 0xDC), ('\u{042D}', 0xDD), 
    ('\u{042E}', 0xDE), ('\u{042F}', 0xDF), ('\u{0430}', 0xE0), ('\u{0431}', 0xE1), 
    ('\u{0432}', 0xE2), ('\u{0433}', 0xE3), ('\u{0434}', 0xE4), ('\u{0435}', 0xE5), 
    ('\u{0436}', 0xE6), ('\u{0437}', 0xE7), ('\u{0438}', 0xE8), ('\u{0439}', 0xE9), 
    ('\u{043A}', 0xEA), ('\u{043B}', 0xEB), ('\u{043C}', 0xEC), ('\u{043D}', 0xED), 
    ('\u{043E}', 0xEE), ('\u{043F}', 0xEF), ('\u{0440}', 0xF0), ('\u{0441}', 0xF1), 
    ('\u{0442}', 0xF2), ('\u{0443}', 0xF3), ('\u{0444}', 0xF4), ('\u{0445}', 0xF5), 
    ('\u{0446}', 0xF6), ('\u{0447}', 0xF7), ('\u{0448}', 0xF8), ('\u{0449}', 0xF9), 
    ('\u{044A}', 0xFA), ('\u{044B}', 0xFB), ('\u{044C}', 0xFC), ('\u{044D}', 0xFD), 
    ('\u{044E}', 0xFE), ('\u{044F}', 0xFF), ('\u{0451}', 0xB8), ('\u{0452}', 0x90), 
    ('\u{0453}', 0x83), ('\u{0454}', 0xBA), ('\u{0455}', 0xBE), ('\u{0456}', 0xB3), 
    ('\u{0457}', 0xBF), ('\u{0458}', 0xBC), ('\u{0459}', 0x9A), ('\u{045A}', 0x9C), 
    ('\u{045B}', 0x9E), ('\u{045C}', 0x9D), ('\u{045E}', 0xA2), ('\u{045F}', 0x9F), 
    ('\u{0490}', 0xA5), ('\u{0491}', 0xB4), ('\u{2013}', 0x96), ('\u{2014}', 0x97), 
    ('\u{2018}', 0x91), ('\u{2019}', 0x92), ('\u{201A}', 0x82), ('\u{201C}', 0x93), 
    ('\u{201D}', 0x94), ('\u{201E}', 0x84), ('\u{2020}', 0x86), ('\u{2021}', 0x87), 
    ('\u{2022}', 0x95), ('\u{2026}', 0x85), ('\u{2030}', 0x89), ('\u{2039}', 0x8B), 
    ('\u{203A}', 0x9B), ('\u{20AC}', 0x88), ('\u{2116}', 0xB9), ('\u{2122}', 0x99), 
];