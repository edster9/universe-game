//! The gate: the one place every change to the world passes through. It
//! applies a set of changes all together or not at all, checks that mass,
//! energy, and credits are conserved and the world is still well formed, and
//! logs what happened. See docs/ideas/world-engine.md, "Conservation is the
//! core's job".

use std::fmt;

use crate::datasheet::Datasheet;
use crate::intent::Intent;
use crate::matter::{self, Composition, MaterialId};
use crate::units::{Credits, Energy, Mass};
use crate::world::{EntityId, World};

/// The only kinds of change the world allows. Laws propose them; only the
/// gate applies them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Change {
    /// Put `entity` in or on `to`: a place, a person, a container.
    Move { entity: EntityId, to: EntityId },
    /// Move credits from one wallet to another.
    Transfer {
        from: EntityId,
        to: EntityId,
        amount: Credits,
    },
    /// Move heat energy between two pieces of matter, or out to a place's
    /// surroundings.
    Heat {
        from: Holder,
        to: Holder,
        amount: Energy,
    },
    /// Burn `mass` of `material` inside `entity`. What burning leaves stays in
    /// the entity, and the chemical energy it released becomes the entity's heat.
    Burn {
        entity: EntityId,
        material: MaterialId,
        mass: Mass,
    },
    /// Make a new piece of matter at `at` out of part of `from`. Heat goes with
    /// it in proportion, so both halves keep the same temperature.
    Split {
        from: EntityId,
        take: Composition,
        at: EntityId,
    },
    /// Pour everything in `from` into `into`. `from` stops existing.
    Merge { from: EntityId, into: EntityId },
    /// Give a piece of matter a shape and the tolerance it was made to, or
    /// take its shape away.
    Shape {
        entity: EntityId,
        shape: Option<(String, u64)>,
    },
    /// Change how closely a shaped part matches its shape, in µm.
    Refine { entity: EntityId, tolerance: u64 },
    /// Put `parts` together to `design`, as a new thing at `at`, with the
    /// datasheet measured for it.
    Assemble {
        design: String,
        parts: Vec<EntityId>,
        at: EntityId,
        datasheet: Datasheet,
    },
    /// Take an assembly apart. Its parts are left where it was, and it stops
    /// existing.
    Disassemble { assembly: EntityId },
    /// Light or put out a chamber.
    Light { chamber: EntityId, lit: bool },
}

/// Something that can hold heat.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Holder {
    Thing(EntityId),
    /// A place's surroundings: the air and ground that heat escapes into.
    Surroundings(EntityId),
}

/// Why something happened.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cause {
    /// Someone did something.
    Action { actor: EntityId, intent: Intent },
    /// The laws acted on their own during a tick: burning, heat, melting.
    Nature { tick: u64 },
}

/// One accepted set of changes, as recorded by the gate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogEntry {
    pub seq: u64,
    pub cause: Cause,
    pub changes: Vec<Change>,
}

/// Why the gate refused a set of changes. The laws should have refused first,
/// so a fault means a law has a bug.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    UnknownEntity(EntityId),
    NotLocated(EntityId),
    IntoItself(EntityId),
    NoWallet(EntityId),
    SameWallet(EntityId),
    Insufficient {
        from: EntityId,
        amount: Credits,
    },
    Overflow(EntityId),
    NotMatter(EntityId),
    NotAPlace(EntityId),
    NotAChamber(EntityId),
    NotEnoughHeat(EntityId),
    NotEnoughMaterial {
        entity: EntityId,
        material: MaterialId,
    },
    DoesNotBurn(MaterialId),
    WouldEmpty(EntityId),
    CannotMerge(EntityId),
    UnknownShape(String),
    UnknownDesign(String),
    NotAPart(EntityId),
    NotAnAssembly(EntityId),
    NotConserved(&'static str),
    Invariant(String),
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::UnknownEntity(id) => write!(f, "no entity {id:?}"),
            Fault::NotLocated(id) => write!(f, "{id:?} isn't anywhere, so it can't move"),
            Fault::IntoItself(id) => write!(f, "{id:?} can't be put inside itself"),
            Fault::NoWallet(id) => write!(f, "{id:?} has no wallet"),
            Fault::SameWallet(id) => write!(f, "{id:?} can't pay itself"),
            Fault::Insufficient { from, amount } => write!(f, "{from:?} doesn't have {amount}"),
            Fault::Overflow(id) => write!(f, "{id:?} can't hold that much more"),
            Fault::NotMatter(id) => write!(f, "{id:?} isn't made of anything"),
            Fault::NotAPlace(id) => write!(f, "{id:?} isn't a place"),
            Fault::NotAChamber(id) => write!(f, "{id:?} isn't a chamber"),
            Fault::NotEnoughHeat(id) => write!(f, "{id:?} doesn't hold that much heat"),
            Fault::NotEnoughMaterial { entity, material } => {
                write!(f, "{entity:?} doesn't have that much of {material:?}")
            }
            Fault::DoesNotBurn(material) => write!(f, "{material:?} doesn't burn"),
            Fault::WouldEmpty(id) => write!(f, "{id:?} would be left with nothing"),
            Fault::CannotMerge(id) => write!(f, "{id:?} can't be merged away"),
            Fault::UnknownShape(shape) => write!(f, "no shape called {shape:?}"),
            Fault::UnknownDesign(design) => write!(f, "no design called {design:?}"),
            Fault::NotAPart(id) => write!(f, "{id:?} can't be a part"),
            Fault::NotAnAssembly(id) => write!(f, "{id:?} isn't an assembly"),
            Fault::NotConserved(what) => write!(f, "total {what} would change"),
            Fault::Invariant(why) => write!(f, "the world would be broken: {why}"),
        }
    }
}

impl std::error::Error for Fault {}

impl World {
    /// Applies `changes`, all together or not at all. On success they're
    /// logged. On any fault, the world is left exactly as it was.
    pub fn apply(&mut self, cause: Cause, changes: Vec<Change>) -> Result<(), Fault> {
        let (mass, energy, credits) =
            (self.total_mass(), self.total_energy(), self.total_credits());

        // Keep a copy to restore if anything goes wrong. The log is set
        // aside so it isn't copied every time. When worlds grow large, this
        // should become a record of how to undo each change.
        let log = std::mem::take(&mut self.log);
        let before = self.clone();
        self.log = log;

        let result = changes
            .iter()
            .try_for_each(|change| self.apply_one(change))
            .and_then(|()| {
                if self.total_mass() != mass {
                    Err(Fault::NotConserved("mass"))
                } else if self.total_energy() != energy {
                    Err(Fault::NotConserved("energy"))
                } else if self.total_credits() != credits {
                    Err(Fault::NotConserved("credits"))
                } else {
                    self.check_invariants().map_err(Fault::Invariant)
                }
            });

        if let Err(fault) = result {
            let log = std::mem::take(&mut self.log);
            *self = before;
            self.log = log;
            return Err(fault);
        }
        let seq = self.log.len() as u64;
        self.log.push(LogEntry {
            seq,
            cause,
            changes,
        });
        Ok(())
    }

    /// Moves the world's clock on by one tick. Nature calls this once the
    /// tick's changes have passed the gate.
    pub(crate) fn advance_clock(&mut self) {
        self.tick += 1;
    }

    fn apply_one(&mut self, change: &Change) -> Result<(), Fault> {
        match change {
            &Change::Move { entity, to } => {
                self.must_exist(entity)?;
                self.must_exist(to)?;
                if entity == to {
                    return Err(Fault::IntoItself(entity));
                }
                if self.location(entity).is_none() {
                    return Err(Fault::NotLocated(entity));
                }
                self.locations.insert(entity, to);
                Ok(())
            }

            &Change::Transfer { from, to, amount } => {
                if from == to {
                    return Err(Fault::SameWallet(from));
                }
                let payer = self.wallet(from).ok_or(Fault::NoWallet(from))?;
                let payee = self.wallet(to).ok_or(Fault::NoWallet(to))?;
                let payer = payer
                    .checked_sub(amount)
                    .ok_or(Fault::Insufficient { from, amount })?;
                let payee = payee.checked_add(amount).ok_or(Fault::Overflow(to))?;
                self.wallets.insert(from, payer);
                self.wallets.insert(to, payee);
                Ok(())
            }

            &Change::Heat { from, to, amount } => {
                self.take_heat(from, amount)?;
                self.give_heat(to, amount)
            }

            &Change::Burn {
                entity,
                material,
                mass,
            } => {
                let fuel = self
                    .materials
                    .get(&material)
                    .ok_or(Fault::DoesNotBurn(material))?;
                if !fuel.burns() {
                    return Err(Fault::DoesNotBurn(material));
                }
                let products = matter::split_by_fractions(mass, &fuel.burns_to);
                let fuel_energy = u128::from(mass.mg()) * u128::from(fuel.energy_density);
                let left_energy: u128 = products
                    .iter()
                    .map(|(m, part)| {
                        u128::from(part.mg()) * u128::from(self.materials[m].energy_density)
                    })
                    .sum();
                let released = fuel_energy
                    .checked_sub(left_energy)
                    .and_then(|e| u64::try_from(e).ok())
                    .ok_or(Fault::DoesNotBurn(material))?;

                let composition = self
                    .matter
                    .get_mut(&entity)
                    .ok_or(Fault::NotMatter(entity))?;
                remove_material(composition, material, mass)
                    .ok_or(Fault::NotEnoughMaterial { entity, material })?;
                for (product, part) in products {
                    add_material(composition, product, part).ok_or(Fault::Overflow(entity))?;
                }
                if composition.is_empty() {
                    return Err(Fault::WouldEmpty(entity));
                }
                self.give_heat(Holder::Thing(entity), Energy::from_uj(released))
            }

            Change::Split { from, take, at } => {
                let (from, at) = (*from, *at);
                self.must_exist(at)?;
                let source = self.matter.get(&from).ok_or(Fault::NotMatter(from))?;
                let capacity_before = matter::heat_capacity(&self.materials, source);
                let mut rest = source.clone();
                for (&material, &mass) in take {
                    remove_material(&mut rest, material, mass).ok_or(Fault::NotEnoughMaterial {
                        entity: from,
                        material,
                    })?;
                }
                if rest.is_empty() || take.values().all(|m| *m == Mass::ZERO) {
                    return Err(Fault::WouldEmpty(from));
                }
                let heat = self.heat(from).ok_or(Fault::NotMatter(from))?;
                let taken_heat = if capacity_before == 0 {
                    0
                } else {
                    u128::from(heat.uj()) * matter::heat_capacity(&self.materials, take)
                        / capacity_before
                };
                let taken_heat =
                    Energy::from_uj(u64::try_from(taken_heat).expect("part of the heat fits"));

                let mut piece: Composition = take.clone();
                piece.retain(|_, m| *m != Mass::ZERO);
                self.matter.insert(from, rest);
                self.heat.insert(
                    from,
                    heat.checked_sub(taken_heat).expect("part of the heat"),
                );
                let new = self.spawn(None, None);
                self.matter.insert(new, piece);
                self.heat.insert(new, taken_heat);
                self.locations.insert(new, at);
                self.portable.insert(new);
                Ok(())
            }

            &Change::Merge { from, into } => {
                if from == into {
                    return Err(Fault::IntoItself(from));
                }
                let mergeable = self.matter.contains_key(&from)
                    && !self.is_place(from)
                    && !self.is_agent(from)
                    && !self.is_container(from)
                    && self.wallet(from).is_none()
                    && self.contents(from).is_empty();
                if !mergeable {
                    return Err(Fault::CannotMerge(from));
                }
                if !self.matter.contains_key(&into) {
                    return Err(Fault::NotMatter(into));
                }
                let composition = self.matter.remove(&from).expect("checked above");
                let heat = self.heat.remove(&from).expect("matter has heat");
                let target = self.matter.get_mut(&into).expect("checked above");
                for (material, mass) in composition {
                    add_material(target, material, mass).ok_or(Fault::Overflow(into))?;
                }
                self.give_heat(Holder::Thing(into), heat)?;
                for components in [&mut self.keys, &mut self.labels, &mut self.shape_of] {
                    components.remove(&from);
                }
                self.locations.remove(&from);
                self.portable.remove(&from);
                self.forms.remove(&from);
                self.tolerance.remove(&from);
                Ok(())
            }

            Change::Shape { entity, shape } => {
                if !self.matter.contains_key(entity) {
                    return Err(Fault::NotMatter(*entity));
                }
                match shape {
                    Some((shape, _)) if !self.shapes.contains_key(shape) => {
                        Err(Fault::UnknownShape(shape.clone()))
                    }
                    Some((shape, tolerance)) => {
                        self.shape_of.insert(*entity, shape.clone());
                        self.tolerance.insert(*entity, *tolerance);
                        Ok(())
                    }
                    None => {
                        self.shape_of.remove(entity);
                        self.tolerance.remove(entity);
                        Ok(())
                    }
                }
            }

            &Change::Refine { entity, tolerance } => {
                let current = self
                    .tolerance
                    .get_mut(&entity)
                    .ok_or(Fault::NotAPart(entity))?;
                *current = tolerance;
                Ok(())
            }

            Change::Assemble {
                design,
                parts,
                at,
                datasheet,
            } => {
                if !self.designs.contains_key(design) {
                    return Err(Fault::UnknownDesign(design.clone()));
                }
                self.must_exist(*at)?;
                let mut seen = std::collections::BTreeSet::new();
                for &part in parts {
                    let fits = self.exists(part)
                        && seen.insert(part)
                        && self.is_portable(part)
                        && !self.is_agent(part)
                        && !self.is_place(part)
                        && !self.is_container(part)
                        && self.location(part).is_some();
                    if !fits {
                        return Err(Fault::NotAPart(part));
                    }
                }
                let new = self.spawn(None, None);
                self.assemblies.insert(
                    new,
                    crate::world::Assembly {
                        design: design.clone(),
                        datasheet: datasheet.clone(),
                    },
                );
                self.portable.insert(new);
                self.locations.insert(new, *at);
                for &part in parts {
                    self.locations.insert(part, new);
                }
                Ok(())
            }

            &Change::Disassemble { assembly } => {
                if !self.assemblies.contains_key(&assembly) {
                    return Err(Fault::NotAnAssembly(assembly));
                }
                let at = self.location(assembly).ok_or(Fault::NotLocated(assembly))?;
                for part in self.contents(assembly) {
                    self.locations.insert(part, at);
                }
                self.assemblies.remove(&assembly);
                for components in [&mut self.keys, &mut self.labels] {
                    components.remove(&assembly);
                }
                self.locations.remove(&assembly);
                self.portable.remove(&assembly);
                Ok(())
            }

            &Change::Light { chamber, lit } => {
                let chamber = self
                    .chambers
                    .get_mut(&chamber)
                    .ok_or(Fault::NotAChamber(chamber))?;
                chamber.lit = lit;
                Ok(())
            }
        }
    }

    fn must_exist(&self, id: EntityId) -> Result<(), Fault> {
        if self.exists(id) {
            Ok(())
        } else {
            Err(Fault::UnknownEntity(id))
        }
    }

    fn take_heat(&mut self, from: Holder, amount: Energy) -> Result<(), Fault> {
        let (store, id) = match from {
            Holder::Thing(id) => (self.heat.get_mut(&id).ok_or(Fault::NotMatter(id))?, id),
            Holder::Surroundings(id) => {
                if !self.is_place(id) {
                    return Err(Fault::NotAPlace(id));
                }
                (self.surroundings.entry(id).or_default(), id)
            }
        };
        *store = store.checked_sub(amount).ok_or(Fault::NotEnoughHeat(id))?;
        Ok(())
    }

    fn give_heat(&mut self, to: Holder, amount: Energy) -> Result<(), Fault> {
        let (store, id) = match to {
            Holder::Thing(id) => (self.heat.get_mut(&id).ok_or(Fault::NotMatter(id))?, id),
            Holder::Surroundings(id) => {
                if !self.is_place(id) {
                    return Err(Fault::NotAPlace(id));
                }
                (self.surroundings.entry(id).or_default(), id)
            }
        };
        *store = store.checked_add(amount).ok_or(Fault::Overflow(id))?;
        Ok(())
    }
}

fn remove_material(composition: &mut Composition, material: MaterialId, mass: Mass) -> Option<()> {
    let have = composition.get_mut(&material)?;
    *have = have.checked_sub(mass)?;
    if *have == Mass::ZERO {
        composition.remove(&material);
    }
    Some(())
}

fn add_material(composition: &mut Composition, material: MaterialId, mass: Mass) -> Option<()> {
    if mass == Mass::ZERO {
        return Some(());
    }
    let have = composition.entry(material).or_default();
    *have = have.checked_add(mass)?;
    Some(())
}
