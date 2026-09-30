//! Instinct: how creatures that aren't persons decide what to do. See
//! docs/ideas/kinds.md.
//!
//! An instinct looks after its own body and follows a few rules its kind
//! gives in data. It only ever proposes ordinary commands, the same ones a
//! person types; the laws decide what happens.

use std::collections::{BTreeSet, VecDeque};

use crate::gate::{Cause, Change, Fault};
use crate::intent::Intent;
use crate::laws;
use crate::matter::State;
use crate::world::{EntityId, Life, World};

/// The least chance of finding food, in parts per ten thousand, that makes
/// a place worth foraging in.
const WORTH_SEARCHING: u64 = 2_000;

/// Lets every creature acting on instinct that's free start what it does
/// next. What it does takes as long as it would for anyone; with nothing to
/// do, it rests for a while.
pub fn act(world: &mut World) -> Result<(), Fault> {
    let free: Vec<EntityId> = world
        .life
        .iter()
        .filter(|(id, life)| {
            life.died_of.is_none()
                && world.instinct(**id).is_some()
                && !world.is_asleep(**id)
                && !world.is_busy(**id)
        })
        .map(|(id, _)| *id)
        .collect();
    for creature in free {
        let now = world.tick();
        let rest = world.instinct(creature).map_or(1_800, |i| i.rest);
        let fled_from = world.place_of(creature);
        let planned = decide(world, creature).and_then(|(intent, fleeing)| {
            laws::plan(world, creature, &intent)
                .ok()
                .map(|p| (intent, p, fleeing))
        });
        match planned {
            Some((intent, plan, fleeing)) => {
                let mut changes = plan.changes;
                // Having fled or charged, keep away from there a while.
                if let (true, Some(place), Some(instinct)) =
                    (fleeing, fled_from, world.instinct(creature))
                {
                    changes.push(Change::Avoid {
                        agent: creature,
                        place,
                        until: now + instinct.wary,
                    });
                }
                changes.push(Change::Occupy {
                    agent: creature,
                    until: now + plan.seconds.max(60),
                });
                world.apply(
                    Cause::Action {
                        actor: creature,
                        intent,
                    },
                    changes,
                )?;
            }
            None => world.apply(
                Cause::Nature { tick: now },
                vec![Change::Occupy {
                    agent: creature,
                    until: now + rest,
                }],
            )?,
        }
    }
    Ok(())
}

/// What a creature wants to do now, most pressing first: get away from what
/// it fears, sleep at night, eat or drop what it holds, drink when thirsty,
/// forage when hungry, and otherwise sometimes wander. `None` means rest;
/// `true` alongside means it's fleeing.
fn decide(world: &World, me: EntityId) -> Option<(Intent, bool)> {
    let instinct = world.instinct(me)?;
    let life = world.life(me)?;
    let here = world.place_of(me)?;
    // Within its range, and not somewhere it was scared lately.
    let within =
        |p: EntityId| world.range(me).is_none_or(|r| r.contains(&p)) && !world.is_avoiding(me, p);
    // Ways out it can take on foot, within the places it keeps to.
    let ways = |from: EntityId| -> Vec<EntityId> {
        world
            .exits(from)
            .iter()
            .copied()
            .filter(|&to| within(to) && world.crossing(from, to).is_none())
            .collect()
    };
    let roll = |salt: u64| world.roll(u64::from(me.0) ^ (salt << 40));
    let go = |to: EntityId| Intent::Go {
        place: world.key(to).to_string(),
        aboard: None,
    };
    let any_way = |salt: u64| {
        let ways = ways(here);
        let n = u64::try_from(ways.len()).expect("a few ways");
        (n > 0).then(|| go(ways[usize::try_from(roll(salt) % n).expect("an index")]))
    };

    // Get away from anything it fears; cornered or hurt, turn on it.
    let threat = world.contents(here).into_iter().find(|&other| {
        other != me
            && world.is_living(other)
            && instinct.flees.iter().any(|k| world.is_kind(other, k))
    });
    if let Some(threat) = threat {
        // Fear of what's here beats wariness of a place: flee somewhere it
        // isn't avoiding if it can, anywhere in its range if it must.
        let escapes: Vec<EntityId> = world
            .exits(here)
            .iter()
            .copied()
            .filter(|&to| {
                world.range(me).is_none_or(|r| r.contains(&to))
                    && world.crossing(here, to).is_none()
            })
            .collect();
        let calm: Vec<EntityId> = escapes
            .iter()
            .copied()
            .filter(|&to| !world.is_avoiding(me, to))
            .collect();
        let choices = if calm.is_empty() { escapes } else { calm };
        let n = u64::try_from(choices.len()).expect("a few ways");
        let escape = (n > 0).then(|| go(choices[usize::try_from(roll(1) % n).expect("an index")]));
        // Hurt, it turns on the threat once, then runs; cornered, it fights.
        let hurt = world.bleeding(me) > 0 && !world.is_avoiding(me, here);
        if instinct.charges && (hurt || escape.is_none()) {
            return Some((
                Intent::Attack {
                    target: world.key(threat).to_string(),
                    with: None,
                },
                true,
            ));
        }
        return escape.map(|go| (go, true));
    }
    let plain = |intent: Intent| Some((intent, false));

    // Sleep at night, once it has been awake a while.
    let sleepy = life
        .sleep
        .as_ref()
        .zip(world.awake_for(me))
        .is_some_and(|(sleep, awake)| awake >= sleep.awake / 16);
    if world.is_night() && sleepy {
        return plain(Intent::Sleep {
            seconds: None,
            shelter: None,
        });
    }

    // Eat what it holds, or drop it if it can't.
    if let Some(held) = world.contents(me).into_iter().next() {
        let item = world.key(held).to_string();
        return plain(if digestible(world, life, held) {
            Intent::Eat { item }
        } else {
            Intent::Drop { item }
        });
    }

    // Drink when thirsty, going to something drinkable if there's none here.
    let fluid = world
        .composition(me)
        .and_then(|c| c.get(&life.fluid))
        .map_or(0, |m| m.mg());
    if fluid + 1_000_000 < life.fluid_normal.mg() {
        let drink_at = |p: EntityId| {
            world
                .contents(p)
                .into_iter()
                .find(|&s| drinkable(world, life, s))
        };
        if let Some(source) = drink_at(here) {
            return plain(Intent::Drink {
                source: world.key(source).to_string(),
            });
        }
        return toward(here, &ways, |p| drink_at(p).is_some()).and_then(|p| plain(go(p)));
    }

    // Hungry, it goes first for food lying where it can get at it: on the
    // ground, or in something it can reach into. Food to be had without
    // searching is worth going out of its way for.
    if hungry(world, me, life) {
        let loose = |p: EntityId| -> Option<Intent> {
            for thing in world.contents(p) {
                if world.is_container(thing) && !laws::out_of_reach(world, me, thing) {
                    if let Some(inside) = world
                        .held(thing)
                        .into_iter()
                        .find(|&i| edible_loose(world, life, i))
                    {
                        return Some(Intent::TakeFrom {
                            item: world.key(inside).to_string(),
                            from: world.key(thing).to_string(),
                        });
                    }
                } else if edible_loose(world, life, thing) {
                    return Some(Intent::Take {
                        item: world.key(thing).to_string(),
                    });
                }
            }
            None
        };
        if let Some(take) = loose(here) {
            return plain(take);
        }
        if let Some(p) = toward(here, &ways, |p| loose(p).is_some()) {
            return plain(go(p));
        }
    }

    // Forage when hungry, going to food if there's none here.
    if hungry(world, me, life) {
        let food = |p: EntityId| {
            world.contents(p).into_iter().find(|&s| {
                // Only where a search has a fair chance: a picked-over patch
                // isn't worth staying for.
                world.pieces(s).is_some_and(|pieces| pieces.needs.is_none())
                    && digestible(world, life, s)
                    && laws::finding_chance(world, s) >= WORTH_SEARCHING
            })
        };
        if let Some(source) = food(here) {
            return plain(Intent::Gather {
                source: world.key(source).to_string(),
            });
        }
        return toward(here, &ways, |p| food(p).is_some()).and_then(|p| plain(go(p)));
    }

    // Otherwise, sometimes wander.
    if roll(2) < instinct.wander {
        return any_way(3).and_then(plain);
    }
    None
}

/// Whether a body could take in something.
fn digestible(world: &World, life: &Life, id: EntityId) -> bool {
    world
        .composition(id)
        .is_some_and(|c| c.keys().any(|m| life.digests.contains(m)))
}

/// Food lying loose: something that can be picked up, solid, and all of it
/// food for this body.
fn edible_loose(world: &World, life: &Life, id: EntityId) -> bool {
    world.is_portable(id)
        && !world.is_agent(id)
        && world.assembly(id).is_none()
        && world.is_all(id, State::Solid)
        && world
            .composition(id)
            .is_some_and(|c| c.keys().all(|m| life.digests.contains(m)))
}

/// Whether something is nothing but the fluid a body needs, and liquid.
fn drinkable(world: &World, life: &Life, id: EntityId) -> bool {
    world
        .composition(id)
        .is_some_and(|c| c.len() == 1 && c.contains_key(&life.fluid))
        && world.is_all(id, State::Liquid)
}

/// Hungry: its reserve is down, and it holds less than a day's food at rest
/// besides. An instinct keeps about a day's food in it, to see it through
/// the night.
fn hungry(world: &World, me: EntityId, life: &Life) -> bool {
    let (Some((reserve, start)), Some(body)) = (life.reserve, world.composition(me)) else {
        return false;
    };
    let left = body.get(&reserve).map_or(0, |m| m.mg());
    let food: u128 = body
        .iter()
        .filter(|(m, _)| **m != reserve && life.digests.contains(m))
        .map(|(m, mass)| u128::from(mass.mg()) * u128::from(world.materials()[m].energy_density))
        .sum();
    let a_day = u128::from(life.resting_power) * 86_400;
    left < start.mg() && food < a_day
}

/// The first step on the shortest way to a place that satisfies `wanted`.
fn toward(
    from: EntityId,
    ways: &dyn Fn(EntityId) -> Vec<EntityId>,
    wanted: impl Fn(EntityId) -> bool,
) -> Option<EntityId> {
    let mut seen = BTreeSet::from([from]);
    let mut queue: VecDeque<(EntityId, EntityId)> =
        ways(from).into_iter().map(|first| (first, first)).collect();
    while let Some((place, first)) = queue.pop_front() {
        if !seen.insert(place) {
            continue;
        }
        if wanted(place) {
            return Some(first);
        }
        for next in ways(place) {
            queue.push_back((next, first));
        }
    }
    None
}
