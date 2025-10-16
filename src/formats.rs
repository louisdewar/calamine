// SPDX-License-Identifier: MIT
//
// Copyright 2016-2025, Johann Tuffe.

use crate::datatype::{Data, DataRef, ExcelDateTime, ExcelDateTimeType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellFormat {
    Other,
    DateTime,
    TimeDelta,
}

/// ARGB color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255, 255 = fully opaque)
    pub a: u8,
}

impl Color {
    /// Parse from ARGB hex string (e.g., "FFFF0000" for opaque red)
    pub fn from_argb_hex(hex: &str) -> Result<Self, std::num::ParseIntError> {
        if hex.len() != 8 {
            return Err(u8::from_str_radix("", 16).unwrap_err());
        }
        let a = u8::from_str_radix(&hex[0..2], 16)?;
        let r = u8::from_str_radix(&hex[2..4], 16)?;
        let g = u8::from_str_radix(&hex[4..6], 16)?;
        let b = u8::from_str_radix(&hex[6..8], 16)?;
        Ok(Self { r, g, b, a })
    }

    /// Parse from RGB hex string (e.g., "FF0000" for opaque red)
    pub fn from_rgb_hex(hex: &str) -> Result<Self, std::num::ParseIntError> {
        if hex.len() != 6 {
            return Err(u8::from_str_radix("", 16).unwrap_err());
        }
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        Ok(Self { r, g, b, a: 0xFF })
    }

    /// Convert to ARGB hex string (e.g., "FFFF0000")
    pub fn to_argb_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}{:02X}", self.a, self.r, self.g, self.b)
    }

    /// Returns a tinted version of the color based on Excel tint adjustment.
    pub fn with_tint(self, tint: f64) -> Self {
        if tint == 0.0 {
            return self;
        }

        fn apply(channel: u8, tint: f64) -> u8 {
            let value = channel as f64 / 255.0;
            let adjusted = if tint < 0.0 {
                value * (1.0 + tint)
            } else {
                value + (1.0 - value) * tint
            };
            (adjusted.clamp(0.0, 1.0) * 255.0).round() as u8
        }

        Self {
            r: apply(self.r, tint),
            g: apply(self.g, tint),
            b: apply(self.b, tint),
            a: self.a,
        }
    }
}

/// Font style information
#[derive(Debug, Clone, PartialEq)]
pub struct FontStyle {
    /// Bold font
    pub bold: Option<bool>,
    /// Italic font
    pub italic: Option<bool>,
    /// Font color
    pub color: Option<Color>,
}

/// Fill style information
#[derive(Debug, Clone, PartialEq)]
pub struct FillStyle {
    /// Background color
    pub background_color: Option<Color>,
}

/// Horizontal alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Fill,
    Justify,
    CenterContinuous,
    Distributed,
    General,
}

/// Vertical alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    Justify,
    Distributed,
}

/// Alignment style information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlignmentStyle {
    /// Horizontal alignment
    pub horizontal: Option<HorizontalAlignment>,
    /// Vertical alignment
    pub vertical: Option<VerticalAlignment>,
    /// Indentation level (0-15)
    pub indent: Option<u8>,
}

/// Complete cell style with all formatting
#[derive(Debug, Clone, PartialEq)]
pub struct CellStyle {
    /// Font styling
    pub font: Option<FontStyle>,
    /// Fill styling
    pub fill: Option<FillStyle>,
    /// Alignment styling
    pub alignment: Option<AlignmentStyle>,
    /// Number format ID
    pub number_format_id: Option<u32>,
}

/// Default indexed color palette for Excel (legacy format).
/// These are RGB values (alpha is implicitly 255/opaque).
/// Indices 0-7 duplicate 8-15 for backwards compatibility.
pub const INDEXED_COLORS: &[Option<Color>; 66] = &[
    // 0-7: Duplicates of 8-15 (backwards compatibility)
    Some(Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }), // 0: Black
    Some(Color { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF }), // 1: White
    Some(Color { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF }), // 2: Red
    Some(Color { r: 0x00, g: 0xFF, b: 0x00, a: 0xFF }), // 3: Green
    Some(Color { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF }), // 4: Blue
    Some(Color { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF }), // 5: Yellow
    Some(Color { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF }), // 6: Magenta
    Some(Color { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF }), // 7: Cyan
    // 8-15: Standard colors
    Some(Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }), // 8: Black
    Some(Color { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF }), // 9: White
    Some(Color { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF }), // 10: Red
    Some(Color { r: 0x00, g: 0xFF, b: 0x00, a: 0xFF }), // 11: Green
    Some(Color { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF }), // 12: Blue
    Some(Color { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF }), // 13: Yellow
    Some(Color { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF }), // 14: Magenta
    Some(Color { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF }), // 15: Cyan
    // 16-63: Extended palette
    Some(Color { r: 0x80, g: 0x00, b: 0x00, a: 0xFF }), // 16: Maroon
    Some(Color { r: 0x00, g: 0x80, b: 0x00, a: 0xFF }), // 17: Dark Green
    Some(Color { r: 0x00, g: 0x00, b: 0x80, a: 0xFF }), // 18: Dark Blue
    Some(Color { r: 0x80, g: 0x80, b: 0x00, a: 0xFF }), // 19: Olive
    Some(Color { r: 0x80, g: 0x00, b: 0x80, a: 0xFF }), // 20: Purple
    Some(Color { r: 0x00, g: 0x80, b: 0x80, a: 0xFF }), // 21: Teal
    Some(Color { r: 0xC0, g: 0xC0, b: 0xC0, a: 0xFF }), // 22: Silver
    Some(Color { r: 0x80, g: 0x80, b: 0x80, a: 0xFF }), // 23: Gray
    Some(Color { r: 0x99, g: 0x99, b: 0xFF, a: 0xFF }), // 24
    Some(Color { r: 0x99, g: 0x33, b: 0x66, a: 0xFF }), // 25
    Some(Color { r: 0xFF, g: 0xFF, b: 0xCC, a: 0xFF }), // 26
    Some(Color { r: 0xCC, g: 0xFF, b: 0xFF, a: 0xFF }), // 27
    Some(Color { r: 0x66, g: 0x00, b: 0x66, a: 0xFF }), // 28
    Some(Color { r: 0xFF, g: 0x80, b: 0x80, a: 0xFF }), // 29
    Some(Color { r: 0x00, g: 0x66, b: 0xCC, a: 0xFF }), // 30
    Some(Color { r: 0xCC, g: 0xCC, b: 0xFF, a: 0xFF }), // 31
    Some(Color { r: 0x00, g: 0x00, b: 0x80, a: 0xFF }), // 32
    Some(Color { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF }), // 33
    Some(Color { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF }), // 34
    Some(Color { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF }), // 35
    Some(Color { r: 0x80, g: 0x00, b: 0x80, a: 0xFF }), // 36
    Some(Color { r: 0x80, g: 0x00, b: 0x00, a: 0xFF }), // 37
    Some(Color { r: 0x00, g: 0x80, b: 0x80, a: 0xFF }), // 38
    Some(Color { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF }), // 39
    Some(Color { r: 0x00, g: 0xCC, b: 0xFF, a: 0xFF }), // 40
    Some(Color { r: 0xCC, g: 0xFF, b: 0xFF, a: 0xFF }), // 41
    Some(Color { r: 0xCC, g: 0xFF, b: 0xCC, a: 0xFF }), // 42
    Some(Color { r: 0xFF, g: 0xFF, b: 0x99, a: 0xFF }), // 43
    Some(Color { r: 0x99, g: 0xCC, b: 0xFF, a: 0xFF }), // 44
    Some(Color { r: 0xFF, g: 0x99, b: 0xCC, a: 0xFF }), // 45
    Some(Color { r: 0xCC, g: 0x99, b: 0xFF, a: 0xFF }), // 46
    Some(Color { r: 0xFF, g: 0xCC, b: 0x99, a: 0xFF }), // 47
    Some(Color { r: 0x33, g: 0x66, b: 0xFF, a: 0xFF }), // 48
    Some(Color { r: 0x33, g: 0xCC, b: 0xCC, a: 0xFF }), // 49
    Some(Color { r: 0x99, g: 0xCC, b: 0x00, a: 0xFF }), // 50
    Some(Color { r: 0xFF, g: 0xCC, b: 0x00, a: 0xFF }), // 51
    Some(Color { r: 0xFF, g: 0x99, b: 0x00, a: 0xFF }), // 52
    Some(Color { r: 0xFF, g: 0x66, b: 0x00, a: 0xFF }), // 53
    Some(Color { r: 0x66, g: 0x66, b: 0x99, a: 0xFF }), // 54
    Some(Color { r: 0x96, g: 0x96, b: 0x96, a: 0xFF }), // 55
    Some(Color { r: 0x00, g: 0x33, b: 0x66, a: 0xFF }), // 56
    Some(Color { r: 0x33, g: 0x99, b: 0x66, a: 0xFF }), // 57
    Some(Color { r: 0x00, g: 0x33, b: 0x00, a: 0xFF }), // 58
    Some(Color { r: 0x33, g: 0x33, b: 0x00, a: 0xFF }), // 59
    Some(Color { r: 0x99, g: 0x33, b: 0x00, a: 0xFF }), // 60
    Some(Color { r: 0x99, g: 0x33, b: 0x66, a: 0xFF }), // 61
    Some(Color { r: 0x33, g: 0x33, b: 0x99, a: 0xFF }), // 62
    Some(Color { r: 0x33, g: 0x33, b: 0x33, a: 0xFF }), // 63
    None, // 64: System Foreground (not defined)
    None, // 65: System Background (not defined)
];

/// Built-in number format codes shipped with Excel (en-US locale).
pub const BUILTIN_NUMBER_FORMATS: &[(u32, &str)] = &[
    (0, "General"),
    (1, "0"),
    (2, "0.00"),
    (3, "#,##0"),
    (4, "#,##0.00"),
    (5, "$#,##0_);($#,##0)"),
    (6, "$#,##0_);[Red]($#,##0)"),
    (7, "$#,##0.00_);($#,##0.00)"),
    (8, "$#,##0.00_);[Red]($#,##0.00)"),
    (9, "0%"),
    (10, "0.00%"),
    (11, "0.00E+00"),
    (12, "# ?/?"),
    (13, "# ??/??"),
    (14, "mm-dd-yy"),
    (15, "d-mmm-yy"),
    (16, "d-mmm"),
    (17, "mmm-yy"),
    (18, "h:mm AM/PM"),
    (19, "h:mm:ss AM/PM"),
    (20, "h:mm"),
    (21, "h:mm:ss"),
    (22, "m/d/yy h:mm"),
    (37, "#,##0 ;(#,##0)"),
    (38, "#,##0 ;[Red](#,##0)"),
    (39, "#,##0.00;(#,##0.00)"),
    (40, "#,##0.00;[Red](#,##0.00)"),
    (41, "_(* #,##0_);_(* (#,##0);_(* \"-\"_);_(@_)"),
    (42, "_($* #,##0_);_($* (#,##0);_($* \"-\"_);_(@_)"),
    (43, "_(* #,##0.00_);_(* (#,##0.00);_(* \"-\"??);_(@_)"),
    (44, "_($* #,##0.00_);_($* (#,##0.00);_($* \"-\"??);_(@_)"),
    (45, "mm:ss"),
    (46, "[h]:mm:ss"),
    (47, "mmss.0"),
    (48, "##0.0E+0"),
    (49, "@"),
];

/// Check excel number format is datetime
pub fn detect_custom_number_format(format: &str) -> CellFormat {
    let mut escaped = false;
    let mut is_quote = false;
    let mut brackets = 0u8;
    let mut prev = ' ';
    let mut hms = false;
    let mut ap = false;
    for s in format.chars() {
        match (s, escaped, is_quote, ap, brackets) {
            (_, true, ..) => escaped = false, // if escaped, ignore
            ('_' | '\\', ..) => escaped = true,
            ('"', _, true, _, _) => is_quote = false,
            (_, _, true, _, _) => (),
            ('"', _, _, _, _) => is_quote = true,
            (';', ..) => return CellFormat::Other, // first format only
            ('[', ..) => brackets += 1,
            (']', .., 1) if hms => return CellFormat::TimeDelta, // if closing
            (']', ..) => brackets = brackets.saturating_sub(1),
            ('a' | 'A', _, _, false, 0) => ap = true,
            ('p' | 'm' | '/' | 'P' | 'M', _, _, true, 0) => return CellFormat::DateTime,
            ('d' | 'm' | 'h' | 'y' | 's' | 'D' | 'M' | 'H' | 'Y' | 'S', _, _, false, 0) => {
                return CellFormat::DateTime
            }
            _ => {
                if hms && s.eq_ignore_ascii_case(&prev) {
                    // ok ...
                } else {
                    hms = prev == '[' && matches!(s, 'm' | 'h' | 's' | 'M' | 'H' | 'S');
                }
            }
        }
        prev = s;
    }
    CellFormat::Other
}

pub fn builtin_format_by_id(id: &[u8]) -> CellFormat {
    match id {
        // mm-dd-yy
        b"14" |
        // d-mmm-yy
        b"15" |
        // d-mmm
        b"16" |
        // mmm-yy
        b"17" |
        // h:mm AM/PM
        b"18" |
        // h:mm:ss AM/PM
        b"19" |
        // h:mm
        b"20" |
        // h:mm:ss
        b"21" |
        // m/d/yy h:mm
        b"22" |
        // mm:ss
        b"45" |
        // mmss.0
        b"47" => CellFormat::DateTime,
        // [h]:mm:ss
        b"46" => CellFormat::TimeDelta,
        _ => CellFormat::Other
    }
}

/// Check if code corresponds to builtin date format
///
/// See `is_builtin_date_format_id`
pub fn builtin_format_by_code(code: u16) -> CellFormat {
    match code {
        14..=22 | 45 | 47 => CellFormat::DateTime,
        46 => CellFormat::TimeDelta,
        _ => CellFormat::Other,
    }
}

// convert i64 to date, if format == Date
pub fn format_excel_i64(value: i64, format: Option<&CellFormat>, is_1904: bool) -> Data {
    match format {
        Some(CellFormat::DateTime) => Data::DateTime(ExcelDateTime::new(
            value as f64,
            ExcelDateTimeType::DateTime,
            is_1904,
        )),
        Some(CellFormat::TimeDelta) => Data::DateTime(ExcelDateTime::new(
            value as f64,
            ExcelDateTimeType::TimeDelta,
            is_1904,
        )),
        _ => Data::Int(value),
    }
}

// convert f64 to date, if format == Date
#[inline]
pub fn format_excel_f64_ref(
    value: f64,
    format: Option<&CellFormat>,
    is_1904: bool,
) -> DataRef<'static> {
    match format {
        Some(CellFormat::DateTime) => DataRef::DateTime(ExcelDateTime::new(
            value,
            ExcelDateTimeType::DateTime,
            is_1904,
        )),
        Some(CellFormat::TimeDelta) => DataRef::DateTime(ExcelDateTime::new(
            value,
            ExcelDateTimeType::TimeDelta,
            is_1904,
        )),
        _ => DataRef::Float(value),
    }
}

// convert f64 to date, if format == Date
pub fn format_excel_f64(value: f64, format: Option<&CellFormat>, is_1904: bool) -> Data {
    format_excel_f64_ref(value, format, is_1904).into()
}

/// Ported from openpyxl, MIT License
/// https://foss.heptapod.net/openpyxl/openpyxl/-/blob/a5e197c530aaa49814fd1d993dd776edcec35105/openpyxl/styles/tests/test_number_style.py
#[test]
fn test_is_date_format() {
    assert_eq!(
        detect_custom_number_format("DD/MM/YY"),
        CellFormat::DateTime
    );
    assert_eq!(
        detect_custom_number_format("H:MM:SS;@"),
        CellFormat::DateTime
    );
    assert_eq!(
        detect_custom_number_format("#,##0\\ [$\\u20bd-46D]"),
        CellFormat::Other
    );
    assert_eq!(
        detect_custom_number_format("m\"M\"d\"D\";@"),
        CellFormat::DateTime
    );
    assert_eq!(
        detect_custom_number_format("[h]:mm:ss"),
        CellFormat::TimeDelta
    );
    assert_eq!(
        detect_custom_number_format("\"Y: \"0.00\"m\";\"Y: \"-0.00\"m\";\"Y: <num>m\";@"),
        CellFormat::Other
    );
    assert_eq!(
        detect_custom_number_format("#,##0\\ [$''u20bd-46D]"),
        CellFormat::Other
    );
    assert_eq!(
        detect_custom_number_format("\"$\"#,##0_);[Red](\"$\"#,##0)"),
        CellFormat::Other
    );
    assert_eq!(
        detect_custom_number_format("[$-404]e\"\\xfc\"m\"\\xfc\"d\"\\xfc\""),
        CellFormat::DateTime
    );
    assert_eq!(
        detect_custom_number_format("0_ ;[Red]\\-0\\ "),
        CellFormat::Other
    );
    assert_eq!(detect_custom_number_format("\\Y000000"), CellFormat::Other);
    assert_eq!(
        detect_custom_number_format("#,##0.0####\" YMD\""),
        CellFormat::Other
    );
    assert_eq!(detect_custom_number_format("[h]"), CellFormat::TimeDelta);
    assert_eq!(detect_custom_number_format("[ss]"), CellFormat::TimeDelta);
    assert_eq!(
        detect_custom_number_format("[s].000"),
        CellFormat::TimeDelta
    );
    assert_eq!(detect_custom_number_format("[m]"), CellFormat::TimeDelta);
    assert_eq!(detect_custom_number_format("[mm]"), CellFormat::TimeDelta);
    assert_eq!(
        detect_custom_number_format("[Blue]\\+[h]:mm;[Red]\\-[h]:mm;[Green][h]:mm"),
        CellFormat::TimeDelta
    );
    assert_eq!(
        detect_custom_number_format("[>=100][Magenta][s].00"),
        CellFormat::TimeDelta
    );
    assert_eq!(
        detect_custom_number_format("[h]:mm;[=0]\\-"),
        CellFormat::TimeDelta
    );
    assert_eq!(
        detect_custom_number_format("[>=100][Magenta].00"),
        CellFormat::Other
    );
    assert_eq!(
        detect_custom_number_format("[>=100][Magenta]General"),
        CellFormat::Other
    );
    assert_eq!(
        detect_custom_number_format("ha/p\\\\m"),
        CellFormat::DateTime
    );
    assert_eq!(
        detect_custom_number_format("#,##0.00\\ _M\"H\"_);[Red]#,##0.00\\ _M\"S\"_)"),
        CellFormat::Other
    );
}
