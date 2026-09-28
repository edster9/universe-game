//! Nature: the laws acting on their own, one tick at a time. Fuel burns in
//! lit chambers, heat spreads and leaks away, materials melt and separate,
//! liquids run together, gases rise into the air, and liquid setting in a form
//! takes its shape.
//!
//! Each step proposes changes from the world as it stands and hands them to
//! the gate, like any action. A fault here is a bug in a law.

use std::collections::BTreeMap;

use crate::gate::{Cause, Change, Fault, Holder};
use crate::matter::{self, Composition, State};
use crate::units::{Energy, Mass};
use crate::world::{EntityId, World};

/// Runs `ticks` ticks.
pub fn run(world: &mut World, ticks: u64) -> Result<(), Fault> {
    for _ in 0..ticks {
        tick(world)?;
    }
    Ok(())
}

/// One tick of nature.
pub fn tick(world: &mut World) -> Result<(), Fault> {
    let now = world.tick();
    let steps: [fn(&World) -> Vec<Change>; 6] =
        [burn, share_heat, lose_heat, separate, pool, set_shapes];
    for step in steps {
        let changes = step(world);
        if !changes.is_empty() {
            world.apply(Cause::Nature { tick: now }, changes)?;
        }
    }
    world.advance_clock();
    Ok(())
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
fn burn(world: &World) -> Vec<Change> {
    let mut changes = Vec::new();
    for (&id, chamber) in &world.chambers {
        if !chamber.lit {
            continue;
        }
        let mut budget = chamber.burn_rate.mg();
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
fn share_heat(world: &World) -> Vec<Change> {
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
/// by heat capacity. Nothing cools below its surroundings.
fn lose_heat(world: &World) -> Vec<Change> {
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
        let rate = match world
            .location(id)
            .and_then(|l| world.chamber(l).map(|c| (l, c)))
        {
            Some((chamber_id, chamber)) => {
                let total = chamber_capacity[&chamber_id];
                if total == 0 {
                    0
                } else {
                    u128::from(chamber.heat_loss) * capacity / total
                }
            }
            None => u128::from(world.settings.open_air_heat_loss),
        };
        let loss = rate * u128::from(temperature.mk() - ambient.mk()) / 1_000;
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

/// Parts of a piece in different states come apart: gas rises into the
/// place's air, and liquid runs out of the solid and stays where the piece is.
/// Gas held inside something rises out of it.
fn separate(world: &World) -> Vec<Change> {
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

        if liquid.is_empty() && solid.is_empty() {
            if location != place && world.contents(id).is_empty() && !world.is_container(id) {
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
        if !liquid.is_empty() && !solid.is_empty() {
            changes.push(Change::Split {
                from: id,
                take: liquid,
                at: location,
            });
        }
    }
    changes
}

/// Liquids in the same container run together, and gases in the same place
/// mix, each into the earliest piece.
fn pool(world: &World) -> Vec<Change> {
    let mut groups: BTreeMap<(EntityId, bool), Vec<EntityId>> = BTreeMap::new();
    for &id in world.matter.keys() {
        if world.is_container(id) || !world.contents(id).is_empty() {
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
fn set_shapes(world: &World) -> Vec<Change> {
    let mut changes = Vec::new();
    for &id in world.matter.keys() {
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
