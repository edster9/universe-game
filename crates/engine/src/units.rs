//! Real units stored as whole numbers, so sums are exact and every machine
//! gets the same result. See docs/technology.md.

use std::fmt;
use std::str::FromStr;

/// A mass, stored in milligrams.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mass(u64);

impl Mass {
    pub const ZERO: Mass = Mass(0);

    pub const fn from_mg(mg: u64) -> Self {
        Mass(mg)
    }

    pub const fn mg(self) -> u64 {
        self.0
    }
}

/// Units a mass can be written in, largest first, with their size in milligrams.
const MASS_UNITS: [(&str, u64); 4] = [
    ("t", 1_000_000_000),
    ("kg", 1_000_000),
    ("g", 1_000),
    ("mg", 1),
];

impl FromStr for Mass {
    type Err = UnitError;

    /// Parses text like "72 kg", "1.2 kg", or "900 g". The value must come out
    /// to a whole number of milligrams.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let error = || UnitError(format!("{text:?} is not a mass like \"2 kg\" or \"900 g\""));
        let text = text.trim();
        let split = text
            .find(|c: char| c.is_ascii_alphabetic())
            .ok_or_else(error)?;
        let (number, unit) = (text[..split].trim(), text[split..].trim());
        let scale = MASS_UNITS
            .iter()
            .find(|(name, _)| *name == unit)
            .map(|&(_, scale)| scale)
            .ok_or_else(error)?;
        parse_scaled(number, scale).map(Mass).ok_or_else(error)
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (unit, scale) = MASS_UNITS
            .iter()
            .copied()
            .find(|&(_, scale)| self.0 >= scale)
            .unwrap_or(("mg", 1));
        write!(f, "{} {unit}", format_scaled(self.0, scale))
    }
}

/// An amount of credits. A plain counter until money becomes real in slice 3
/// (see docs/ideas/money.md).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Credits(u64);

impl Credits {
    pub const ZERO: Credits = Credits(0);

    pub const fn new(amount: u64) -> Self {
        Credits(amount)
    }

    pub const fn amount(self) -> u64 {
        self.0
    }

    pub fn checked_add(self, other: Credits) -> Option<Credits> {
        self.0.checked_add(other.0).map(Credits)
    }

    pub fn checked_sub(self, other: Credits) -> Option<Credits> {
        self.0.checked_sub(other.0).map(Credits)
    }
}

impl fmt::Display for Credits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            1 => write!(f, "1 credit"),
            n => write!(f, "{n} credits"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitError(String);

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for UnitError {}

/// Parses a decimal like "1.25" and multiplies it by `scale` without floating
/// point. Returns `None` if the result isn't a whole number or doesn't fit.
fn parse_scaled(number: &str, scale: u64) -> Option<u64> {
    let (whole, fraction) = number.split_once('.').unwrap_or((number, ""));
    let digits_only = |s: &str| s.chars().all(|c| c.is_ascii_digit());
    if (whole.is_empty() && fraction.is_empty()) || !digits_only(whole) || !digits_only(fraction) {
        return None;
    }
    let scale = u128::from(scale);
    let whole: u128 = if whole.is_empty() {
        0
    } else {
        whole.parse().ok()?
    };
    let mut value = whole.checked_mul(scale)?;
    if !fraction.is_empty() {
        let denominator = 10u128.checked_pow(u32::try_from(fraction.len()).ok()?)?;
        let scaled = fraction.parse::<u128>().ok()?.checked_mul(scale)?;
        if scaled % denominator != 0 {
            return None;
        }
        value = value.checked_add(scaled / denominator)?;
    }
    u64::try_from(value).ok()
}

/// Formats `value / scale` as an exact decimal with trailing zeros removed.
fn format_scaled(value: u64, scale: u64) -> String {
    let (whole, rest) = (value / scale, value % scale);
    if rest == 0 {
        return whole.to_string();
    }
    let width = scale.ilog10() as usize;
    let fraction = format!("{rest:0width$}");
    format!("{whole}.{}", fraction.trim_end_matches('0'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_masses_exactly() {
        assert_eq!("72 kg".parse(), Ok(Mass::from_mg(72_000_000)));
        assert_eq!("1.2 kg".parse(), Ok(Mass::from_mg(1_200_000)));
        assert_eq!("900 g".parse(), Ok(Mass::from_mg(900_000)));
        assert_eq!("64.5kg".parse(), Ok(Mass::from_mg(64_500_000)));
        assert_eq!("0.001 g".parse(), Ok(Mass::from_mg(1)));
    }

    #[test]
    fn refuses_masses_that_are_not_whole_milligrams_or_not_masses() {
        for text in [
            "0.5 mg",
            "-2 kg",
            "2 furlongs",
            "kg",
            "1.2.3 kg",
            "",
            "99999999999999 t",
        ] {
            assert!(text.parse::<Mass>().is_err(), "{text:?} should be refused");
        }
    }

    #[test]
    fn displays_masses_in_the_largest_whole_unit() {
        assert_eq!(Mass::from_mg(72_000_000).to_string(), "72 kg");
        assert_eq!(Mass::from_mg(1_200_000).to_string(), "1.2 kg");
        assert_eq!(Mass::from_mg(900_000).to_string(), "900 g");
        assert_eq!(Mass::from_mg(1_500).to_string(), "1.5 g");
        assert_eq!(Mass::ZERO.to_string(), "0 mg");
    }
}
