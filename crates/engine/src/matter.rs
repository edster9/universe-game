//! Matter: materials and what things are made of.
//!
//! A piece of matter has a composition (how much of each material) and an
//! amount of heat energy. Its temperature is worked out from the two, so
//! energy is what gets conserved exactly and temperature follows from it.
//! See docs/ideas/world-engine.md.

use std::collections::BTreeMap;

use crate::units::{Energy, Mass, Temperature};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialId(pub(crate) u16);

/// A material's properties, all from data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Material {
    pub key: String,
    pub label: String,
    pub melting_point: Temperature,
    pub boiling_point: Temperature,
    /// µJ per mg per K.
    pub specific_heat: u64,
    /// Resistance to being cut or shaped, in hundredths, at the reference
    /// temperature. A tool only works material softer than itself.
    pub hardness: u64,
    /// Chemical energy released by burning, in µJ per mg. Zero if it doesn't burn.
    pub energy_density: u64,
    /// What burning leaves behind, in parts per ten thousand by mass.
    pub burns_to: Vec<(MaterialId, u64)>,
    /// In g per cubic metre. Not used by any law yet.
    pub density: Option<u64>,
    /// In mm per second. Not used by any law yet.
    pub speed_of_sound: Option<u64>,
    /// Electrical resistivity in pΩ·m. `None` means it doesn't conduct.
    pub resistivity: Option<u64>,
    /// Voltage it gives when shaped as a source of charge, in µV.
    pub voltage: Option<u64>,
    /// Above this temperature, it catches fire in the open. `None` if it
    /// doesn't burn on its own.
    pub ignition_point: Option<Temperature>,
    /// How fast its burning surface burns away, in mg per m² per second.
    pub burn_speed: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Solid,
    Liquid,
    Gas,
}

impl Material {
    pub fn state_at(&self, temperature: Temperature) -> State {
        if temperature >= self.boiling_point {
            State::Gas
        } else if temperature >= self.melting_point {
            State::Liquid
        } else {
            State::Solid
        }
    }

    pub fn burns(&self) -> bool {
        self.energy_density > 0 && !self.burns_to.is_empty()
    }

    /// Hardness falls as a material heats, reaching nothing at its melting
    /// point. This is why hot metal can be shaped by a tool that can't touch
    /// it cold.
    pub fn hardness_at(&self, temperature: Temperature, reference: Temperature) -> u64 {
        if temperature <= reference {
            return self.hardness;
        }
        if temperature >= self.melting_point || self.melting_point <= reference {
            return 0;
        }
        let left = u128::from(self.melting_point.mk() - temperature.mk());
        let span = u128::from(self.melting_point.mk() - reference.mk());
        u64::try_from(u128::from(self.hardness) * left / span).unwrap_or(0)
    }
}

pub type Materials = BTreeMap<MaterialId, Material>;

/// How much of each material something is made of.
pub type Composition = BTreeMap<MaterialId, Mass>;

pub fn total_mass(composition: &Composition) -> u128 {
    composition.values().map(|m| u128::from(m.mg())).sum()
}

/// Volume in µm³ of the materials whose density is known. A trace of
/// something with no density (a gas just made by burning, say) doesn't make a
/// solid's size unknown. `None` only if no density is known at all.
pub fn volume(materials: &Materials, composition: &Composition) -> Option<u128> {
    let mut known = false;
    let mut total = 0u128;
    for (id, mass) in composition {
        if let Some(density) = materials[id].density.filter(|&d| d > 0) {
            known = true;
            // mg × 10¹⁵ / (g per m³) gives µm³.
            total += u128::from(mass.mg()) * 1_000_000_000_000_000 / u128::from(density);
        }
    }
    known.then_some(total)
}

/// Surface area in µm², treating the piece as a cube of its volume. `None`
/// if a density is unknown.
pub fn surface_area(materials: &Materials, composition: &Composition) -> Option<u128> {
    let side = cube_root(volume(materials, composition)?);
    Some(6 * side * side)
}

/// The whole-number cube root, rounded down.
pub fn cube_root(n: u128) -> u128 {
    let (mut low, mut high) = (0u128, 1u128 << 43);
    while low < high {
        let mid = (low + high).div_ceil(2);
        if mid
            .checked_mul(mid)
            .and_then(|m| m.checked_mul(mid))
            .is_some_and(|c| c <= n)
        {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}

/// Heat needed to raise the temperature by one kelvin, in µJ per K.
pub fn heat_capacity(materials: &Materials, composition: &Composition) -> u128 {
    composition
        .iter()
        .map(|(id, mass)| u128::from(mass.mg()) * u128::from(materials[id].specific_heat))
        .sum()
}

/// Energy stored in burnable materials, in µJ.
pub fn chemical_energy(materials: &Materials, composition: &Composition) -> u128 {
    composition
        .iter()
        .map(|(id, mass)| u128::from(mass.mg()) * u128::from(materials[id].energy_density))
        .sum()
}

/// The temperature of something holding `energy` with `capacity` µJ/K.
pub fn temperature(energy: Energy, capacity: u128) -> Temperature {
    if capacity == 0 {
        return Temperature::from_mk(0);
    }
    let mk = u128::from(energy.uj()) * 1_000 / capacity;
    Temperature::from_mk(u64::try_from(mk).unwrap_or(u64::MAX))
}

/// The heat energy something with `capacity` µJ/K holds at `temperature`.
pub fn energy_at(temperature: Temperature, capacity: u128) -> u128 {
    capacity * u128::from(temperature.mk()) / 1_000
}

/// The material there is most of. Ties go to the earlier material.
pub fn dominant(composition: &Composition) -> Option<MaterialId> {
    composition
        .iter()
        .max_by(|(a_id, a), (b_id, b)| a.cmp(b).then(b_id.cmp(a_id)))
        .map(|(&id, _)| id)
}

/// Takes `amount` from `composition` in proportion to what's there. The
/// shares add up to exactly `amount`. Returns `None` if there isn't enough.
pub fn proportional(composition: &Composition, amount: Mass) -> Option<Composition> {
    let total = total_mass(composition);
    let wanted = u128::from(amount.mg());
    if wanted > total || total == 0 {
        return None;
    }
    let mut shares: Composition = composition
        .iter()
        .map(|(&id, m)| {
            let share = u128::from(m.mg()) * wanted / total;
            (
                id,
                Mass::from_mg(u64::try_from(share).expect("a share is no bigger than its part")),
            )
        })
        .collect();
    // Rounding down leaves a few milligrams over; hand them out in order.
    let mut left = wanted - total_mass(&shares);
    for (id, share) in shares.iter_mut() {
        let room = u128::from(composition[id].mg() - share.mg());
        let extra = left.min(room);
        *share = Mass::from_mg(share.mg() + u64::try_from(extra).expect("fits"));
        left -= extra;
    }
    shares.retain(|_, m| *m != Mass::ZERO);
    Some(shares)
}

/// Splits `mass` into parts by `fractions` (parts per ten thousand, adding up
/// to ten thousand). Any milligrams lost to rounding go to the first part.
pub fn split_by_fractions(mass: Mass, fractions: &[(MaterialId, u64)]) -> Vec<(MaterialId, Mass)> {
    let mut parts: Vec<(MaterialId, Mass)> = fractions
        .iter()
        .map(|&(id, f)| {
            let share = u128::from(mass.mg()) * u128::from(f) / 10_000;
            (
                id,
                Mass::from_mg(u64::try_from(share).expect("a share is no bigger than the whole")),
            )
        })
        .collect();
    let given: u64 = parts.iter().map(|(_, m)| m.mg()).sum();
    if let Some(first) = parts.first_mut() {
        first.1 = Mass::from_mg(first.1.mg() + (mass.mg() - given));
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn composition(parts: &[(u16, u64)]) -> Composition {
        parts
            .iter()
            .map(|&(id, mg)| (MaterialId(id), Mass::from_mg(mg)))
            .collect()
    }

    #[test]
    fn proportional_shares_add_up_exactly() {
        let c = composition(&[(0, 3_000_001), (1, 2_000_000), (2, 7)]);
        let part = proportional(&c, Mass::from_mg(1_000_003)).unwrap();
        assert_eq!(total_mass(&part), 1_000_003);
        for (id, m) in &part {
            assert!(*m <= c[id]);
        }
        assert_eq!(proportional(&c, Mass::from_mg(6_000_000)), None);
    }

    #[test]
    fn cube_roots_are_exact_or_round_down() {
        assert_eq!(cube_root(27), 3);
        assert_eq!(cube_root(26), 2);
        assert_eq!(cube_root(1_000_000_000_000), 10_000);
    }

    #[test]
    fn fractions_add_up_exactly() {
        let parts = split_by_fractions(
            Mass::from_mg(1_001),
            &[(MaterialId(0), 9_500), (MaterialId(1), 500)],
        );
        assert_eq!(parts.iter().map(|(_, m)| m.mg()).sum::<u64>(), 1_001);
    }
}
