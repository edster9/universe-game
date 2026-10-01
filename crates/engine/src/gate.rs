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
    /// Move `entity` to `at` (µm east and north) within the place it's in.
    Spot { entity: EntityId, at: (i64, i64) },
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
    /// Something colder than the air around it warms by `amount`. The heat
    /// comes from what the place's surroundings have taken in and, beyond
    /// that, from sunlight, which is what warms the air by day.
    Warm {
        entity: EntityId,
        place: EntityId,
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
    /// Move part of one piece of matter into another, with its share of the
    /// heat: eating, drinking.
    Shift {
        from: EntityId,
        to: EntityId,
        take: Composition,
    },
    /// Let part of a piece of matter pass into a place's surroundings, with
    /// its share of the heat: breath, sweat, gas clearing from the air.
    Release {
        from: EntityId,
        take: Composition,
        place: EntityId,
    },
    /// `mass` of one material in a piece turns into another, as some earths
    /// fire hard. Their stored energy must match.
    Transform {
        entity: EntityId,
        from: MaterialId,
        to: MaterialId,
        mass: Mass,
    },
    /// A body works hard until `until`.
    Exert { agent: EntityId, until: u64 },
    /// A body stores `mass` of a food it has digested as its reserve: a share
    /// of the food's energy (`efficiency`, in parts per ten thousand) becomes
    /// the reserve, the rest is released as heat, and what's left of the
    /// food's mass becomes what it burns to.
    Store {
        entity: EntityId,
        from: MaterialId,
        mass: Mass,
        into: MaterialId,
        efficiency: u64,
    },
    /// A creature acting on instinct is busy until `until`.
    Occupy { agent: EntityId, until: u64 },
    /// A creature keeps away from a place until `until`.
    Avoid {
        agent: EntityId,
        place: EntityId,
        until: u64,
    },
    /// Someone is told or reads something, from `source`. Until they see it,
    /// it's only possible.
    Hear {
        agent: EntityId,
        claim: crate::world::Claim,
        source: String,
    },
    /// Someone sees what's at a place: its fixed things and creatures.
    /// `gone` is what they remembered there that isn't any more, which they
    /// correct.
    Sight {
        agent: EntityId,
        place: EntityId,
        things: std::collections::BTreeSet<EntityId>,
        gone: std::collections::BTreeSet<EntityId>,
    },
    /// Someone sees for themselves whether something they were told, by
    /// `source`, is true.
    Settle {
        agent: EntityId,
        claim: crate::world::Claim,
        held: bool,
        source: String,
    },
    /// Someone sees a place, from afar or by being there.
    See { agent: EntityId, place: EntityId },
    /// Someone learns the way from one place to another.
    Learn {
        agent: EntityId,
        from: EntityId,
        to: EntityId,
    },
    /// A body is wounded, and bleeds at `rate` mg a second, less as it clots.
    /// A wound wakes a sleeper.
    Wound { agent: EntityId, rate: u64 },
    /// Someone with a mind of their own takes on a request, to do when
    /// they're free.
    Request { agent: EntityId, intent: Intent },
    /// They take up the first request they had, doing it or finding they
    /// can't.
    TakeUp { agent: EntityId },
    /// Someone remembers going for `at`.
    Struck { agent: EntityId, at: EntityId },
    /// Someone notices something that happened to them, and remembers it
    /// until they hear of it.
    Notice {
        agent: EntityId,
        news: crate::world::News,
    },
    /// A body sleeps until `until`, and wakes less tired, in a shelter if it
    /// has one.
    Sleep {
        agent: EntityId,
        until: u64,
        shelter: Option<EntityId>,
    },
    /// A body dies. It stays where it is, as matter.
    Die { agent: EntityId, cause: String },
    /// Someone starts something they keep doing over time.
    StartActivity {
        agent: EntityId,
        activity: crate::world::Activity,
    },
    /// Someone stops what they were doing.
    EndActivity { agent: EntityId },
    /// Something alive grows by `mass`, turning matter taken from `from` into
    /// more of itself. The chemical energy it gains comes from sunlight.
    Grow {
        entity: EntityId,
        from: EntityId,
        mass: Mass,
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
    /// Put `parts` together to `design`, or to none, as something new, as a
    /// new thing at `at`, with the datasheet measured for it.
    Assemble {
        design: Option<String>,
        parts: Vec<EntityId>,
        at: EntityId,
        datasheet: Datasheet,
    },
    /// Take an assembly apart. Its parts are left where it was, and it stops
    /// existing.
    Disassemble { assembly: EntityId },
    /// Light or put out a chamber.
    Light { chamber: EntityId, lit: bool },
    /// Someone learns a word, or another meaning for one they know. See
    /// docs/ideas/vocabulary.md.
    Word {
        agent: EntityId,
        word: String,
        meaning: crate::words::Meaning,
    },
    /// Someone learns a way to make what a word names.
    Recipe {
        agent: EntityId,
        word: String,
        recipe: crate::words::Recipe,
    },
    /// Someone has made something, which "it" now means to them.
    Made { agent: EntityId, thing: EntityId },
    /// A player's body takes in vitality, a named inflow: `inflow` µJ of
    /// energy, of which `stored` tops up its stamina and the rest warms it;
    /// `drawn` µJ of stamina also warms it, for hard work; and `fluid` of its
    /// body's fluid, lost to sweat or bleeding, comes back at its
    /// temperature.
    Vitality {
        entity: EntityId,
        inflow: u64,
        stored: u64,
        drawn: u64,
        fluid: Mass,
    },
    /// Someone starts an action that takes time. It's carried out when it's
    /// due, at `until`; meanwhile they're busy.
    Begin {
        agent: EntityId,
        intent: Intent,
        until: u64,
    },
    /// Someone's action in progress is over, and how it came out.
    End {
        agent: EntityId,
        outcome: crate::world::Outcome,
    },
    /// Someone starts wearing something they carry, or, with `None`, stops.
    Wear {
        agent: EntityId,
        item: EntityId,
        worn: Option<crate::world::Worn>,
    },
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
    /// Somewhere beyond the edge of the place it's in.
    OutsidePlace(EntityId),
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
    NotAlive(EntityId),
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
            Fault::OutsidePlace(id) => write!(f, "{id:?} would be beyond the edge of its place"),
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
            Fault::NotAlive(id) => write!(f, "{id:?} isn't alive"),
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
    ///
    /// Every table notes what it held before each change, so a refused set is
    /// undone from those notes, and conservation and the world's structure
    /// are checked only where something changed. Builds with debug checks
    /// (every test run) also check some sets the long way, and undo them to
    /// prove the notes restore the world exactly.
    pub fn apply(&mut self, cause: Cause, changes: Vec<Change>) -> Result<(), Fault> {
        // One set in eight is audited: enough to catch a table that isn't
        // recording across thousands of sets, without slowing every proof.
        #[cfg(debug_assertions)]
        let audit = self.logged.is_multiple_of(8).then(|| self.audit_start());
        let totals = Totals::of(self);
        self.journals(|j| j.begin());

        let applied = changes.iter().try_for_each(|change| self.apply_one(change));
        #[cfg(debug_assertions)]
        let every_change_applied = applied.is_ok();
        let result = applied
            .map(|()| {
                // What's no longer on someone isn't worn any more.
                let worn: Vec<EntityId> = self.worn.keys().copied().collect();
                for item in worn {
                    let on_someone =
                        self.exists(item) && self.location(item).is_some_and(|l| self.is_agent(l));
                    if !on_someone {
                        self.worn.remove(&item);
                    }
                }
            })
            .and_then(|()| self.check_changes(&totals));

        #[cfg(debug_assertions)]
        if let Some(audit) = audit {
            self.audit_end(audit, &totals, &result, every_change_applied);
        }

        if let Err(fault) = result {
            self.journals(|j| j.undo());
            totals.restore(self);
            return Err(fault);
        }
        self.journals(|j| j.commit());
        self.log.push(LogEntry {
            seq: self.logged,
            cause,
            changes,
        });
        self.logged += 1;
        // Forget the oldest, a batch at a time.
        if self.log.len() >= self.log_window.saturating_mul(2).max(1) {
            let forget = self.log.len() - self.log_window;
            self.log.drain(..forget);
        }
        Ok(())
    }

    /// Checks what a set of changes touched: mass, energy, and credits
    /// conserved (less what entered from named sources), and the world still
    /// well formed around everything that changed.
    fn check_changes(&self, totals: &Totals) -> Result<(), Fault> {
        let (mass, energy, credits) = self.change_in_totals();
        if mass != self.vital_matter as i128 - totals.vital_matter as i128 {
            return Err(Fault::NotConserved("mass"));
        }
        let entered = (self.sunlight + self.vital_energy) as i128
            - (totals.sunlight + totals.vital_energy) as i128;
        if energy != entered {
            return Err(Fault::NotConserved("energy"));
        }
        if credits != 0 {
            return Err(Fault::NotConserved("credits"));
        }
        self.check_touched().map_err(Fault::Invariant)
    }

    /// How much total mass, energy, and credits have changed since the
    /// tables began recording, counting only the entries that changed.
    fn change_in_totals(&self) -> (i128, i128, i128) {
        fn diff<K: Ord + Clone, V: Clone>(
            table: &crate::journal::Table<K, V>,
            measure: impl Fn(&V) -> u128,
        ) -> i128 {
            table
                .originals()
                .into_iter()
                .map(|(key, old)| {
                    table.get(key).map_or(0, &measure) as i128 - old.map_or(0, &measure) as i128
                })
                .sum()
        }
        let materials = &self.materials;
        let mass = diff(&self.masses, |m| u128::from(m.mg()))
            + diff(&self.matter, matter::total_mass)
            + diff(&self.reservoir, matter::total_mass);
        let energy = diff(&self.heat, |e| u128::from(e.uj()))
            + diff(&self.surroundings, |e| u128::from(e.uj()))
            + diff(&self.matter, |c| matter::chemical_energy(materials, c))
            + diff(&self.reservoir, |c| matter::chemical_energy(materials, c))
            + diff(&self.life, |l| {
                l.vitality.as_ref().map_or(0, |v| u128::from(v.stamina))
            });
        let credits = diff(&self.wallets, |c| u128::from(c.amount()));
        (mass, energy, credits)
    }

    #[cfg(debug_assertions)]
    fn audit_start(&mut self) -> Audit {
        let log = std::mem::take(&mut self.log);
        let before = self.clone();
        self.log = log;
        Audit {
            before,
            mass: self.own_mass(),
            energy: self.own_energy(),
            credits: self.total_credits(),
        }
    }

    /// The long way: the whole world weighed and checked, compared with the
    /// quick check if every change could be made; then the set undone from the tables' notes, which must
    /// give back exactly the world before it.
    #[cfg(debug_assertions)]
    fn audit_end(
        &mut self,
        audit: Audit,
        totals: &Totals,
        quick: &Result<(), Fault>,
        every_change_applied: bool,
    ) {
        let full = if self.own_mass() != audit.mass {
            Err(Fault::NotConserved("mass"))
        } else if self.own_energy() != audit.energy {
            Err(Fault::NotConserved("energy"))
        } else if self.total_credits() != audit.credits {
            Err(Fault::NotConserved("credits"))
        } else {
            self.check_invariants().map_err(Fault::Invariant)
        };
        // A change that couldn't be made at all leaves the world half done;
        // only a complete set is compared.
        match (quick, &full) {
            _ if !every_change_applied => {}
            (Ok(()), Err(fault)) => panic!("the gate's quick check missed: {fault}"),
            (Err(fault), Ok(())) => {
                panic!("the gate's quick check refused what the whole world allows: {fault}")
            }
            _ => {}
        }
        let log = std::mem::take(&mut self.log);
        let after = self.clone();
        self.journals(|j| j.undo());
        totals.restore(self);
        assert!(
            *self == audit.before,
            "undoing a set of changes didn't give back the world before it"
        );
        *self = after;
        self.log = log;
    }

    /// Moves the world's clock on. Nature calls this once a step's changes
    /// have passed the gate.
    pub(crate) fn advance_clock(&mut self, seconds: u64) {
        self.tick += seconds;
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
                self.put(entity, to);
                Ok(())
            }

            &Change::Spot { entity, at } => {
                self.must_exist(entity)?;
                let place = self
                    .location(entity)
                    .filter(|&p| self.is_place(p))
                    .ok_or(Fault::NotAPlace(entity))?;
                let middle = self.position(place).unwrap_or((0, 0));
                if crate::world::distance(middle, at) > self.size(place) {
                    return Err(Fault::OutsidePlace(entity));
                }
                self.spots.insert(entity, at);
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

            &Change::Warm {
                entity,
                place,
                amount,
            } => {
                if !self.is_place(place) {
                    return Err(Fault::NotAPlace(place));
                }
                let air = self.surroundings.or_insert_with(place, Default::default);
                let from_air = (*air).min(amount);
                *air = air.checked_sub(from_air).expect("no more than it has");
                self.sunlight += u128::from(amount.uj() - from_air.uj());
                self.give_heat(Holder::Thing(entity), amount)
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

            &Change::Store {
                entity,
                from,
                mass,
                into,
                efficiency,
            } => {
                let (Some(food), Some(reserve)) =
                    (self.materials.get(&from), self.materials.get(&into))
                else {
                    return Err(Fault::DoesNotBurn(from));
                };
                if !food.burns() || reserve.energy_density == 0 || efficiency > 10_000 {
                    return Err(Fault::DoesNotBurn(from));
                }
                let energy = u128::from(mass.mg()) * u128::from(food.energy_density);
                let stored = energy * u128::from(efficiency) / 10_000;
                let into_mass = u64::try_from(stored / u128::from(reserve.energy_density))
                    .expect("less than the food's mass")
                    .min(mass.mg());
                let products = matter::split_by_fractions(
                    Mass::from_mg(mass.mg() - into_mass),
                    &food.burns_to,
                );
                let left: u128 = u128::from(into_mass) * u128::from(reserve.energy_density)
                    + products
                        .iter()
                        .map(|(m, part)| {
                            u128::from(part.mg()) * u128::from(self.materials[m].energy_density)
                        })
                        .sum::<u128>();
                let released = energy
                    .checked_sub(left)
                    .and_then(|e| u64::try_from(e).ok())
                    .ok_or(Fault::DoesNotBurn(from))?;
                let composition = self
                    .matter
                    .get_mut(&entity)
                    .ok_or(Fault::NotMatter(entity))?;
                remove_material(composition, from, mass).ok_or(Fault::NotEnoughMaterial {
                    entity,
                    material: from,
                })?;
                add_material(composition, into, Mass::from_mg(into_mass))
                    .ok_or(Fault::Overflow(entity))?;
                for (product, part) in products {
                    add_material(composition, product, part).ok_or(Fault::Overflow(entity))?;
                }
                composition.retain(|_, m| m.mg() > 0);
                self.give_heat(Holder::Thing(entity), Energy::from_uj(released))
            }

            Change::Split { from, take, at } => {
                self.must_exist(*at)?;
                let (piece, heat) = self.take_part(*from, take)?;
                let new = self.spawn(None, None);
                self.matter.insert(new, piece);
                self.heat.insert(new, heat);
                self.put(new, *at);
                self.portable.insert(new);
                Ok(())
            }

            Change::Shift { from, to, take } => {
                if from == to {
                    return Err(Fault::IntoItself(*from));
                }
                if !self.matter.contains_key(to) {
                    return Err(Fault::NotMatter(*to));
                }
                let (piece, heat) = self.take_part(*from, take)?;
                let target = self.matter.get_mut(to).expect("checked above");
                for (material, mass) in piece {
                    add_material(target, material, mass).ok_or(Fault::Overflow(*to))?;
                }
                self.give_heat(Holder::Thing(*to), heat)
            }

            Change::Release { from, take, place } => {
                if !self.is_place(*place) {
                    return Err(Fault::NotAPlace(*place));
                }
                let (piece, heat) = self.take_part(*from, take)?;
                let reservoir = self.reservoir.or_insert_with(*place, Default::default);
                for (material, mass) in piece {
                    add_material(reservoir, material, mass).ok_or(Fault::Overflow(*place))?;
                }
                self.give_heat(Holder::Surroundings(*place), heat)
            }

            Change::StartActivity { agent, activity } => {
                if !self.is_agent(*agent) {
                    return Err(Fault::NotAlive(*agent));
                }
                let crate::world::Activity::Rubbing {
                    first,
                    second,
                    dust,
                    ..
                } = activity;
                for &id in [first, second, dust] {
                    if !self.matter.contains_key(&id) {
                        return Err(Fault::NotMatter(id));
                    }
                }
                self.activities.insert(*agent, activity.clone());
                Ok(())
            }

            &Change::EndActivity { agent } => {
                self.activities
                    .remove(&agent)
                    .ok_or(Fault::NotAlive(agent))?;
                // Stopping what they were doing ends the hard work too.
                let now = self.tick;
                if let Some(life) = self.life.get_mut(&agent) {
                    life.working_until = life.working_until.min(now);
                }
                Ok(())
            }

            &Change::Grow { entity, from, mass } => {
                let source = self.matter.get(&from).ok_or(Fault::NotMatter(from))?;
                let own = self.matter.get(&entity).ok_or(Fault::NotMatter(entity))?;
                let taken = matter::proportional(source, mass).ok_or(Fault::WouldEmpty(from))?;
                // What grows is more of what the grower is already
                // made of, in the same shares.
                let grown = scale(own, mass).ok_or(Fault::WouldEmpty(entity))?;
                let before = matter::chemical_energy(&self.materials, &taken);
                let after = matter::chemical_energy(&self.materials, &grown);
                let from_sun = after
                    .checked_sub(before)
                    .ok_or(Fault::NotConserved("energy"))?;
                let (_, heat) = self.take_part(from, &taken)?;
                let body = self.matter.get_mut(&entity).expect("checked above");
                for (material, part) in grown {
                    add_material(body, material, part).ok_or(Fault::Overflow(entity))?;
                }
                self.give_heat(Holder::Thing(entity), heat)?;
                self.sunlight += from_sun;
                Ok(())
            }

            &Change::Transform {
                entity,
                from,
                to,
                mass,
            } => {
                let (a, b) = (&self.materials[&from], &self.materials[&to]);
                if a.energy_density != b.energy_density {
                    return Err(Fault::NotConserved("energy"));
                }
                let composition = self
                    .matter
                    .get_mut(&entity)
                    .ok_or(Fault::NotMatter(entity))?;
                remove_material(composition, from, mass).ok_or(Fault::NotEnoughMaterial {
                    entity,
                    material: from,
                })?;
                add_material(composition, to, mass).ok_or(Fault::Overflow(entity))
            }

            &Change::Exert { agent, until } => {
                let life = self.life.get_mut(&agent).ok_or(Fault::NotAlive(agent))?;
                life.working_until = until;
                Ok(())
            }

            Change::Request { agent, intent } => {
                let mind = self.minds.get_mut(agent).ok_or(Fault::NotAlive(*agent))?;
                mind.requests.push(intent.clone());
                Ok(())
            }

            &Change::TakeUp { agent } => {
                let mind = self.minds.get_mut(&agent).ok_or(Fault::NotAlive(agent))?;
                if mind.requests.is_empty() {
                    return Err(Fault::Invariant(format!("{agent:?} had no request")));
                }
                mind.requests.remove(0);
                Ok(())
            }

            &Change::Struck { agent, at } => {
                let now = self.tick;
                let memory = self
                    .memories
                    .get_mut(&agent)
                    .ok_or(Fault::NotAlive(agent))?;
                memory.struck.insert(at, now);
                Ok(())
            }

            Change::Notice { agent, news } => {
                self.must_exist(*agent)?;
                let now = self.tick;
                if let Some(memory) = self.memories.get_mut(agent) {
                    match *news {
                        crate::world::News::Attacked { by, .. } => {
                            memory.attackers.insert(by, now);
                        }
                        crate::world::News::Took { by, .. } => {
                            memory.robbed_by.insert(by, now);
                        }
                    }
                }
                // Someone nobody plays notices at once; the rest hear of it.
                if !self.minds.contains_key(agent) {
                    self.news
                        .or_insert_with(*agent, Vec::new)
                        .push(news.clone());
                }
                Ok(())
            }

            &Change::Wound { agent, rate } => {
                let now = self.tick;
                let life = self.life.get_mut(&agent).ok_or(Fault::NotAlive(agent))?;
                if life.died_of.is_some() {
                    return Err(Fault::NotAlive(agent));
                }
                // Forget wounds that have stopped bleeding.
                let clots = life.clots.max(1);
                life.wounds.retain(|&(r, since)| {
                    u32::try_from(now.saturating_sub(since) / clots)
                        .ok()
                        .and_then(|k| r.checked_shr(k))
                        .is_some_and(|left| left > 0)
                });
                life.wounds.push((rate, now));
                // A wound interrupts whatever it was doing.
                self.busy_until.remove(&agent);
                if self.pending.remove(&agent).is_some() {
                    self.outcomes
                        .insert(agent, crate::world::Outcome::Interrupted);
                }
                let life = self.life.get_mut(&agent).expect("checked above");
                // A wound wakes a sleeper, who has slept only until now.
                if let Some(sleep) = life.sleep.as_mut()
                    && sleep.until > now
                {
                    let unslept = u128::from(sleep.until - now) * u128::from(sleep.awake)
                        / u128::from(sleep.need.max(1));
                    sleep.debt += u64::try_from(unslept).unwrap_or(u64::MAX);
                    sleep.since = now;
                    sleep.until = now;
                }
                Ok(())
            }

            &Change::Occupy { agent, until } => {
                if !self.is_agent(agent) {
                    return Err(Fault::NotAlive(agent));
                }
                self.busy_until.insert(agent, until);
                Ok(())
            }

            &Change::Avoid {
                agent,
                place,
                until,
            } => {
                if !self.is_agent(agent) {
                    return Err(Fault::NotAlive(agent));
                }
                if !self.is_place(place) {
                    return Err(Fault::NotAPlace(place));
                }
                self.avoiding
                    .or_insert_with(agent, Default::default)
                    .insert(place, until);
                Ok(())
            }

            &Change::See { agent, place } => {
                if !self.is_place(place) {
                    return Err(Fault::NotAPlace(place));
                }
                if let Some(memory) = self.memories.get_mut(&agent) {
                    memory.places.insert(place);
                    if memory
                        .possible
                        .remove(&crate::world::Claim::Place(place))
                        .is_some()
                    {
                        memory.confirmed += 1;
                    }
                }
                Ok(())
            }

            Change::Hear {
                agent,
                claim,
                source,
            } => {
                let memory = self
                    .memories
                    .get_mut(agent)
                    .ok_or(Fault::NotAlive(*agent))?;
                let known = match claim {
                    crate::world::Claim::Place(p) => memory.places.contains(p),
                    crate::world::Claim::Way(a, b) => memory.ways.contains(&(*a, *b)),
                    crate::world::Claim::Thing(..) => false,
                };
                if !known {
                    memory.possible.insert(claim.clone(), source.clone());
                }
                Ok(())
            }

            Change::Sight {
                agent,
                place,
                things,
                gone,
            } => {
                let now = self.tick;
                let memory = self
                    .memories
                    .get_mut(agent)
                    .ok_or(Fault::NotAlive(*agent))?;
                memory.corrected += u64::try_from(gone.len()).expect("a count");
                memory.sightings.insert(*place, (now, things.clone()));
                Ok(())
            }

            Change::Settle {
                agent, claim, held, ..
            } => {
                let memory = self
                    .memories
                    .get_mut(agent)
                    .ok_or(Fault::NotAlive(*agent))?;
                if memory.possible.remove(claim).is_some() {
                    if *held {
                        memory.confirmed += 1;
                    } else {
                        memory.corrected += 1;
                    }
                }
                Ok(())
            }

            Change::Word {
                agent,
                word,
                meaning,
            } => {
                let lexicon = self
                    .lexicons
                    .get_mut(agent)
                    .ok_or(Fault::UnknownEntity(*agent))?;
                let entry = (word.clone(), meaning.clone());
                if !lexicon.words.contains(&entry) {
                    lexicon.words.push(entry);
                }
                Ok(())
            }
            Change::Recipe {
                agent,
                word,
                recipe,
            } => {
                let lexicon = self
                    .lexicons
                    .get_mut(agent)
                    .ok_or(Fault::UnknownEntity(*agent))?;
                let entry = (word.clone(), recipe.clone());
                if !lexicon.recipes.contains(&entry) {
                    lexicon.recipes.push(entry);
                }
                Ok(())
            }
            &Change::Vitality {
                entity,
                inflow,
                stored,
                drawn,
                fluid,
            } => {
                let temperature = self.temperature(entity).ok_or(Fault::NotMatter(entity))?;
                let life = self.life.get_mut(&entity).ok_or(Fault::NotAlive(entity))?;
                let fluid_material = life.fluid;
                let vitality = life.vitality.as_mut().ok_or(Fault::NotAlive(entity))?;
                if stored > inflow || drawn > vitality.stamina {
                    return Err(Fault::NotEnoughHeat(entity));
                }
                vitality.stamina = vitality.stamina - drawn + stored;
                if vitality.stamina > vitality.most {
                    return Err(Fault::Overflow(entity));
                }
                // Fluid comes in at the body's temperature, its warmth part
                // of the inflow.
                let capacity = u128::from(fluid.mg())
                    * u128::from(self.materials[&fluid_material].specific_heat);
                let fluid_heat = matter::energy_at(temperature, capacity);
                if fluid.mg() > 0 {
                    let body = self
                        .matter
                        .get_mut(&entity)
                        .ok_or(Fault::NotMatter(entity))?;
                    add_material(body, fluid_material, fluid).ok_or(Fault::Overflow(entity))?;
                }
                let warmth = u128::from(inflow - stored) + u128::from(drawn) + fluid_heat;
                let warmth = u64::try_from(warmth).map_err(|_| Fault::Overflow(entity))?;
                self.vital_energy += u128::from(inflow) + fluid_heat;
                self.vital_matter += u128::from(fluid.mg());
                self.give_heat(Holder::Thing(entity), Energy::from_uj(warmth))
            }
            Change::Begin {
                agent,
                intent,
                until,
            } => {
                if !self.is_agent(*agent) {
                    return Err(Fault::NotAlive(*agent));
                }
                if self.pending.contains_key(agent) {
                    return Err(Fault::Invariant(format!("{agent:?} is already busy")));
                }
                self.pending.insert(
                    *agent,
                    crate::world::Pending {
                        intent: intent.clone(),
                        since: self.tick,
                        until: *until,
                    },
                );
                Ok(())
            }
            Change::End { agent, outcome } => {
                self.pending
                    .remove(agent)
                    .ok_or_else(|| Fault::Invariant(format!("{agent:?} wasn't busy")))?;
                self.outcomes.insert(*agent, outcome.clone());
                Ok(())
            }
            Change::Wear { agent, item, worn } => {
                if self.location(*item) != Some(*agent) {
                    return Err(Fault::Invariant(format!(
                        "{agent:?} can only wear what they carry"
                    )));
                }
                match worn {
                    Some(worn) => self.worn.insert(*item, worn.clone()),
                    None => self.worn.remove(item),
                };
                Ok(())
            }
            &Change::Made { agent, thing } => {
                if !self.exists(thing) {
                    return Err(Fault::UnknownEntity(thing));
                }
                if let Some(lexicon) = self.lexicons.get_mut(&agent) {
                    lexicon.last_made = Some(thing);
                }
                Ok(())
            }
            &Change::Learn { agent, from, to } => {
                if !self.exits(from).contains(&to) {
                    return Err(Fault::NotAPlace(to));
                }
                if let Some(memory) = self.memories.get_mut(&agent) {
                    memory.ways.insert((from, to));
                    if memory
                        .possible
                        .remove(&crate::world::Claim::Way(from, to))
                        .is_some()
                    {
                        memory.confirmed += 1;
                    }
                }
                Ok(())
            }

            &Change::Sleep {
                agent,
                until,
                shelter,
            } => {
                let now = self.tick;
                let life = self.life.get_mut(&agent).ok_or(Fault::NotAlive(agent))?;
                let sleep = life.sleep.as_mut().ok_or(Fault::NotAlive(agent))?;
                // Sleep takes off wakefulness at the rate a full night's
                // sleep does: `awake` for every `need`.
                let awake = sleep.debt + now.saturating_sub(sleep.since);
                let rested = u128::from(until.saturating_sub(now)) * u128::from(sleep.awake)
                    / u128::from(sleep.need.max(1));
                sleep.debt = u64::try_from(u128::from(awake).saturating_sub(rested))
                    .expect("no more than it was");
                sleep.since = until;
                sleep.until = until;
                sleep.shelter = shelter;
                Ok(())
            }

            Change::Die { agent, cause } => {
                let life = self.life.get_mut(agent).ok_or(Fault::NotAlive(*agent))?;
                if life.died_of.is_some() {
                    return Err(Fault::NotAlive(*agent));
                }
                life.died_of = Some(cause.clone());
                self.agents.remove(agent);
                Ok(())
            }

            &Change::Merge { from, into } => {
                if from == into {
                    return Err(Fault::IntoItself(from));
                }
                let mergeable = self.matter.contains_key(&from)
                    && !self.is_place(from)
                    && !self.is_agent(from)
                    && !self.life.contains_key(&from)
                    && !self.in_use(from)
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
                self.unput(from);
                self.portable.remove(&from);
                self.forms.remove(&from);
                self.tolerance.remove(&from);
                self.pieces.remove(&from);
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
                        self.stop_casting(*entity);
                        self.shape_of.insert(*entity, shape.clone());
                        self.tolerance.insert(*entity, *tolerance);
                        // A shape whose role is to cast makes a form.
                        if let Some(casts) = self.shapes[shape].casts.clone() {
                            self.forms.insert(
                                *entity,
                                crate::world::Form {
                                    shape: casts,
                                    tolerance: *tolerance,
                                },
                            );
                            self.containers.insert(*entity);
                        }
                        // So does a shape whose role is to hold things.
                        if self.shapes[shape].capacity.is_some() {
                            self.containers.insert(*entity);
                        }
                        Ok(())
                    }
                    None => {
                        self.stop_casting(*entity);
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
                if let Some(design) = design
                    && !self.designs.contains_key(design)
                {
                    return Err(Fault::UnknownDesign(design.clone()));
                }
                let def = design.as_ref().map(|d| self.designs[d].clone());
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
                        parts: parts.clone(),
                        datasheet: datasheet.clone(),
                    },
                );
                self.portable.insert(new);
                if def.as_ref().is_some_and(|d| d.holds) {
                    self.containers.insert(new);
                }
                // A design that encloses heat makes a chamber.
                if let Some(chamber) = def.and_then(|d| d.chamber) {
                    self.chambers.insert(new, chamber);
                }
                self.put(new, *at);
                for &part in parts {
                    self.put(part, new);
                }
                Ok(())
            }

            &Change::Disassemble { assembly } => {
                if !self.assemblies.contains_key(&assembly) {
                    return Err(Fault::NotAnAssembly(assembly));
                }
                let at = self.location(assembly).ok_or(Fault::NotLocated(assembly))?;
                for part in self.contents(assembly) {
                    self.put(part, at);
                }
                self.assemblies.remove(&assembly);
                self.containers.remove(&assembly);
                self.chambers.remove(&assembly);
                for components in [&mut self.keys, &mut self.labels] {
                    components.remove(&assembly);
                }
                self.unput(assembly);
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

    /// Takes `take` out of the piece `from`, with heat in proportion to heat
    /// capacity so both parts keep the same temperature. `from` must keep
    /// something.
    fn take_part(
        &mut self,
        from: EntityId,
        take: &Composition,
    ) -> Result<(Composition, Energy), Fault> {
        let source = self.matter.get(&from).ok_or(Fault::NotMatter(from))?;
        let capacity_before = matter::heat_capacity(&self.materials, source);
        let mut rest = source.clone();
        for (&material, &mass) in take {
            remove_material(&mut rest, material, mass).ok_or(Fault::NotEnoughMaterial {
                entity: from,
                material,
            })?;
        }
        let mut piece: Composition = take.clone();
        piece.retain(|_, m| *m != Mass::ZERO);
        if rest.is_empty() || piece.is_empty() {
            return Err(Fault::WouldEmpty(from));
        }
        let heat = self.heat(from).ok_or(Fault::NotMatter(from))?;
        let taken = (u128::from(heat.uj()) * matter::heat_capacity(&self.materials, &piece))
            .checked_div(capacity_before)
            .unwrap_or(0);
        let taken = Energy::from_uj(u64::try_from(taken).expect("part of the heat fits"));
        self.matter.insert(from, rest);
        self.heat
            .insert(from, heat.checked_sub(taken).expect("part of the heat"));
        Ok((piece, taken))
    }

    /// A piece shaped to cast stops being a form when its shape changes.
    fn stop_casting(&mut self, id: EntityId) {
        let Some(shape) = self.shape_of.get(&id) else {
            return;
        };
        let def = &self.shapes[shape];
        if def.casts.is_some() || def.capacity.is_some() {
            self.forms.remove(&id);
            self.containers.remove(&id);
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
                (self.surroundings.or_insert_with(id, Default::default), id)
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
                (self.surroundings.or_insert_with(id, Default::default), id)
            }
        };
        *store = store.checked_add(amount).ok_or(Fault::Overflow(id))?;
        Ok(())
    }
}

/// `mass` split in the same shares as `composition`, adding up exactly.
fn scale(composition: &Composition, mass: Mass) -> Option<Composition> {
    let total = matter::total_mass(composition);
    if total == 0 {
        return None;
    }
    let shares: Vec<(MaterialId, u64)> = composition
        .iter()
        .map(|(&m, part)| {
            (
                m,
                u64::try_from(u128::from(part.mg()) * 10_000 / total).unwrap_or(0),
            )
        })
        .collect();
    let mut shares = shares;
    let given: u64 = shares.iter().map(|(_, s)| s).sum();
    if let Some(first) = shares.first_mut() {
        first.1 += 10_000 - given;
    }
    let mut grown = Composition::new();
    for (material, part) in matter::split_by_fractions(mass, &shares) {
        if part != Mass::ZERO {
            grown.insert(material, part);
        }
    }
    Some(grown)
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

/// The running totals the gate keeps outside the tables, as they were before
/// a set of changes.
struct Totals {
    next_id: u32,
    sunlight: u128,
    vital_energy: u128,
    vital_matter: u128,
}

impl Totals {
    fn of(world: &World) -> Totals {
        Totals {
            next_id: world.next_id,
            sunlight: world.sunlight,
            vital_energy: world.vital_energy,
            vital_matter: world.vital_matter,
        }
    }

    fn restore(&self, world: &mut World) {
        world.next_id = self.next_id;
        world.sunlight = self.sunlight;
        world.vital_energy = self.vital_energy;
        world.vital_matter = self.vital_matter;
    }
}

#[cfg(debug_assertions)]
struct Audit {
    before: World,
    mass: u128,
    energy: u128,
    credits: u128,
}
