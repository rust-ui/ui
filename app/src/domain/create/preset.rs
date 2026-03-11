/// Compact preset encoding for /create page state.
///
/// Bit layout (15 bits total):
///   bits 0-2:   base_color index  (3 bits, 7 values)
///   bits 3-5:   radius index      (3 bits, 5 values)
///   bits 6-10:  color_theme index (5 bits, 18 values: 0=None, 1-17=accent)
///   bits 11-14: font index        (4 bits, 16 values: 0=Inter default)
///
/// Format: 'a' (version prefix) + base62 chars
/// Example: Neutral + 0.5rem + None + Inter → bits = 0|(2<<3)|0|0 = 16 → "aG"
///
/// Backward-compat rules:
///   - Never reorder ThemeName::ALL, RADII, ColorTheme::ALL, or FontName indices — only append.
///   - Old presets (11-bit) decode font bits as 0 → Inter. No breakage.
///   - New fields must go at higher bit offsets.
use super::components::color_theme_picker::ColorTheme;
use super::components::font_picker::FontName;
use super::components::theme_picker::ThemeName;

const RADII: &[f32] = &[0.0, 0.3, 0.5, 0.75, 1.0];
const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const VERSION: char = 'a';

fn to_base62(mut n: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut bytes = Vec::new();
    while n > 0 {
        if let Some(&b) = BASE62.get((n % 62) as usize) {
            bytes.push(b);
        }
        n /= 62;
    }
    bytes.reverse();
    String::from_utf8(bytes).unwrap_or_default()
}

fn from_base62(s: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for b in s.bytes() {
        let idx = BASE62.iter().position(|&c| c == b)? as u32;
        result = result.checked_mul(62)?.checked_add(idx)?;
    }
    Some(result)
}

pub fn encode_preset(theme: ThemeName, radius: f32, color_theme: ColorTheme, font: FontName) -> String {
    let color_idx = theme.to_index() as u32;
    let radius_idx = RADII.iter().position(|&r| (r - radius).abs() < f32::EPSILON).unwrap_or(2) as u32;
    let ct_idx = color_theme.to_index() as u32;
    let font_idx = font.to_index() as u32;
    let bits = color_idx | (radius_idx << 3) | (ct_idx << 6) | (font_idx << 11);
    format!("{}{}", VERSION, to_base62(bits))
}

pub fn decode_preset(code: &str) -> Option<(ThemeName, f32, ColorTheme, FontName)> {
    let mut chars = code.chars();
    if chars.next()? != VERSION {
        return None;
    }
    let rest: String = chars.collect();
    if rest.is_empty() {
        return None;
    }
    let bits = from_base62(&rest)?;
    let color_idx = bits & 0b111;
    let radius_idx = ((bits >> 3) & 0b111) as usize;
    let ct_idx = (bits >> 6) & 0b11111;
    let font_idx = (bits >> 11) & 0b1111;
    let theme = ThemeName::from_index(color_idx)?;
    let radius = RADII.get(radius_idx).copied()?;
    let color_theme = ColorTheme::from_index(ct_idx).unwrap_or_default();
    let font = FontName::from_index(font_idx).unwrap_or_default();
    Some((theme, radius, color_theme, font))
}
