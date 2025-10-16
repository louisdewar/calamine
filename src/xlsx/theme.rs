// SPDX-License-Identifier: MIT
//
// Copyright 2016-2025.

use std::borrow::Cow;
use std::io::BufRead;

use quick_xml::events::{attributes::Attribute, BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader as XmlReader;

use crate::formats::Color;

use super::XlsxError;

/// Parsed workbook theme containing theme color definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    colors: [Option<Color>; 12],
}

impl Default for Theme {
    fn default() -> Self {
        Self { colors: [None; 12] }
    }
}

impl Theme {
    /// Returns the resolved theme color at the given index (0-11).
    #[inline]
    pub fn color(&self, index: usize) -> Option<Color> {
        self.colors.get(index).and_then(|c| *c)
    }

    /// Returns all parsed theme colors in SpreadsheetML order.
    #[inline]
    pub fn colors(&self) -> &[Option<Color>; 12] {
        &self.colors
    }

    fn set_color(&mut self, index: usize, color: Color) {
        if let Some(slot) = self.colors.get_mut(index) {
            *slot = Some(color);
        }
    }
}

/// Parse theme colors from the provided XML reader.
pub fn parse_theme<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Theme, XlsxError> {
    let mut buf = Vec::with_capacity(256);
    let mut theme = Theme::default();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) if e.local_name().as_ref() == b"clrScheme" => {
                parse_color_scheme(reader, e.name(), &mut theme)?;
            }
            Event::End(ref e) if e.local_name().as_ref() == b"themeElements" => break,
            Event::Eof => break,
            _ => (),
        }
    }

    Ok(theme)
}

fn parse_color_scheme<R: BufRead>(
    reader: &mut XmlReader<R>,
    end: QName<'_>,
    theme: &mut Theme,
) -> Result<(), XlsxError> {
    let mut buf = Vec::with_capacity(128);

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => {
                let local_name = e.local_name();
                let name = local_name.as_ref();
                if let Some(index) = theme_index(name) {
                    if let Some(color) = read_theme_color(reader, e)? {
                        theme.set_color(index, color);
                    }
                } else {
                    reader.read_to_end_into(e.name(), &mut Vec::new())?;
                }
            }
            Event::Empty(ref e) => {
                let local_name = e.local_name();
                let name = local_name.as_ref();
                if let Some(index) = theme_index(name) {
                    if let Some(color) = color_from_child(e)? {
                        theme.set_color(index, color);
                    }
                }
            }
            Event::End(ref e) if e.name() == end => break,
            Event::Eof => return Err(XlsxError::XmlEof("clrScheme")),
            _ => (),
        }
    }

    Ok(())
}

fn read_theme_color<R: BufRead>(
    reader: &mut XmlReader<R>,
    parent: &BytesStart<'_>,
) -> Result<Option<Color>, XlsxError> {
    let mut buf = Vec::with_capacity(64);
    let mut color = None;

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf)? {
            Event::Empty(ref e) => {
                if let Some(parsed) = color_from_child(e)? {
                    color = Some(parsed);
                }
            }
            Event::Start(ref e) => {
                if let Some(parsed) = color_from_child(e)? {
                    color = Some(parsed);
                }
                reader.read_to_end_into(e.name(), &mut Vec::new())?;
            }
            Event::End(ref e) if e.name() == parent.name() => break,
            Event::Eof => return Err(XlsxError::XmlEof("theme color")),
            _ => (),
        }
    }

    Ok(color)
}

fn color_from_child(e: &BytesStart<'_>) -> Result<Option<Color>, XlsxError> {
    let mut rgb = None;

    for attr in e.attributes() {
        match attr.map_err(XlsxError::XmlAttr)? {
            Attribute {
                key: QName(b"val"),
                value,
            } if e.local_name().as_ref() == b"srgbClr" => {
                let hex = bytes_to_str(&value)?;
                if let Ok(color) = Color::from_rgb_hex(hex) {
                    rgb = Some(color);
                }
            }
            Attribute {
                key: QName(b"lastClr"),
                value,
            } if e.local_name().as_ref() == b"sysClr" => {
                let hex = bytes_to_str(&value)?;
                if let Ok(color) = Color::from_rgb_hex(hex) {
                    rgb = Some(color);
                }
            }
            _ => (),
        }
    }

    Ok(rgb)
}

fn bytes_to_str<'a>(value: &'a Cow<'a, [u8]>) -> Result<&'a str, XlsxError> {
    Ok(std::str::from_utf8(value.as_ref()).map_err(|_| XlsxError::Unexpected("Invalid UTF-8"))?)
}

fn theme_index(name: &[u8]) -> Option<usize> {
    match name {
        b"lt1" => Some(0),
        b"dk1" => Some(1),
        b"lt2" => Some(2),
        b"dk2" => Some(3),
        b"accent1" => Some(4),
        b"accent2" => Some(5),
        b"accent3" => Some(6),
        b"accent4" => Some(7),
        b"accent5" => Some(8),
        b"accent6" => Some(9),
        b"hlink" => Some(10),
        b"folHlink" => Some(11),
        _ => None,
    }
}
