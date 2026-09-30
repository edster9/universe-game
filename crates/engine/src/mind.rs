//! Minds: how people nobody plays decide what to do. See
//! docs/ideas/npc-minds.md and docs/challenges/a-companion.md.
//!
//! A mind has a scope, how far it thinks, and standing orders: lines in data
//! saying when to do what, in the person's own words. Underneath every scope
//! is the body's instinct. A mind knows only what its person perceives and
//! remembers, and only ever proposes commands, which the laws check like
//! anyone's.

use std::collections::{BTreeSet, VecDeque};

use crate::gate::{Cause, Change, Fault};
use crate::intent::{Command, Intent};
use crate::laws;
use crate::world::{EntityId, Life, World};

/// How long someone with nothing to do waits before thinking again.
const IDLE: u64 = 600;

/// How long ago someone went for them and still counts as just now, for
/// striking back.
const JUST_NOW: u64 = 60;

/// How long someone who went for them stays a danger while they're around.
const DANGER: u64 = 3_600;

/// How far a mind thinks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// Its standing orders, nothing else: when none applies, it waits.
    Confined,
    /// Its orders, and when none applies, it looks after itself.
    Resident,
}

/// How a mind meets someone who goes for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Temperament {
    /// Strikes at them for as long as they're there.
    Fight,
    /// Strikes back when struck, and otherwise carries on.
    Defend,
    /// Gets away from them.
    Flee,
    /// Neither fights nor runs.
    GiveIn,
}

/// A mind: its scope, its temperament, and its standing orders, first
/// first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mind {
    pub scope: Scope,
    pub temperament: Temperament,
    pub orders: Vec<Order>,
    /// What they've been asked to do and have taken on, first first.
    pub requests: Vec<Intent>,
    /// What things are worth to them, a kilo, by their word for each: the
    /// highest that fits a thing counts.
    pub values: Vec<(String, u64)>,
}

/// "When these hold, do this."
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order {
    pub when: Vec<Condition>,
    pub command: Intent,
    /// The order as written, to show.
    pub text: String,
}

/// Something a person can tell for themselves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Condition {
    Night,
    Day,
    Hungry,
    Thirsty,
    /// Tired enough to sleep.
    Tired,
    /// Where they are, by their word for it.
    At(String),
    /// Something they carry, by their word for it.
    Carrying(String),
    /// Something where they are, by their word for it.
    Sees(String),
    /// At or after this time of day, in seconds.
    After(u64),
    /// Before this time of day, in seconds.
    Before(u64),
    Not(Box<Condition>),
}

impl Scope {
    pub fn parse(text: &str) -> Result<Scope, String> {
        match text {
            "confined" => Ok(Scope::Confined),
            "resident" => Ok(Scope::Resident),
            other => Err(format!(
                "the mind {other:?} isn't one there is: try \"confined\" or \"resident\""
            )),
        }
    }
}

impl Temperament {
    pub fn parse(text: &str) -> Result<Temperament, String> {
        match text {
            "fight" => Ok(Temperament::Fight),
            "defend" => Ok(Temperament::Defend),
            "flee" => Ok(Temperament::Flee),
            "give in" => Ok(Temperament::GiveIn),
            other => Err(format!(
                "the temperament {other:?} isn't one there is: try \"fight\", \"defend\", \"flee\", or \"give in\""
            )),
        }
    }
}

impl Order {
    /// Reads an order: conditions, a colon, and a command, as in
    /// "thirsty, at the spring: drink from the spring".
    pub fn parse(text: &str) -> Result<Order, String> {
        let (when, command) = text
            .split_once(':')
            .ok_or_else(|| format!("the order {text:?} needs a colon before its command"))?;
        let when = when
            .split(',')
            .map(str::trim)
            .filter(|c| !c.is_empty())
            .map(Condition::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let command = match crate::intent::parse(command.trim()) {
            Ok(Command::Act(intent)) => intent,
            _ => return Err(format!("the order {text:?} doesn't end with a command")),
        };
        Ok(Order {
            when,
            command,
            text: text.to_string(),
        })
    }
}

impl Condition {
    fn parse(text: &str) -> Result<Condition, String> {
        if let Some(rest) = text.strip_prefix("not ") {
            return Ok(Condition::Not(Box::new(Condition::parse(rest.trim())?)));
        }
        let clock = |t: &str| -> Result<u64, String> {
            let (h, m) = t
                .trim()
                .split_once(':')
                .ok_or_else(|| format!("{t:?} isn't a time like 08:30"))?;
            let h: u64 = h.parse().map_err(|_| format!("{t:?} isn't a time"))?;
            let m: u64 = m.parse().map_err(|_| format!("{t:?} isn't a time"))?;
            Ok(h * 3_600 + m * 60)
        };
        Ok(match text {
            "night" => Condition::Night,
            "day" => Condition::Day,
            "hungry" => Condition::Hungry,
            "thirsty" => Condition::Thirsty,
            "tired" => Condition::Tired,
            _ => {
                if let Some(rest) = text.strip_prefix("at ") {
                    Condition::At(rest.trim().to_string())
                } else if let Some(rest) = text.strip_prefix("carrying ") {
                    Condition::Carrying(rest.trim().to_string())
                } else if let Some(rest) = text.strip_prefix("sees ") {
                    Condition::Sees(rest.trim().to_string())
                } else if let Some(rest) = text.strip_prefix("after ") {
                    Condition::After(clock(rest)?)
                } else if let Some(rest) = text.strip_prefix("before ") {
                    Condition::Before(clock(rest)?)
                } else {
                    return Err(format!("{text:?} isn't something a person can tell"));
                }
            }
        })
    }

    fn holds(&self, world: &World, me: EntityId, life: &Life) -> bool {
        let here = world.place_of(me);
        match self {
            Condition::Night => world.is_night(),
            Condition::Day => !world.is_night(),
            Condition::Hungry => crate::instinct::hungry(world, me, life),
            Condition::Thirsty => thirsty(world, me, life),
            Condition::Tired => tired(world, me, life),
            Condition::At(name) => here.is_some_and(|p| laws::calls(world, me, p, name)),
            Condition::Carrying(name) => laws::finds(world, me, world.contents(me), name).is_some(),
            Condition::Sees(name) => here.is_some_and(|p| {
                let around = world.contents(p).into_iter().filter(|&e| e != me);
                laws::finds(world, me, around, name).is_some()
            }),
            Condition::After(t) => world.time_of_day().is_some_and(|now| now >= *t),
            Condition::Before(t) => world.time_of_day().is_some_and(|now| now < *t),
            Condition::Not(inner) => !inner.holds(world, me, life),
        }
    }
}

/// Thirsty: a kilo or more short of the body's normal fluid.
pub(crate) fn thirsty(world: &World, me: EntityId, life: &Life) -> bool {
    let fluid = world
        .composition(me)
        .and_then(|c| c.get(&life.fluid))
        .map_or(0, |m| m.mg());
    fluid + 1_000_000 < life.fluid_normal.mg()
}

/// Tired enough to sleep: awake a sixteenth of its waking day or more.
pub(crate) fn tired(world: &World, me: EntityId, life: &Life) -> bool {
    life.sleep
        .as_ref()
        .zip(world.awake_for(me))
        .is_some_and(|(sleep, awake)| awake >= sleep.awake / 16)
}

/// Lets everyone with a mind who's free start what they do next: meeting
/// danger by their temperament; what they've been asked; the first standing
/// order that holds and that the laws allow; otherwise what the body wants;
/// otherwise, if their scope lets them, looking after themselves. With
/// nothing to do, they wait a while.
pub fn act(world: &mut World) -> Result<(), Fault> {
    let free: Vec<EntityId> = world
        .minds
        .keys()
        .copied()
        .filter(|&id| world.is_living(id) && !world.is_asleep(id) && !world.is_busy(id))
        .collect();
    for me in free {
        let now = world.tick();
        let mut choices = choices(world, me);
        // Unless they're in danger, what they were asked comes before their
        // own orders. They take it up once, doing it or finding they can't.
        let in_danger = world
            .mind(me)
            .is_some_and(|m| !meet(world, me, m.temperament).is_empty());
        if let Some(request) = world.mind(me).and_then(|m| m.requests.first().cloned())
            && !in_danger
        {
            world.apply(
                Cause::Nature { tick: now },
                vec![Change::TakeUp { agent: me }],
            )?;
            choices.insert(0, request);
        }
        let mut started = false;
        for intent in choices {
            match laws::start(world, me, intent) {
                Ok(laws::Started::Due(_)) => {}
                Ok(laws::Started::Now { seconds, .. }) => {
                    if seconds == 0 {
                        // Something quick: a moment's pause before the next.
                        world.apply(
                            Cause::Nature { tick: now },
                            vec![Change::Occupy {
                                agent: me,
                                until: now + 1,
                            }],
                        )?;
                    }
                }
                Err(laws::ActError::Refused(_)) => continue,
                Err(laws::ActError::Fault(fault)) => return Err(fault),
            }
            started = true;
            break;
        }
        if !started {
            world.apply(
                Cause::Nature { tick: now },
                vec![Change::Occupy {
                    agent: me,
                    until: now + IDLE,
                }],
            )?;
        }
    }
    Ok(())
}

/// What someone would do now, in order of preference.
fn choices(world: &World, me: EntityId) -> Vec<Intent> {
    let (Some(mind), Some(life)) = (world.mind(me), world.life(me)) else {
        return Vec::new();
    };
    let mut choices = meet(world, me, mind.temperament);
    choices.extend(
        mind.orders
            .iter()
            .filter(|order| order.when.iter().all(|c| c.holds(world, me, life)))
            .map(|order| order.command.clone()),
    );
    choices.extend(instinct(world, me, life));
    if mind.scope == Scope::Resident {
        choices.extend(look_after(world, me, life));
    }
    choices
}

/// Meeting someone here who went for them, by temperament: strike at them,
/// strike back, or get away. The nearest danger comes first.
fn meet(world: &World, me: EntityId, temperament: Temperament) -> Vec<Intent> {
    let (Some(here), Some(memory)) = (world.place_of(me), world.memory(me)) else {
        return Vec::new();
    };
    let now = world.tick();
    let danger = |within: u64| -> Option<EntityId> {
        memory
            .attackers
            .iter()
            .filter(|&(&who, &when)| {
                now.saturating_sub(when) <= within
                    && world.is_living(who)
                    && world.place_of(who) == Some(here)
            })
            .max_by_key(|&(&who, &when)| (when, who))
            .map(|(&who, _)| who)
    };
    let strike = |at: EntityId| -> Vec<Intent> {
        let weapon = laws::best_edge(world, &world.contents(me));
        vec![Intent::Attack {
            target: laws::pointer(at),
            with: weapon.map(laws::pointer),
        }]
    };
    // A blow not yet answered: they went for me since I last went for them.
    let unanswered = |who: EntityId| {
        let went_for_me = memory.attackers.get(&who).copied().unwrap_or(0);
        memory
            .struck
            .get(&who)
            .is_none_or(|&mine| mine < went_for_me)
    };
    match temperament {
        Temperament::Fight => danger(DANGER).map(strike).unwrap_or_default(),
        Temperament::Defend => danger(JUST_NOW)
            .filter(|&who| unanswered(who))
            .map(strike)
            .unwrap_or_default(),
        Temperament::Flee => match danger(DANGER) {
            Some(threat) => world
                .exits(here)
                .iter()
                .copied()
                .filter(|&to| world.crossing(here, to).is_none())
                .filter(|&to| world.place_of(threat) != Some(to))
                .map(go)
                .collect(),
            None => Vec::new(),
        },
        Temperament::GiveIn => Vec::new(),
    }
}

/// What the body wants, whatever the mind's scope: sleep at night when
/// tired, eat what's carried when hungry, drink what's here when thirsty.
fn instinct(world: &World, me: EntityId, life: &Life) -> Vec<Intent> {
    let mut wants = Vec::new();
    if world.is_night() && tired(world, me, life) {
        wants.push(Intent::Sleep {
            seconds: None,
            shelter: None,
        });
    }
    if crate::instinct::hungry(world, me, life)
        && let Some(food) = world
            .contents(me)
            .into_iter()
            .find(|&f| crate::instinct::digestible(world, life, f))
    {
        wants.push(Intent::Eat {
            item: laws::pointer(food),
        });
    }
    if thirsty(world, me, life)
        && let Some(here) = world.place_of(me)
        && let Some(source) = world
            .contents(here)
            .into_iter()
            .find(|&s| crate::instinct::drinkable(world, life, s))
    {
        wants.push(Intent::Drink {
            source: laws::pointer(source),
        });
    }
    wants
}

/// Looking after themselves, from what they remember: going for something
/// to drink, or
/// for food to gather, at the nearest place they remember seeing it, and
/// gathering it there.
fn look_after(world: &World, me: EntityId, life: &Life) -> Vec<Intent> {
    let Some(here) = world.place_of(me) else {
        return Vec::new();
    };
    let remembered = |place: EntityId| -> Vec<EntityId> {
        let mut seen: Vec<EntityId> = world
            .memory(me)
            .and_then(|m| m.sightings.get(&place))
            .map(|(_, things)| things.iter().copied().collect())
            .unwrap_or_default();
        if place == here {
            seen = world.contents(here);
        }
        seen
    };
    let drink = |p: EntityId| {
        remembered(p)
            .into_iter()
            .any(|s| world.exists(s) && crate::instinct::drinkable(world, life, s))
    };
    let food = |p: EntityId| {
        remembered(p).into_iter().find(|&s| {
            world.exists(s)
                && world.pieces(s).is_some_and(|pieces| pieces.needs.is_none())
                && crate::instinct::digestible(world, life, s)
        })
    };
    let mut wants = Vec::new();
    if thirsty(world, me, life)
        && let Some(to) = toward(world, me, here, drink)
    {
        wants.push(go(to));
    }
    if crate::instinct::hungry(world, me, life) {
        if let Some(source) = food(here) {
            wants.push(Intent::Gather {
                source: laws::pointer(source),
            });
        } else if let Some(to) = toward(world, me, here, |p| food(p).is_some()) {
            wants.push(go(to));
        }
    }
    wants
}

fn go(to: EntityId) -> Intent {
    Intent::Go {
        place: laws::pointer(to),
        aboard: None,
    }
}

/// The first step on the shortest way, by ways they know and on foot, to a
/// place that satisfies `wanted`.
fn toward(
    world: &World,
    me: EntityId,
    from: EntityId,
    wanted: impl Fn(EntityId) -> bool,
) -> Option<EntityId> {
    let known = |a: EntityId, b: EntityId| {
        world
            .memory(me)
            .is_none_or(|m| !m.finds_ways || m.ways.contains(&(a, b)) || m.ways.contains(&(b, a)))
    };
    let ways = |p: EntityId| -> Vec<EntityId> {
        world
            .exits(p)
            .iter()
            .copied()
            .filter(|&to| known(p, to) && world.crossing(p, to).is_none())
            .collect()
    };
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
