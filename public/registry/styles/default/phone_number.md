---
title: "Phone Number"
name: "phone_number"
cargo_dependencies: []
registry_dependencies: []
type: "components:utils"
path: "utils/phone_number.rs"
---

# Phone Number

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add phone_number
```

## Component Code

```rust
use super::country::Country;

/// A phone number containing only digits.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Creates a new PhoneNumber with a maximum number of digits.
    pub fn new(input: &str, max_digits: usize) -> Self {
        Self(input.chars().filter(|c| c.is_ascii_digit()).take(max_digits).collect())
    }

    /// Returns the raw digits as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Formats the phone number according to the given country's format.
    pub fn format(&self, country: Country) -> String {
        PhoneFormat::for_country(country).format(&self.0)
    }

    /// Returns the full international format: "+33 6 12 34 56 78"
    pub fn format_international(&self, country: Country) -> String {
        if self.0.is_empty() {
            return String::new();
        }
        format!("{} {}", country.dial_code_formatted(), self.format(country))
    }
}

/// Phone number formatting rules for a country.
pub struct PhoneFormat {
    /// Digit group sizes (e.g., [2, 2, 2, 2, 2] for France "06 12 34 56 78")
    pub groups: &'static [usize],
    /// Maximum digits allowed (excluding country code)
    pub max_digits: usize,
}

impl PhoneFormat {
    /// Returns the phone format for a given country.
    pub const fn for_country(country: Country) -> Self {
        match country {
            // North America (NANP): 3-3-4, 10 digits
            Country::UnitedStatesOfAmerica
            | Country::Canada
            | Country::Bahamas
            | Country::Barbados
            | Country::DominicanRepublic
            | Country::Jamaica
            | Country::TrinidadAndTobago => Self { groups: &[3, 3, 4], max_digits: 10 },

            // France: 2-2-2-2-2, 10 digits
            Country::France => Self { groups: &[2, 2, 2, 2, 2], max_digits: 10 },

            // UK: 4-3-4, 11 digits
            Country::UnitedKingdom => Self { groups: &[4, 3, 4], max_digits: 11 },

            // Germany: 3-4-4, 11 digits
            Country::Germany => Self { groups: &[3, 4, 4], max_digits: 11 },

            // Thailand: 3-3-4, 10 digits
            Country::Thailand => Self { groups: &[3, 3, 4], max_digits: 10 },

            // Japan: 3-4-4, 11 digits
            Country::Japan => Self { groups: &[3, 4, 4], max_digits: 11 },

            // China: 3-4-4, 11 digits
            Country::China => Self { groups: &[3, 4, 4], max_digits: 11 },

            // India: 5-5, 10 digits
            Country::India => Self { groups: &[5, 5], max_digits: 10 },

            // Australia: 3-3-3, 9 digits
            Country::Australia => Self { groups: &[3, 3, 3], max_digits: 9 },

            // Brazil: 2-5-4, 11 digits
            Country::Brazil => Self { groups: &[2, 5, 4], max_digits: 11 },

            // Mexico: 2-4-4, 10 digits
            Country::Mexico => Self { groups: &[2, 4, 4], max_digits: 10 },

            // Spain: 3-3-3, 9 digits
            Country::Spain => Self { groups: &[3, 3, 3], max_digits: 9 },

            // Italy: 3-3-4, 10 digits
            Country::Italy => Self { groups: &[3, 3, 4], max_digits: 10 },

            // Default: 3-3-4, 15 digits (ITU-T E.164 max)
            _ => Self { groups: &[3, 3, 4], max_digits: 15 },
        }
    }

    /// Formats a raw digit string according to the format groups.
    pub fn format(&self, digits: &str) -> String {
        let mut result = String::new();
        let mut chars = digits.chars().peekable();

        for (i, &group_size) in self.groups.iter().enumerate() {
            if chars.peek().is_none() {
                break;
            }

            if i > 0 {
                result.push(' ');
            }

            for _ in 0..group_size {
                if let Some(c) = chars.next() {
                    result.push(c);
                } else {
                    break;
                }
            }
        }

        // Append any remaining digits (for countries with variable length)
        for c in chars {
            result.push(c);
        }

        result
    }

    /// Returns a placeholder string showing the format pattern.
    /// Example: "06 12 34 56 78" for France.
    pub fn placeholder(&self) -> String {
        let digits: String = (0..self.max_digits).map(|i| char::from(b'0' + (i % 10) as u8)).collect();
        self.format(&digits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────────────────────────────────────────────────────
    // Format tests
    // ─────────────────────────────────────────────────────────────

    #[test]
    fn test_france_format() {
        let format = PhoneFormat::for_country(Country::France);
        assert_eq!(format.format("0612345678"), "06 12 34 56 78");
        assert_eq!(format.format("061234"), "06 12 34");
        assert_eq!(format.format("06"), "06");
        assert_eq!(format.max_digits, 10);
    }

    #[test]
    fn test_usa_format() {
        let format = PhoneFormat::for_country(Country::UnitedStatesOfAmerica);
        assert_eq!(format.format("5551234567"), "555 123 4567");
        assert_eq!(format.format("555123"), "555 123");
        assert_eq!(format.format("555"), "555");
        assert_eq!(format.max_digits, 10);
    }

    #[test]
    fn test_canada_format() {
        let format = PhoneFormat::for_country(Country::Canada);
        assert_eq!(format.format("4161234567"), "416 123 4567");
        assert_eq!(format.max_digits, 10);
    }

    #[test]
    fn test_uk_format() {
        let format = PhoneFormat::for_country(Country::UnitedKingdom);
        assert_eq!(format.format("07123456789"), "0712 345 6789");
        assert_eq!(format.format("0712345"), "0712 345");
        assert_eq!(format.max_digits, 11);
    }

    #[test]
    fn test_germany_format() {
        let format = PhoneFormat::for_country(Country::Germany);
        assert_eq!(format.format("01512345678"), "015 1234 5678");
        assert_eq!(format.max_digits, 11);
    }

    #[test]
    fn test_thailand_format() {
        let format = PhoneFormat::for_country(Country::Thailand);
        assert_eq!(format.format("0812345678"), "081 234 5678");
        assert_eq!(format.max_digits, 10);
    }

    #[test]
    fn test_japan_format() {
        let format = PhoneFormat::for_country(Country::Japan);
        assert_eq!(format.format("09012345678"), "090 1234 5678");
        assert_eq!(format.max_digits, 11);
    }

    #[test]
    fn test_china_format() {
        let format = PhoneFormat::for_country(Country::China);
        assert_eq!(format.format("13812345678"), "138 1234 5678");
        assert_eq!(format.max_digits, 11);
    }

    #[test]
    fn test_india_format() {
        let format = PhoneFormat::for_country(Country::India);
        assert_eq!(format.format("9876543210"), "98765 43210");
        assert_eq!(format.max_digits, 10);
    }

    #[test]
    fn test_australia_format() {
        let format = PhoneFormat::for_country(Country::Australia);
        assert_eq!(format.format("412345678"), "412 345 678");
        assert_eq!(format.max_digits, 9);
    }

    #[test]
    fn test_brazil_format() {
        let format = PhoneFormat::for_country(Country::Brazil);
        assert_eq!(format.format("11987654321"), "11 98765 4321");
        assert_eq!(format.max_digits, 11);
    }

    #[test]
    fn test_mexico_format() {
        let format = PhoneFormat::for_country(Country::Mexico);
        assert_eq!(format.format("5512345678"), "55 1234 5678");
        assert_eq!(format.max_digits, 10);
    }

    #[test]
    fn test_spain_format() {
        let format = PhoneFormat::for_country(Country::Spain);
        assert_eq!(format.format("612345678"), "612 345 678");
        assert_eq!(format.max_digits, 9);
    }

    #[test]
    fn test_italy_format() {
        let format = PhoneFormat::for_country(Country::Italy);
        assert_eq!(format.format("3123456789"), "312 345 6789");
        assert_eq!(format.max_digits, 10);
    }

    // ─────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────

    #[test]
    fn test_empty_input() {
        let format = PhoneFormat::for_country(Country::France);
        assert_eq!(format.format(""), "");
    }

    #[test]
    fn test_single_digit() {
        let format = PhoneFormat::for_country(Country::France);
        assert_eq!(format.format("0"), "0");
    }

    #[test]
    fn test_partial_group() {
        let format = PhoneFormat::for_country(Country::France);
        assert_eq!(format.format("061"), "06 1");
        assert_eq!(format.format("0612"), "06 12");
        assert_eq!(format.format("06123"), "06 12 3");
    }

    #[test]
    fn test_default_format_for_unknown_country() {
        // Countries without specific format should use default 3-3-4
        let format = PhoneFormat::for_country(Country::Afghanistan);
        assert_eq!(format.format("1234567890"), "123 456 7890");
        assert_eq!(format.max_digits, 15);
    }

    #[test]
    fn test_overflow_digits_appended() {
        // Extra digits beyond format should be appended
        let format = PhoneFormat::for_country(Country::Spain);
        // Spain is 3-3-3 (9 digits), but if we pass more they get appended
        assert_eq!(format.format("123456789012"), "123 456 789012");
    }

    // ─────────────────────────────────────────────────────────────
    // NANP countries share same format
    // ─────────────────────────────────────────────────────────────

    #[test]
    fn test_nanp_countries_same_format() {
        let countries = [
            Country::UnitedStatesOfAmerica,
            Country::Canada,
            Country::Bahamas,
            Country::Barbados,
            Country::DominicanRepublic,
            Country::Jamaica,
            Country::TrinidadAndTobago,
        ];

        for country in countries {
            let format = PhoneFormat::for_country(country);
            assert_eq!(format.groups, &[3, 3, 4], "NANP format for {:?}", country);
            assert_eq!(format.max_digits, 10, "NANP max_digits for {:?}", country);
        }
    }

    // ─────────────────────────────────────────────────────────────
    // PhoneNumber tests
    // ─────────────────────────────────────────────────────────────

    #[test]
    fn test_phone_number_filters_non_digits() {
        let phone = PhoneNumber::new("06 12-34.56", 10);
        assert_eq!(phone.as_str(), "06123456");
    }

    #[test]
    fn test_phone_number_respects_max_digits() {
        let phone = PhoneNumber::new("0612345678901234", 10);
        assert_eq!(phone.as_str(), "0612345678");
    }

    #[test]
    fn test_phone_number_format() {
        let phone = PhoneNumber::new("0612345678", 10);
        assert_eq!(phone.format(Country::France), "06 12 34 56 78");
    }

    #[test]
    fn test_phone_number_default_is_empty() {
        let phone = PhoneNumber::default();
        assert_eq!(phone.as_str(), "");
        assert!(phone.is_empty());
    }

    #[test]
    fn test_phone_number_format_international() {
        let phone = PhoneNumber::new("0612345678", 10);
        assert_eq!(phone.format_international(Country::France), "+33 06 12 34 56 78");
    }

    #[test]
    fn test_phone_number_format_international_empty() {
        let phone = PhoneNumber::default();
        assert_eq!(phone.format_international(Country::France), "");
    }

    #[test]
    fn test_placeholder_france() {
        let format = PhoneFormat::for_country(Country::France);
        assert_eq!(format.placeholder(), "01 23 45 67 89");
    }

    #[test]
    fn test_placeholder_usa() {
        let format = PhoneFormat::for_country(Country::UnitedStatesOfAmerica);
        assert_eq!(format.placeholder(), "012 345 6789");
    }
}
```
