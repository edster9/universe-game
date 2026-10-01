//! Doing something more than once from one command, as a player would ask:
//! "gather sticks x3", "gather wood 500 g" (until they carry that much
//! more), "drop wood x2", "drop all", and several commands in a row,
//! "go to sticks; gather sticks x3". It's the player's own repeating, not a
//! law: each time is a command like any other, checked by the laws, and the
//! first refusal stops the rest. The seed of saved skills.

use std::collections::VecDeque;

use engine::intent::{self, Command, Intent};
use engine::laws::{self, ActError};
use engine::units::{self, Mass};
use engine::world::Outcome;

use super::{Reply, Session, sentence};

/// The most times one step is done, so a search that never finds stops.
const MOST: u32 = 100;

pub(super) struct Queue {
    steps: VecDeque<Step>,
    /// How each time came out, in order.
    told: Vec<String>,
    /// When it began, and what the player carried then, in mg.
    began: u64,
    carried: u64,
}

struct Step {
    /// The command, or `None` for "drop all".
    line: Option<String>,
    until: Until,
    done: u32,
    /// What the player carried when this step began, in mg.
    carried: Option<u64>,
}

enum Until {
    Times(u32),
    /// Until what they carry has changed by this much, either way: more
    /// when gathering, less when dropping.
    Moved(u64),
    /// Until there's nothing left to drop.
    Empty,
}

/// How one time went.
enum Once {
    Done(String),
    /// Under way, in real time: its outcome comes later.
    Started,
    Refused(String),
}

/// The steps in `line`, if it asks for more than one command: `None` for a
/// plain command.
fn plan(line: &str) -> Option<VecDeque<Step>> {
    let parts: Vec<&str> = line
        .split(';')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect();
    let steps: VecDeque<Step> = parts.iter().map(|p| step(p)).collect();
    let plain = steps.len() == 1 && matches!(steps[0].until, Until::Times(1));
    (!plain && !steps.is_empty()).then_some(steps)
}

/// One command, with how many times: "x3" or an amount at the end, or
/// "drop all".
fn step(part: &str) -> Step {
    let step = |line: Option<&str>, until| Step {
        line: line.map(str::to_string),
        until,
        done: 0,
        carried: None,
    };
    if part.eq_ignore_ascii_case("drop all") {
        return step(None, Until::Empty);
    }
    let words: Vec<&str> = part.split_whitespace().collect();
    let Some((last, before)) = words.split_last() else {
        return step(Some(part), Until::Times(1));
    };
    let times = last
        .strip_prefix(['x', 'X', '×'])
        .and_then(|n| n.parse::<u32>().ok())
        .filter(|&n| n > 0);
    if let Some(n) = times.filter(|_| !before.is_empty()) {
        return step(Some(&before.join(" ")), Until::Times(n.min(MOST)));
    }
    // An amount: "500g", or "500 g".
    let amount = |text: &str| {
        text.starts_with(|c: char| c.is_ascii_digit())
            .then(|| text.parse::<Mass>().ok())
            .flatten()
            .filter(|m| m.mg() > 0)
    };
    if let Some(mass) = amount(last).filter(|_| !before.is_empty()) {
        return step(Some(&before.join(" ")), Until::Moved(mass.mg()));
    }
    if let Some((number, rest)) = before.split_last().filter(|(_, rest)| !rest.is_empty())
        && let Some(mass) = amount(&format!("{number} {last}"))
    {
        return step(Some(&rest.join(" ")), Until::Moved(mass.mg()));
    }
    step(Some(part), Until::Times(1))
}

impl Session {
    /// Runs `line` as a queue, if it asks for more than one command.
    pub(super) fn queued(&mut self, line: &str) -> Option<Reply> {
        let steps = plan(line)?;
        let carried = self.carried();
        self.queue = Some(Queue {
            steps,
            told: Vec::new(),
            began: self.world.tick(),
            carried,
        });
        Some(match self.go_on() {
            Some(reply) => reply,
            None => Reply::say(self.starting()),
        })
    }

    /// Whether the player has more to do in a queue.
    pub fn queue_busy(&self) -> bool {
        self.queue.is_some()
    }

    /// In real time, when the player's action is over: how it came out,
    /// and on to the next. `None` if there's no queue.
    pub(super) fn queue_catch_up(&mut self) -> Option<String> {
        self.queue.as_ref()?;
        let Some(outcome) = self.world.take_outcome(self.player) else {
            // Nothing under way and nothing to hear of: go on.
            return self.go_on().map(|r| r.text).or(Some(String::new()));
        };
        let intent = self.started.remove(&self.player);
        let told = match (outcome, intent) {
            (Outcome::Done(changes), Some(intent)) => {
                let mut text = self.describe(&intent, &changes);
                for news in self.memory_news(&changes) {
                    text = format!("{text}\n{news}");
                }
                text
            }
            (Outcome::Done(_), None) => "You finish.".into(),
            (Outcome::Failed(why), _) => {
                return Some(self.finish(Some(format!("You couldn't finish: {}", sentence(&why)))));
            }
            (Outcome::Interrupted, _) => {
                return Some(self.finish(Some("You were cut short, and didn't finish.".into())));
            }
        };
        if let Some(queue) = &mut self.queue {
            queue.told.push(told);
        }
        Some(self.go_on().map(|r| r.text).unwrap_or_default())
    }

    /// Stops the queue, if there is one: what was done so far.
    pub(super) fn stop_queue(&mut self) -> Option<String> {
        self.queue.as_ref()?;
        Some(self.finish(None))
    }

    /// Does what's next in the queue, until it's waiting on an action under
    /// way (`None`) or done (the whole story).
    fn go_on(&mut self) -> Option<Reply> {
        loop {
            let carried = self.carried();
            let queue = self.queue.as_mut()?;
            let Some(step) = queue.steps.front_mut() else {
                return Some(Reply::say(self.finish(None)));
            };
            let from = *step.carried.get_or_insert(carried);
            let over = step.done >= MOST
                || match step.until {
                    Until::Times(n) => step.done >= n,
                    Until::Moved(mass) => carried.abs_diff(from) >= mass,
                    Until::Empty => false,
                };
            let line = match &step.line {
                _ if over => None,
                Some(line) => Some(line.clone()),
                None => self
                    .droppable()
                    .map(|id| format!("drop {}", laws::pointer(id))),
            };
            let Some(line) = line else {
                if let Some(queue) = &mut self.queue {
                    queue.steps.pop_front();
                }
                continue;
            };
            if let Some(step) = self.queue.as_mut().and_then(|q| q.steps.front_mut()) {
                step.done += 1;
            }
            match self.once(&line) {
                Once::Done(text) => {
                    if let Some(queue) = &mut self.queue {
                        queue.told.push(text);
                    }
                }
                Once::Started => return None,
                Once::Refused(why) => {
                    let nothing_done = self.queue.as_ref().is_some_and(|q| q.told.is_empty());
                    let text = self.finish(Some(why));
                    return Some(if nothing_done {
                        Reply::refuse(text)
                    } else {
                        Reply::say(text)
                    });
                }
            }
        }
    }

    /// Does `line` once.
    fn once(&mut self, line: &str) -> Once {
        let intent = match intent::parse(line) {
            Ok(Command::Act(intent)) => intent,
            // Anything else is answered at once.
            _ => {
                let reply = self.handle(line);
                return if reply.refused {
                    Once::Refused(reply.text)
                } else {
                    Once::Done(reply.text)
                };
            }
        };
        let result = if self.real_time {
            match laws::start(&mut self.world, self.player, intent.clone()) {
                Ok(laws::Started::Due(_)) => {
                    self.started.insert(self.player, intent);
                    return Once::Started;
                }
                Ok(laws::Started::Now { changes, .. }) => Ok(changes),
                Err(e) => Err(e),
            }
        } else {
            let began = self.world.tick();
            let result = laws::perform(&mut self.world, self.player, intent.clone());
            self.spent = self.world.tick() - began;
            result
        };
        match result {
            Ok(changes) => {
                let mut text = self.describe(&intent, &changes);
                for news in self.memory_news(&changes) {
                    text = format!("{text}\n{news}");
                }
                Once::Done(text)
            }
            Err(ActError::Refused(refusal)) => Once::Refused(self.refusal(&refusal)),
            Err(fault @ ActError::Fault(_)) => Once::Refused(format!("!! {fault}")),
        }
    }

    /// Ends the queue: how each time came out, alike ones together, what
    /// they carry now against before, how long it took, and why it stopped
    /// early, if it did.
    fn finish(&mut self, stopped: Option<String>) -> String {
        let Some(queue) = self.queue.take() else {
            return stopped.unwrap_or_default();
        };
        let mut lines: Vec<(String, u32)> = Vec::new();
        for text in queue.told {
            match lines.last_mut() {
                Some((last, n)) if *last == text => *n += 1,
                _ => lines.push((text, 1)),
            }
        }
        let mut text: Vec<String> = lines
            .into_iter()
            .map(|(t, n)| if n > 1 { format!("{t} (x{n})") } else { t })
            .collect();
        let carried = self.carried();
        let took = self.world.tick() - queue.began;
        let mut summary = match carried.cmp(&queue.carried) {
            std::cmp::Ordering::Greater => format!(
                "You carry {} more than before.",
                Mass::from_mg(carried - queue.carried)
            ),
            std::cmp::Ordering::Less => format!(
                "You carry {} less than before.",
                Mass::from_mg(queue.carried - carried)
            ),
            std::cmp::Ordering::Equal => String::new(),
        };
        if took > 0 && !text.is_empty() {
            summary = format!("{summary} (That took {}.)", units::show_duration(took));
        }
        text.push(summary.trim().to_string());
        if let Some(why) = stopped {
            text.push(why);
        }
        text.retain(|t| !t.is_empty());
        text.join("\n")
    }

    /// What the player will hear on starting a queue in real time.
    fn starting(&self) -> String {
        let Some(queue) = &self.queue else {
            return String::new();
        };
        let doing = self
            .started
            .get(&self.player)
            .map_or_else(|| "keep going".to_string(), Intent::to_string);
        let mut text = format!("You start to {doing}");
        if let Some(step) = queue.steps.front() {
            match step.until {
                Until::Times(n) if n > 1 => text += &format!(", {n} times"),
                Until::Moved(mass) => text += &format!(", for {}", Mass::from_mg(mass)),
                _ => {}
            }
        }
        let then: Vec<String> = queue.steps.iter().skip(1).map(Step::said).collect();
        if !then.is_empty() {
            text += &format!(", then {}", then.join(", then "));
        }
        format!("{text}.")
    }

    /// What the player carries, in mg.
    fn carried(&self) -> u64 {
        self.world.carried_mass(self.player).mg()
    }

    /// The first thing the player holds and could drop: not worn.
    fn droppable(&self) -> Option<engine::world::EntityId> {
        self.world
            .held(self.player)
            .into_iter()
            .find(|&id| self.world.worn(id).is_none())
    }
}

impl Step {
    /// The step as the player asked for it.
    fn said(&self) -> String {
        let line = self.line.as_deref().unwrap_or("drop all");
        match self.until {
            Until::Times(n) if n > 1 => format!("{line} x{n}"),
            Until::Moved(mass) => format!("{line} {}", Mass::from_mg(mass)),
            _ => line.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(line: &str) -> Vec<String> {
        plan(line)
            .map(|steps| steps.iter().map(Step::said).collect())
            .unwrap_or_default()
    }

    #[test]
    fn times_amounts_and_rows() {
        assert!(read("gather sticks").is_empty());
        assert!(read("walk to 5 20").is_empty());
        assert_eq!(read("gather sticks x3"), ["gather sticks x3"]);
        assert_eq!(read("gather wood 500g"), ["gather wood 500 g"]);
        assert_eq!(read("gather wood 1.5 kg"), ["gather wood 1.5 kg"]);
        assert_eq!(read("drop all"), ["drop all"]);
        assert_eq!(
            read("go to sticks; gather sticks x2"),
            ["go to sticks", "gather sticks x2"]
        );
        // Nothing to repeat.
        assert!(read("x3").is_empty());
    }
}
