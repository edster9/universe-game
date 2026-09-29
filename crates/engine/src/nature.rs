//! Nature: the laws acting on their own as time passes. Fuel burns in lit
//! chambers, heat spreads and leaks away, bodies live on what they've eaten,
//! lose fluid, and sweat, air clears, materials melt and separate, liquids run
//! together, gases rise into the air, and liquid setting in a form takes its
//! shape.
//!
//! Each step proposes changes from the world as it stands and hands them to
//! the gate, like any action. A fault here is a bug in a law.
//!
//! Time moves in steps. When something fast is happening (a fire, something
//! hot, melting), a step is one second. When all is calm, nature takes bigger
//! steps, so days pass quickly and still replay exactly.

use std::collections::BTreeMap;

use crate::gate::{Cause, Change, Fault, Holder};
use crate::matter::{self, Composition, State};
use crate::units::{Energy, Mass};
use crate::world::{EntityId, World};

/// How far from its surroundings' temperature something non-living can be
/// and still count as calm, in mK.
const CALM_WITHIN_MK: u64 = 5_000;

const SECONDS_PER_DAY: u64 = 86_400;

/// Lets `seconds` of time pass.
pub fn run(world: &mut World, seconds: u64) -> Result<(), Fault> {
    let mut left = seconds;
    while left > 0 {
        let dt = step_size(world, left);
        step(world, dt)?;
        left -= dt;
    }
    Ok(())
}

/// One second of nature.
pub fn tick(world: &mut World) -> Result<(), Fault> {
    step(world, 1)
}

type Law = fn(&World, u64) -> Vec<Change>;

/// `dt` seconds of nature, as one step.
fn step(world: &mut World, dt: u64) -> Result<(), Fault> {
    let now = world.tick();
    let laws: [Law; 9] = [
        burn,
        share_heat,
        lose_heat,
        live,
        clear_air,
        limits_of_life,
        separate,
        pool,
        set_shapes,
    ];
    for law in laws {
        let changes = law(world, dt);
        if !changes.is_empty() {
            world.apply(Cause::Nature { tick: now }, changes)?;
        }
    }
    world.advance_clock(dt);
    Ok(())
}

/// One second while anything is burning, hot, or changing state; up to the
/// world's calm step otherwise. A step never runs past the end of someone's
/// hard work.
fn step_size(world: &World, left: u64) -> u64 {
    if world.chambers.values().any(|c| c.lit) {
        return 1;
    }
    for &id in world.matter.keys() {
        if world.life.contains_key(&id) {
            continue;
        }
        let Some(place) = world.place_of(id) else {
            continue;
        };
        let temperature = world.temperature(id).unwrap_or_default();
        if temperature.mk().abs_diff(world.ambient(place).mk()) > CALM_WITHIN_MK {
            return 1;
        }
    }
    let mut dt = left.min(world.settings.calm_step.max(1));
    for life in world.life.values().filter(|l| l.died_of.is_none()) {
        if life.working_until > world.tick {
            dt = dt.min(life.working_until - world.tick);
        }
    }
    dt.max(1)
}

fn matter_pieces(world: &World, holder: EntityId) -> Vec<EntityId> {
    world
        .contents(holder)
        .into_iter()
        .filter(|&e| world.composition(e).is_some())
        .collect()
}

/// Lit chambers burn fuel inside them, up to their burn rate. A chamber with
/// nothing left to burn goes out.
fn burn(world: &World, dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, chamber) in &world.chambers {
        if !chamber.lit {
            continue;
        }
        let mut budget = chamber.burn_rate.mg().saturating_mul(dt);
        let mut has_fuel = false;
        for piece in matter_pieces(world, id) {
            for (&material, &mass) in world.composition(piece).expect("matter") {
                if !world.materials[&material].burns() {
                    continue;
                }
                has_fuel = true;
                let amount = budget.min(mass.mg());
                if amount > 0 {
                    changes.push(Change::Burn {
                        entity: piece,
                        material,
                        mass: Mass::from_mg(amount),
                    });
                    budget -= amount;
                }
            }
        }
        if !has_fuel {
            changes.push(Change::Light {
                chamber: id,
                lit: false,
            });
        }
    }
    changes
}

/// Everything inside a chamber comes to the same temperature.
fn share_heat(world: &World, _dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for &id in world.chambers.keys() {
        let pieces = matter_pieces(world, id);
        if pieces.len() < 2 {
            continue;
        }
        let capacities: Vec<u128> = pieces.iter().map(|&p| world.heat_capacity(p)).collect();
        let energies: Vec<u128> = pieces
            .iter()
            .map(|&p| u128::from(world.heat(p).expect("matter").uj()))
            .collect();
        let total_capacity: u128 = capacities.iter().sum();
        let total_energy: u128 = energies.iter().sum();
        if total_capacity == 0 {
            continue;
        }
        let mut targets: Vec<u128> = capacities
            .iter()
            .map(|c| total_energy * c / total_capacity)
            .collect();
        targets[0] += total_energy - targets.iter().sum::<u128>();

        // Pair pieces with too much heat against pieces with too little.
        let mut givers: Vec<(EntityId, u128)> = Vec::new();
        let mut takers: Vec<(EntityId, u128)> = Vec::new();
        for ((&piece, &energy), &target) in pieces.iter().zip(&energies).zip(&targets) {
            if energy > target {
                givers.push((piece, energy - target));
            } else if target > energy {
                takers.push((piece, target - energy));
            }
        }
        let (mut g, mut t) = (0, 0);
        while g < givers.len() && t < takers.len() {
            let amount = givers[g].1.min(takers[t].1);
            changes.push(Change::Heat {
                from: Holder::Thing(givers[g].0),
                to: Holder::Thing(takers[t].0),
                amount: Energy::from_uj(u64::try_from(amount).expect("part of one piece's heat")),
            });
            givers[g].1 -= amount;
            takers[t].1 -= amount;
            if givers[g].1 == 0 {
                g += 1;
            }
            if takers[t].1 == 0 {
                t += 1;
            }
        }
    }
    changes
}

/// Anything warmer than its surroundings loses heat to them, in proportion to
/// the difference. Inside a chamber, its insulation sets the rate, shared out
/// by heat capacity. A body loses heat at its own rate. Nothing cools below
/// its surroundings.
fn lose_heat(world: &World, dt: u64) -> Vec<Change> {
    let chamber_capacity: BTreeMap<EntityId, u128> = world
        .chambers
        .keys()
        .map(|&c| {
            (
                c,
                matter_pieces(world, c)
                    .iter()
                    .map(|&p| world.heat_capacity(p))
                    .sum(),
            )
        })
        .collect();

    let mut changes = Vec::new();
    for (&id, energy) in &world.heat {
        let Some(place) = world.place_of(id) else {
            continue;
        };
        let ambient = world.ambient(place);
        let temperature = world.temperature(id).expect("matter");
        if temperature <= ambient {
            continue;
        }
        let capacity = world.heat_capacity(id);
        let chamber = world
            .location(id)
            .and_then(|l| world.chamber(l).map(|c| (l, c)));
        let rate = if let Some(life) = world.life.get(&id) {
            u128::from(life.heat_loss)
        } else if let Some((chamber_id, chamber)) = chamber {
            let total = chamber_capacity[&chamber_id];
            if total == 0 {
                0
            } else {
                u128::from(chamber.heat_loss) * capacity / total
            }
        } else {
            u128::from(world.settings.open_air_heat_loss)
        };
        let loss = rate * u128::from(temperature.mk() - ambient.mk()) * u128::from(dt) / 1_000;
        let above_ambient =
            u128::from(energy.uj()).saturating_sub(matter::energy_at(ambient, capacity));
        let loss = loss.min(above_ambient);
        if loss > 0 {
            changes.push(Change::Heat {
                from: Holder::Thing(id),
                to: Holder::Surroundings(place),
                amount: Energy::from_uj(u64::try_from(loss).expect("part of one piece's heat")),
            });
        }
    }
    changes
}

/// A living body burns what it has digested to keep going, the least
/// energy-rich first, faster while working. It loses fluid through breath and
/// skin, twice as fast while working. Above its set point it sweats, and each
/// mg of sweat carries heat away.
fn live(world: &World, dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, life) in &world.life {
        if life.died_of.is_some() {
            continue;
        }
        let (Some(composition), Some(place)) = (world.composition(id), world.place_of(id)) else {
            continue;
        };
        let working = life.working_until > world.tick;

        // Burning stores.
        let power = if working {
            life.working_power
        } else {
            life.resting_power
        };
        let mut needed = u128::from(power) * u128::from(dt);
        let mut stores: Vec<_> = composition
            .iter()
            .filter(|(m, _)| life.digests.contains(m) && world.materials[m].burns())
            .map(|(&m, &mass)| (world.materials[&m].energy_density, m, mass))
            .collect();
        stores.sort();
        for (density, material, mass) in stores {
            if needed == 0 {
                break;
            }
            let density = u128::from(density).max(1);
            let amount = needed.div_ceil(density).min(u128::from(mass.mg()));
            if amount == 0 {
                continue;
            }
            changes.push(Change::Burn {
                entity: id,
                material,
                mass: Mass::from_mg(u64::try_from(amount).expect("no more than it has")),
            });
            needed = needed.saturating_sub(amount * density);
        }

        // Losing fluid, and sweating.
        let have = composition.get(&life.fluid).map_or(0, |m| m.mg());
        let rate = if working {
            life.fluid_loss * 2
        } else {
            life.fluid_loss
        };
        let mut lost = (u128::from(rate) * u128::from(dt) / u128::from(SECONDS_PER_DAY)) as u64;
        let capacity = world.heat_capacity(id);
        let temperature = world.temperature(id).unwrap_or_default();
        let mut carried = 0u128;
        if temperature > life.set_point && life.sweat_heat > 0 {
            let excess = capacity * u128::from(temperature.mk() - life.set_point.mk()) / 1_000;
            let most = u128::from(life.sweat_rate) * u128::from(dt) / u128::from(SECONDS_PER_DAY);
            carried = excess.min(most * u128::from(life.sweat_heat));
            lost += u64::try_from(carried / u128::from(life.sweat_heat)).expect("sweat fits");
        }
        let lost = lost.min(have.saturating_sub(1));
        if lost > 0 {
            changes.push(Change::Release {
                from: id,
                take: Composition::from([(life.fluid, Mass::from_mg(lost))]),
                place,
            });
        }
        if carried > 0 {
            changes.push(Change::Heat {
                from: Holder::Thing(id),
                to: Holder::Surroundings(place),
                amount: Energy::from_uj(u64::try_from(carried).expect("part of the body's heat")),
            });
        }
    }
    changes
}

/// A body dies when its fluid falls below its minimum, when it has nothing
/// left to burn, or when its temperature leaves the range it can live in.
fn limits_of_life(world: &World, _dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, life) in &world.life {
        if life.died_of.is_some() {
            continue;
        }
        let Some(composition) = world.composition(id) else {
            continue;
        };
        let fluid = composition.get(&life.fluid).copied().unwrap_or(Mass::ZERO);
        let has_stores = composition
            .keys()
            .any(|m| life.digests.contains(m) && world.materials[m].burns());
        let temperature = world.temperature(id).unwrap_or_default();
        let cause = if fluid < life.fluid_minimum {
            Some("thirst")
        } else if !has_stores {
            Some("hunger")
        } else if temperature < life.coldest {
            Some("cold")
        } else if temperature > life.hottest {
            Some("heat")
        } else {
            None
        };
        if let Some(cause) = cause {
            changes.push(Change::Die {
                agent: id,
                cause: cause.into(),
            });
        }
    }
    changes
}

/// Gas hanging in a place's open air passes into its surroundings a little at
/// a time.
fn clear_air(world: &World, dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    let share = u128::from(world.settings.air_clearing) * u128::from(dt);
    for (&id, composition) in &world.matter {
        let Some(place) = world.location(id).filter(|&l| world.is_place(l)) else {
            continue;
        };
        if world.life.contains_key(&id) || !world.is_all(id, State::Gas) || !world.is_portable(id) {
            continue;
        }
        let total = matter::total_mass(composition);
        let amount = (total * share / 10_000).min(total.saturating_sub(1));
        if amount == 0 {
            continue;
        }
        let amount = Mass::from_mg(u64::try_from(amount).expect("part of the gas"));
        if let Some(take) = matter::proportional(composition, amount) {
            changes.push(Change::Release {
                from: id,
                take,
                place,
            });
        }
    }
    changes
}

/// Parts of a piece in different states come apart: gas rises into the
/// place's air, and liquid runs out of the solid and stays where the piece is.
/// Gas held inside something rises out of it. A body holds its liquids, but
/// not gas.
fn separate(world: &World, _dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, composition) in &world.matter {
        let (Some(place), Some(location)) = (world.place_of(id), world.location(id)) else {
            continue;
        };
        let states = world.states(id);
        let part = |state: State| -> Composition {
            states
                .iter()
                .filter(|&&(_, s)| s == state)
                .map(|&(m, _)| (m, composition[&m]))
                .collect()
        };
        let (gas, liquid, solid) = (part(State::Gas), part(State::Liquid), part(State::Solid));
        let body = world.life.contains_key(&id);

        if liquid.is_empty() && solid.is_empty() {
            if !body
                && location != place
                && world.contents(id).is_empty()
                && !world.is_container(id)
            {
                changes.push(Change::Move {
                    entity: id,
                    to: place,
                });
            }
            continue;
        }
        if !gas.is_empty() {
            changes.push(Change::Split {
                from: id,
                take: gas,
                at: place,
            });
        }
        if !body && !liquid.is_empty() && !solid.is_empty() {
            changes.push(Change::Split {
                from: id,
                take: liquid,
                at: location,
            });
        }
    }
    changes
}

/// Loose liquids in the same container run together, and loose gases in the
/// same place mix, each into the earliest piece. Fixed pools and seas and
/// living bodies stay as they are.
fn pool(world: &World, _dt: u64) -> Vec<Change> {
    let mut groups: BTreeMap<(EntityId, bool), Vec<EntityId>> = BTreeMap::new();
    for &id in world.matter.keys() {
        if world.is_container(id)
            || !world.contents(id).is_empty()
            || !world.is_portable(id)
            || world.life.contains_key(&id)
        {
            continue;
        }
        let Some(location) = world.location(id) else {
            continue;
        };
        if world.is_all(id, State::Liquid) {
            groups.entry((location, true)).or_default().push(id);
        } else if world.is_all(id, State::Gas) && world.is_place(location) {
            groups.entry((location, false)).or_default().push(id);
        }
    }
    groups
        .values()
        .filter(|pieces| pieces.len() > 1)
        .flat_map(|pieces| {
            pieces[1..].iter().map(|&from| Change::Merge {
                from,
                into: pieces[0],
            })
        })
        .collect()
}

/// A solid piece inside a form takes the form's shape, made to the form's
/// tolerance. Anything that melts loses its shape.
fn set_shapes(world: &World, _dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for &id in world.matter.keys() {
        if world.life.contains_key(&id) {
            continue;
        }
        let current = world.shape(id);
        if world.is_all(id, State::Solid) {
            let form = world.location(id).and_then(|l| world.form(l));
            if let Some(form) = form
                && current != Some(form.shape.as_str())
            {
                changes.push(Change::Shape {
                    entity: id,
                    shape: Some((form.shape.clone(), form.tolerance)),
                });
            }
        } else if current.is_some() {
            changes.push(Change::Shape {
                entity: id,
                shape: None,
            });
        }
    }
    changes
}
