//! Real units stored as whole numbers, so sums are exact and every machine
//! gets the same result. See docs/technology.md.

use std::fmt;
use std::str::FromStr;

/// How much game time one tick is. Rates written "per second" in data are
/// per tick.
pub const SECONDS_PER_TICK: u64 = 1;

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

    pub fn checked_add(self, other: Mass) -> Option<Mass> {
        self.0.checked_add(other.0).map(Mass)
    }

    pub fn checked_sub(self, other: Mass) -> Option<Mass> {
        self.0.checked_sub(other.0).map(Mass)
    }
}

/// Units a mass can be written in, largest first, with their size in milligrams.
const MASS_UNITS: &[(&str, u64)] = &[
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
        parse_quantity(text, MASS_UNITS, "a mass like \"2 kg\" or \"900 g\"").map(Mass)
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (unit, scale) = largest_unit(self.0, MASS_UNITS);
        write!(f, "{} {unit}", format_scaled(self.0, scale))
    }
}

/// A temperature, stored in millikelvin.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Temperature(u64);

impl Temperature {
    pub const fn from_mk(mk: u64) -> Self {
        Temperature(mk)
    }

    pub const fn mk(self) -> u64 {
        self.0
    }
}

impl FromStr for Temperature {
    type Err = UnitError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        parse_quantity(text, &[("K", 1_000)], "a temperature like \"293 K\"").map(Temperature)
    }
}

impl fmt::Display for Temperature {
    /// Shown to the nearest kelvin.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} K", (self.0 + 500) / 1_000)
    }
}

/// An amount of energy, stored in microjoules.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Energy(u64);

impl Energy {
    pub const ZERO: Energy = Energy(0);

    pub const fn from_uj(uj: u64) -> Self {
        Energy(uj)
    }

    pub const fn uj(self) -> u64 {
        self.0
    }

    pub fn checked_add(self, other: Energy) -> Option<Energy> {
        self.0.checked_add(other.0).map(Energy)
    }

    pub fn checked_sub(self, other: Energy) -> Option<Energy> {
        self.0.checked_sub(other.0).map(Energy)
    }
}

const ENERGY_UNITS: &[(&str, u64)] = &[
    ("GJ", 1_000_000_000_000_000),
    ("MJ", 1_000_000_000_000),
    ("kJ", 1_000_000_000),
    ("J", 1_000_000),
    ("mJ", 1_000),
    ("µJ", 1),
];

impl fmt::Display for Energy {
    /// Shown to three decimal places in the largest fitting unit.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (unit, scale) = largest_unit(self.0, ENERGY_UNITS);
        write!(f, "{} {unit}", format_rounded(self.0, scale, 3))
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

/// Units for material properties in data files. Each converts to the whole
/// number the engine stores.
pub mod property {
    /// Specific heat, stored as µJ per mg per K (numerically equal to J/(kg*K)).
    pub const SPECIFIC_HEAT: &[(&str, u64)] = &[("kJ/(kg*K)", 1_000), ("J/(kg*K)", 1)];
    /// Chemical energy per mass, stored as µJ per mg (numerically equal to J/kg).
    pub const ENERGY_DENSITY: &[(&str, u64)] =
        &[("MJ/kg", 1_000_000), ("kJ/kg", 1_000), ("J/kg", 1)];
    /// Heat lost per kelvin of temperature difference, stored as µJ per K per tick.
    pub const HEAT_LOSS: &[(&str, u64)] = &[
        ("kW/K", 1_000_000_000 * super::SECONDS_PER_TICK),
        ("W/K", 1_000_000 * super::SECONDS_PER_TICK),
        ("mW/K", 1_000 * super::SECONDS_PER_TICK),
    ];
    /// Density, stored in g per cubic metre.
    pub const DENSITY: &[(&str, u64)] = &[("kg/m3", 1_000)];
    /// Speed of sound, stored in mm per second.
    pub const SPEED: &[(&str, u64)] = &[("m/s", 1_000)];
    /// A flow of mass, stored in mg per second.
    pub const MASS_PER_SECOND_FLOW: &[(&str, u64)] = &[("g/s", 1_000), ("mg/s", 1)];
    /// How fast the air cools with height, stored in mK per km.
    pub const LAPSE_RATE: &[(&str, u64)] = &[("K/km", 1_000)];
    /// Electrical resistivity, stored in pΩ·m.
    pub const RESISTIVITY: &[(&str, u64)] = &[("uOhm*m", 1_000_000), ("nOhm*m", 1_000)];
    /// Voltage, stored in µV.
    pub const VOLTAGE: &[(&str, u64)] = &[("V", 1_000_000), ("mV", 1_000)];
    /// Length, stored in µm.
    pub const LENGTH: &[(&str, u64)] = &[
        ("km", 1_000_000_000),
        ("m", 1_000_000),
        ("cm", 10_000),
        ("mm", 1_000),
        ("um", 1),
        ("µm", 1),
    ];
    /// Resistance per µm of roughness where two surfaces touch, stored in µΩ per µm.
    pub const TOUCH_RESISTANCE: &[(&str, u64)] = &[("Ohm/um", 1_000_000), ("mOhm/um", 1_000)];
    /// A span of time, stored in seconds.
    pub const DURATION: &[(&str, u64)] = &[("day", 86_400), ("h", 3_600), ("min", 60), ("s", 1)];
    /// Strength when pulled, stored in pascals.
    pub const STRESS: &[(&str, u64)] =
        &[("GPa", 1_000_000_000), ("MPa", 1_000_000), ("kPa", 1_000)];
    /// Power, stored in µW (so µJ per second).
    pub const POWER: &[(&str, u64)] = &[("kW", 1_000_000_000), ("W", 1_000_000), ("mW", 1_000)];
    /// In µJ.
    pub const ENERGY: &[(&str, u64)] = &[
        ("MJ", 1_000_000_000_000),
        ("kJ", 1_000_000_000),
        ("J", 1_000_000),
    ];
    /// Heat passed per square metre of surface per kelvin of difference,
    /// stored in mW per m² per K.
    pub const HEAT_TRANSFER: &[(&str, u64)] = &[("W/(m2*K)", 1_000), ("mW/(m2*K)", 1)];
    /// How fast a burning surface burns away, stored in mg per m² per second.
    pub const BURN_SPEED: &[(&str, u64)] = &[("g/(m2*s)", 1_000), ("mg/(m2*s)", 1)];
    /// Mass per second, stored in mg per second.
    pub const MASS_PER_SECOND: &[(&str, u64)] = &[("g/s", 1_000), ("mg/s", 1)];
    /// A rate of mass, stored in mg per day.
    pub const MASS_RATE: &[(&str, u64)] = &[
        ("kg/day", 1_000_000),
        ("g/day", 1_000),
        ("kg/h", 24_000_000),
        ("g/h", 24_000),
    ];
}

/// Units for showing stored whole numbers, largest first.
pub mod show {
    /// µm.
    pub const LENGTH: &[(&str, u64)] = &[("m", 1_000_000), ("mm", 1_000), ("µm", 1)];
    /// µm³.
    pub const VOLUME: &[(&str, u64)] = &[
        ("cm³", 1_000_000_000_000),
        ("mm³", 1_000_000_000),
        ("µm³", 1),
    ];
    /// µΩ.
    pub const RESISTANCE: &[(&str, u64)] = &[("Ω", 1_000_000), ("mΩ", 1_000), ("µΩ", 1)];
    /// µV.
    pub const VOLTAGE: &[(&str, u64)] = &[("V", 1_000_000), ("mV", 1_000), ("µV", 1)];
    /// µA.
    pub const CURRENT: &[(&str, u64)] = &[("A", 1_000_000), ("mA", 1_000), ("µA", 1)];
    /// µW, and µW per K.
    pub const POWER: &[(&str, u64)] = &[("W", 1_000_000), ("mW", 1_000), ("µW", 1)];
}

/// Shows a stored whole number in the largest fitting unit, to at most
/// `decimals` places.
pub fn show(value: u128, units: &[(&str, u64)], decimals: u32) -> String {
    let (unit, scale) = units
        .iter()
        .copied()
        .find(|&(_, scale)| value >= u128::from(scale))
        .unwrap_or(units[units.len() - 1]);
    let scale = u128::from(scale);
    let step = (scale / 10u128.pow(decimals)).max(1);
    let rounded = (value + step / 2) / step * step;
    let (whole, rest) = (rounded / scale, rounded % scale);
    if rest == 0 {
        return format!("{whole} {unit}");
    }
    let width = scale.ilog10() as usize;
    let fraction = format!("{rest:0width$}");
    format!("{whole}.{} {unit}", fraction.trim_end_matches('0'))
}

/// Shows a number of seconds as hours, minutes, and seconds.
pub fn show_duration(seconds: u64) -> String {
    let (h, m, s) = (seconds / 3_600, seconds / 60 % 60, seconds % 60);
    match (h, m, s) {
        (0, 0, s) => format!("{s} s"),
        (0, m, 0) => format!("{m} min"),
        (0, m, s) => format!("{m} min {s} s"),
        (h, 0, _) => format!("{h} h"),
        (h, m, _) => format!("{h} h {m} min"),
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

/// Parses "<number> <unit>" using `units` (name, scale) into a whole number.
pub fn parse_quantity(text: &str, units: &[(&str, u64)], expected: &str) -> Result<u64, UnitError> {
    let error = || UnitError(format!("{text:?} is not {expected}"));
    let trimmed = text.trim();
    let split = trimmed
        .find(|c: char| c.is_alphabetic())
        .ok_or_else(error)?;
    let (number, unit) = (trimmed[..split].trim(), trimmed[split..].trim());
    let scale = units
        .iter()
        .find(|(name, _)| *name == unit)
        .map(|&(_, scale)| scale)
        .ok_or_else(error)?;
    parse_scaled(number, scale).ok_or_else(error)
}

/// Parses a plain decimal like "4.5", scaled by `scale`.
pub fn parse_number(text: &str, scale: u64, expected: &str) -> Result<u64, UnitError> {
    parse_scaled(text.trim(), scale).ok_or_else(|| UnitError(format!("{text:?} is not {expected}")))
}

/// Parses a percentage like "60%" or "2.5%" into parts per ten thousand.
pub fn parse_percent(text: &str) -> Result<u64, UnitError> {
    let error = || UnitError(format!("{text:?} is not a percentage like \"40%\""));
    let number = text.trim().strip_suffix('%').ok_or_else(error)?;
    parse_scaled(number.trim(), 100).ok_or_else(error)
}

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

fn largest_unit<'a>(value: u64, units: &[(&'a str, u64)]) -> (&'a str, u64) {
    units
        .iter()
        .copied()
        .find(|&(_, scale)| value >= scale)
        .unwrap_or(units[units.len() - 1])
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

/// Formats `value / scale` rounded to at most `decimals` places.
fn format_rounded(value: u64, scale: u64, decimals: u32) -> String {
    let step = (scale / 10u64.pow(decimals)).max(1);
    let rounded = (u128::from(value) + u128::from(step) / 2) / u128::from(step) * u128::from(step);
    format_scaled(u64::try_from(rounded).unwrap_or(value), scale)
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

    #[test]
    fn parses_material_properties() {
        assert_eq!("1811 K".parse(), Ok(Temperature::from_mk(1_811_000)));
        assert_eq!(
            parse_quantity("449 J/(kg*K)", property::SPECIFIC_HEAT, "x"),
            Ok(449)
        );
        assert_eq!(
            parse_quantity("30 MJ/kg", property::ENERGY_DENSITY, "x"),
            Ok(30_000_000)
        );
        assert_eq!(
            parse_quantity("33 W/K", property::HEAT_LOSS, "x"),
            Ok(33_000_000)
        );
        assert_eq!(parse_percent("2.5%"), Ok(250));
        assert_eq!(parse_number("4.5", 100, "x"), Ok(450));
    }

    #[test]
    fn shows_stored_numbers_in_fitting_units() {
        assert_eq!(show(2_160_000, show::RESISTANCE, 3), "2.16 Ω");
        assert_eq!(show(1_883, show::RESISTANCE, 3), "1.883 mΩ");
        assert_eq!(show(2_590_673_570, show::VOLUME, 3), "2.591 mm³");
        assert_eq!(show_duration(9_600), "2 h 40 min");
        assert_eq!(show_duration(600), "10 min");
    }

    #[test]
    fn displays_energy_and_temperature() {
        assert_eq!(Energy::from_uj(30_000_000_000_000).to_string(), "30 MJ");
        assert_eq!(Energy::from_uj(1_234_567_890).to_string(), "1.235 kJ");
        assert_eq!(Temperature::from_mk(1_810_600).to_string(), "1811 K");
    }
}
