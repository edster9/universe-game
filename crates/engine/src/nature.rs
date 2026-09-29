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
use crate::world::{Activity, EntityId, World};

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

/// Up to `seconds` of nature, stopping early once `busy` is false.
pub fn run_while(
    world: &mut World,
    seconds: u64,
    busy: impl Fn(&World) -> bool,
) -> Result<(), Fault> {
    let mut left = seconds;
    while left > 0 && busy(world) {
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

/// How hard a body shivers: the extra power it burns is this many times
/// the heat it would lose for the same shortfall below its set point. High
/// enough to hold a body within a fraction of a kelvin of its set point.
const SHIVER_GAIN: u128 = 20;

type Law = fn(&World, u64) -> Vec<Change>;

/// `dt` seconds of nature, as one step.
fn step(world: &mut World, dt: u64) -> Result<(), Fault> {
    let now = world.tick();
    let laws: [Law; 16] = [
        burn,
        burn_in_the_open,
        friction,
        share_heat,
        conduct,
        lose_heat,
        live,
        grow,
        transform,
        clear_air,
        limits_of_life,
        fall_asleep,
        finish_activities,
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
    // Creatures acting on instinct choose what to do next.
    crate::instinct::act(world)?;
    world.advance_clock(dt);
    Ok(())
}

/// One second while anything is burning, hot, or changing state; up to the
/// world's calm step otherwise. A step never runs past the end of someone's
/// hard work.
fn step_size(world: &World, left: u64) -> u64 {
    if world.chambers.values().any(|c| c.lit) || !world.activities.is_empty() {
        return 1;
    }
    for &id in world.matter.keys() {
        // Bodies keep their own heat, and fixed sources (a body of liquid, a
        // bank of earth) are too big to change quickly, however far they lag
        // the air.
        if world.life.contains_key(&id) || !world.is_portable(id) {
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
        .held(holder)
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

/// Anything warmer than its surroundings loses heat to them, and anything
/// colder takes heat from them, in proportion to the difference. Inside a
/// chamber, its insulation sets the rate, shared out by heat capacity. A body
/// exchanges heat at its own rate. Nothing passes its surroundings'
/// temperature. Warming draws on the heat the surroundings have taken in, and
/// beyond that on sunlight.
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
        if temperature == ambient {
            continue;
        }
        let (hotter, colder) = (temperature.max(ambient), temperature.min(ambient));
        let capacity = world.heat_capacity(id);
        let chamber = world
            .location(id)
            .and_then(|l| world.chamber(l).map(|c| (l, c)));
        let difference = u128::from(hotter.mk() - colder.mk());
        let flow = if let Some(life) = world.life.get(&id) {
            // A shelter keeps in a share of a sleeper's heat.
            let kept = sheltered(world, id).unwrap_or(0).min(10_000);
            u128::from(life.heat_loss) * difference * u128::from(dt) / 1_000
                * u128::from(10_000 - kept)
                / 10_000
        } else if let Some((chamber_id, chamber)) = chamber {
            let total = chamber_capacity[&chamber_id];
            let rate = if total == 0 {
                0
            } else {
                u128::from(chamber.heat_loss) * capacity / total
            };
            rate * difference * u128::from(dt) / 1_000
        } else if let (Some(convection), Some(area)) =
            (world.settings.convection, surface(world, id))
        {
            // Convection from the surface, plus radiation, which dominates
            // once something glows.
            let convected =
                u128::from(convection) * area * difference * u128::from(dt) / 1_000_000_000_000;
            convected
                + radiated(
                    area,
                    hotter.mk(),
                    colder.mk(),
                    world.settings.emissivity,
                    dt,
                )
        } else {
            u128::from(world.settings.open_air_heat_loss) * difference * u128::from(dt) / 1_000
        };
        // Never past the air's temperature, either way.
        let at_ambient = matter::energy_at(ambient, capacity);
        let have = u128::from(energy.uj());
        if temperature > ambient {
            let loss = flow.min(have.saturating_sub(at_ambient));
            if loss > 0 {
                changes.push(Change::Heat {
                    from: Holder::Thing(id),
                    to: Holder::Surroundings(place),
                    amount: Energy::from_uj(u64::try_from(loss).expect("part of one piece's heat")),
                });
            }
        } else {
            let gain = flow.min(at_ambient.saturating_sub(have));
            if gain > 0 {
                changes.push(Change::Warm {
                    entity: id,
                    place,
                    amount: Energy::from_uj(u64::try_from(gain).expect("a piece's worth of heat")),
                });
            }
        }
    }
    changes
}

/// Surface area of a piece in µm², if its materials' densities are known.
fn surface(world: &World, id: EntityId) -> Option<u128> {
    matter::surface_area(&world.materials, world.matter.get(&id)?)
}

/// Heat radiated in `dt` seconds, in µJ: emissivity × σ × area × (T⁴ − T₀⁴).
fn radiated(area_um2: u128, mk: u64, ambient_mk: u64, emissivity: u64, dt: u64) -> u128 {
    // In centikelvin and mm², σ × 10¹⁴ = 5,670,374 gives watts × 10²⁸.
    let fourth = |mk: u64| u128::from(mk / 10).pow(4);
    let difference = fourth(mk).saturating_sub(fourth(ambient_mk));
    let watts_e28 = 5_670_374u128
        .saturating_mul(area_um2 / 1_000_000)
        .saturating_mul(difference);
    // Watts × 10²⁸ ÷ 10²² is µJ per second.
    watts_e28 / 1_000_000_000_000_000_000 * u128::from(emissivity) * u128::from(dt) / 100_000_000
}

/// Anything holding a material that burns on its own, hotter than that
/// material's ignition point, burns: its surface burns away at the material's
/// burn speed, and the energy released heats it. A share of that heat goes
/// into the things held with it, split by their surfaces: a flame heats what
/// it's piled with. Inside a lit chamber, the chamber's own draught does the
/// burning instead.
fn burn_in_the_open(world: &World, dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, composition) in &world.matter {
        let in_lit_chamber = world
            .location(id)
            .and_then(|l| world.chamber(l))
            .is_some_and(|c| c.lit);
        if !world.is_burning(id) || in_lit_chamber {
            continue;
        }
        let Some(area) = surface(world, id) else {
            continue;
        };
        let temperature = world.temperature(id).unwrap_or_default();
        for (&material, &mass) in composition {
            let m = &world.materials[&material];
            if !m.burns() || m.ignition_point.is_none_or(|point| temperature < point) {
                continue;
            }
            let amount =
                (area * u128::from(m.burn_speed) * u128::from(dt) / 1_000_000_000_000).max(1);
            let amount = u64::try_from(amount).unwrap_or(u64::MAX).min(mass.mg());
            if amount == 0 {
                continue;
            }
            changes.push(Change::Burn {
                entity: id,
                material,
                mass: Mass::from_mg(amount),
            });
            let released = u128::from(amount) * u128::from(m.energy_density);
            changes.extend(flame(world, id, released));
        }
    }
    changes
}

/// Heat from a burning piece to the other things held with it, split by
/// their surfaces.
fn flame(world: &World, id: EntityId, released: u128) -> Vec<Change> {
    let Some(holder) = world.location(id).filter(|&h| world.is_container(h)) else {
        return Vec::new();
    };
    let neighbours: Vec<(EntityId, u128)> = matter_pieces(world, holder)
        .into_iter()
        .filter(|&n| n != id)
        .filter_map(|n| surface(world, n).map(|a| (n, a)))
        .collect();
    let total: u128 = neighbours.iter().map(|(_, a)| a).sum();
    if total == 0 {
        return Vec::new();
    }
    let shared = released * u128::from(world.settings.flame_share) / 10_000;
    neighbours
        .into_iter()
        .filter_map(|(n, area)| {
            let amount = shared * area / total;
            (amount > 0).then(|| Change::Heat {
                from: Holder::Thing(id),
                to: Holder::Thing(n),
                amount: Energy::from_uj(u64::try_from(amount).expect("part of the heat released")),
            })
        })
        .collect()
}

/// Rubbing two things together wears the softer one into dust, and turns a
/// share of the worker's extra effort into heat in that dust.
fn friction(world: &World, dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    let reference = world.settings.reference_temperature;
    for (&agent, activity) in &world.activities {
        let Activity::Rubbing {
            first,
            second,
            dust,
            ..
        } = *activity;
        let (Some(life), true) = (world.life.get(&agent), world.is_living(agent)) else {
            continue;
        };
        if !world.matter.contains_key(&dust) {
            continue;
        }
        let hardness = |id: EntityId| {
            world.matter.get(&id).and_then(matter::dominant).map(|m| {
                world.materials[&m]
                    .hardness_at(world.temperature(id).unwrap_or(reference), reference)
            })
        };
        let softer = if hardness(second) < hardness(first) {
            second
        } else {
            first
        };
        if let Some(composition) = world.matter.get(&softer) {
            let wear =
                (world.settings.wear_rate * dt).min(world.mass(softer).mg().saturating_sub(1));
            if let Some(take) =
                matter::proportional(composition, Mass::from_mg(wear)).filter(|t| !t.is_empty())
            {
                changes.push(Change::Shift {
                    from: softer,
                    to: dust,
                    take,
                });
            }
        }
        let effort = life.working_power.saturating_sub(life.resting_power);
        let heat = u128::from(effort) * u128::from(world.settings.friction_share) / 10_000
            * u128::from(dt);
        let heat = heat.min(u128::from(world.heat(agent).map_or(0, |e| e.uj())));
        if heat > 0 {
            changes.push(Change::Heat {
                from: Holder::Thing(agent),
                to: Holder::Thing(dust),
                amount: Energy::from_uj(u64::try_from(heat).expect("part of the body's heat")),
            });
        }
    }
    changes
}

/// Dust and the two things being rubbed to make it pass heat between them,
/// in proportion to the difference and to the smaller one's surface. (Fire
/// spreads through a pile by flame, not by touch: see `flame`.)
fn conduct(world: &World, dt: u64) -> Vec<Change> {
    let mut pairs: Vec<(EntityId, EntityId)> = Vec::new();
    for activity in world.activities.values() {
        let Activity::Rubbing {
            first,
            second,
            dust,
            ..
        } = *activity;
        pairs.push((dust, first));
        pairs.push((dust, second));
    }

    // First, each pair's flow on its own terms: in proportion to the
    // difference and the smaller surface, never past an even temperature.
    let mut flows: Vec<(EntityId, EntityId, u128)> = Vec::new();
    for (a, b) in pairs {
        let (Some(area_a), Some(area_b)) = (surface(world, a), surface(world, b)) else {
            continue;
        };
        let (Some(ta), Some(tb)) = (world.temperature(a), world.temperature(b)) else {
            continue;
        };
        let (hot, cold) = if ta > tb { (a, b) } else { (b, a) };
        let difference = u128::from(ta.mk().abs_diff(tb.mk()));
        if difference == 0 {
            continue;
        }
        let area = area_a.min(area_b);
        let flow = u128::from(world.settings.touch_transfer) * area * difference * u128::from(dt)
            / 1_000_000_000_000;
        let (ch, cc) = (world.heat_capacity(hot), world.heat_capacity(cold));
        let even = if ch + cc == 0 {
            0
        } else {
            difference * ch / 1_000 * cc / (ch + cc)
        };
        let flow = flow.min(even);
        if flow > 0 {
            flows.push((hot, cold, flow));
        }
    }

    // Then, however many things a piece touches, it gives away at most half
    // the heat it holds above its coldest neighbour, so it can't overshoot.
    let mut out: BTreeMap<EntityId, (u128, u64)> = BTreeMap::new();
    for &(hot, cold, flow) in &flows {
        let coldest = world.temperature(cold).map_or(0, |t| t.mk());
        let entry = out.entry(hot).or_insert((0, coldest));
        entry.0 += flow;
        entry.1 = entry.1.min(coldest);
    }
    let allowed: BTreeMap<EntityId, (u128, u128)> = out
        .into_iter()
        .map(|(hot, (total, coldest))| {
            let above = u128::from(
                world
                    .temperature(hot)
                    .map_or(0, |t| t.mk())
                    .saturating_sub(coldest),
            );
            let spare = above * world.heat_capacity(hot) / 1_000 / 2;
            (hot, (spare.min(total), total))
        })
        .collect();

    flows
        .into_iter()
        .filter_map(|(hot, cold, flow)| {
            let (spare, total) = allowed[&hot];
            let flow = if total == 0 { 0 } else { flow * spare / total };
            (flow > 0).then(|| Change::Heat {
                from: Holder::Thing(hot),
                to: Holder::Thing(cold),
                amount: Energy::from_uj(u64::try_from(flow).expect("part of one piece's heat")),
            })
        })
        .collect()
}

/// An activity stops when its time is up, or when the worker dies or lets go
/// of what they were rubbing.
fn finish_activities(world: &World, _dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&agent, activity) in &world.activities {
        let Activity::Rubbing {
            first,
            second,
            dust,
            until,
        } = *activity;
        let holding = |id: EntityId| world.location(id) == Some(agent);
        // Rubbing into a container stops once something else in it catches:
        // what the rubbing was for.
        let caught = world.location(dust).is_some_and(|container| {
            world.is_container(container)
                && world
                    .held(container)
                    .into_iter()
                    .any(|e| e != dust && world.is_burning(e))
        });
        let over = world.tick >= until
            || caught
            || !world.is_living(agent)
            || !holding(first)
            || !holding(second)
            || !world.matter.contains_key(&dust);
        if over {
            changes.push(Change::EndActivity { agent });
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

        // Burning stores. Below its set point, a body burns more to keep
        // warm (shivering), the colder the more, up to its working power.
        let temperature = world.temperature(id).unwrap_or_default();
        let power = if working {
            life.working_power
        } else {
            let below = u128::from(life.set_point.mk().saturating_sub(temperature.mk()));
            let shiver = u128::from(life.heat_loss) * below * SHIVER_GAIN / 1_000;
            let most = u128::from(life.working_power.saturating_sub(life.resting_power));
            life.resting_power + u64::try_from(shiver.min(most)).expect("at most working power")
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

        // Storing a surplus: food beyond half a day's needs at rest becomes
        // the reserve again, at up to the body's resting power, until the reserve
        // is back where it began.
        if let (Some(efficiency), Some((reserve, start))) = (life.stores, life.reserve) {
            let have = composition.get(&reserve).map_or(0, |m| m.mg());
            let mut foods: Vec<_> = composition
                .iter()
                .filter(|(m, _)| {
                    **m != reserve && life.digests.contains(m) && world.materials[m].burns()
                })
                .map(|(&m, &mass)| (world.materials[&m].energy_density, m, mass))
                .collect();
            foods.sort();
            let food_energy: u128 = foods
                .iter()
                .map(|(density, _, mass)| u128::from(*density) * u128::from(mass.mg()))
                .sum::<u128>()
                // Less what this step burns.
                .saturating_sub(u128::from(power) * u128::from(dt));
            let half_a_day = u128::from(life.resting_power) * u128::from(SECONDS_PER_DAY) / 2;
            let surplus = food_energy.saturating_sub(half_a_day);
            let room = u128::from(start.mg().saturating_sub(have))
                * u128::from(world.materials[&reserve].energy_density)
                * 10_000
                / u128::from(efficiency.max(1));
            let energy = surplus
                .min(room)
                .min(u128::from(life.resting_power) * u128::from(dt));
            if let Some(&(density, material, mass)) = foods.first()
                && energy > 0
            {
                let amount = (energy / u128::from(density.max(1))).min(u128::from(mass.mg()));
                if amount > 0 {
                    changes.push(Change::Store {
                        entity: id,
                        from: material,
                        mass: Mass::from_mg(u64::try_from(amount).expect("no more than it has")),
                        into: reserve,
                        efficiency,
                    });
                }
            }
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
        // Wounds bleed.
        lost += world.bleeding(id).saturating_mul(dt);
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

/// The share of heat kept in for someone asleep in a shelter at the same
/// place, in parts per ten thousand.
fn sheltered(world: &World, body: EntityId) -> Option<u64> {
    let sleep = world.life.get(&body)?.sleep.as_ref()?;
    let shelter = sleep.shelter.filter(|_| sleep.until > world.tick)?;
    if world.place_of(shelter) != world.place_of(body) {
        return None;
    }
    world.designs[&world.assembly(shelter)?.design].shelter
}

/// A body awake too long falls asleep where it is, until rested.
fn fall_asleep(world: &World, _dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, life) in &world.life {
        let Some(sleep) = life.sleep.as_ref().filter(|_| life.died_of.is_none()) else {
            continue;
        };
        if world.is_asleep(id) {
            continue;
        }
        let awake = world.awake_for(id).unwrap_or(0);
        if awake >= sleep.collapse {
            let need = u128::from(awake) * u128::from(sleep.need) / u128::from(sleep.awake.max(1));
            changes.push(Change::Sleep {
                shelter: None,
                agent: id,
                until: world.tick() + u64::try_from(need).unwrap_or(u64::MAX).max(1),
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
        let cause = if fluid < life.fluid_minimum && world.bleeding(id) > 0 {
            Some("bleeding")
        } else if fluid < life.fluid_minimum {
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

/// A material hot enough to turn into another does, as some earths fire hard.
fn transform(world: &World, _dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, composition) in &world.matter {
        let temperature = world.temperature(id).unwrap_or_default();
        for (&material, &mass) in composition {
            if let Some((to, at)) = world.materials[&material].becomes
                && temperature >= at
            {
                changes.push(Change::Transform {
                    entity: id,
                    from: material,
                    to,
                    mass,
                });
            }
        }
    }
    changes
}

/// Living sources grow toward their limit, quickly while small and slowing
/// as they fill up, drawing matter from their source and energy from
/// sunlight.
fn grow(world: &World, dt: u64) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, growth) in &world.growth {
        let mass = u128::from(world.mass(id).mg());
        let limit = u128::from(growth.limit.mg());
        if mass == 0 || mass >= limit {
            continue;
        }
        let amount = u128::from(growth.rate) * mass * (limit - mass) / limit * u128::from(dt)
            / 10_000
            / 86_400;
        let spare = u128::from(world.mass(growth.from).mg()).saturating_sub(1);
        let amount = amount.min(spare).min(limit - mass);
        if amount > 0 {
            changes.push(Change::Grow {
                entity: id,
                from: growth.from,
                mass: Mass::from_mg(u64::try_from(amount).expect("less than the limit")),
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
            || world.in_use(id)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glowing_things_radiate_far_more_than_warm_ones() {
        // Radiation grows with the fourth power of temperature. At 1300 K a
        // surface is ten times further above 300 K air than at 400 K, but
        // radiates hundreds of times as much.
        let area = 10_000_000_000; // 100 cm²
        let warm = radiated(area, 400_000, 300_000, 9_000, 1);
        let glowing = radiated(area, 1_300_000, 300_000, 9_000, 1);
        assert!(glowing > 100 * warm, "{glowing} vs {warm}");
        // 100 cm² at 1300 K radiates about 1.45 kW.
        assert!(
            (1_400_000_000..1_500_000_000).contains(&glowing),
            "{glowing} µJ"
        );
    }
}
